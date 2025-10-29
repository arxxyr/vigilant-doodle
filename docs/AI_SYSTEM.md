# AI 系统架构文档

本文档描述 Vigilant Doodle 游戏的敌人 AI 系统架构。

## 概述

AI 系统采用模块化设计，将敌人行为逻辑分离为独立的模块，便于扩展和维护。

## 目录结构

```
crates/game/src/
├── ai/                      # AI 系统模块（新增）
│   ├── mod.rs              # 模块入口和插件定义
│   ├── detection.rs        # 寻敌检测系统
│   ├── enemy_ai.rs         # 状态机和 AI 核心逻辑
│   └── behavior.rs         # 行为执行系统
├── gameplay/
│   ├── enemy.rs            # 敌人基础组件（简化后）
│   ├── player.rs
│   └── movement.rs
└── ...
```

## 架构设计

### 三阶段流水线

AI 系统采用三阶段流水线架构：

```
┌─────────────────────────────────────────────────────────┐
│                      AI System                          │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │  Detection   │─→│   Decision   │─→│  Execution   │ │
│  │   检测玩家    │  │   状态转换    │  │   执行行为    │ │
│  └──────────────┘  └──────────────┘  └──────────────┘ │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

#### 1. Detection 阶段（检测）

- **职责**：检测玩家位置，判断是否进入/离开追踪范围
- **文件**：`ai/detection.rs`
- **核心组件**：`DetectionConfig`
- **关键参数**：
  - `detection_range`: 35.0 - 初始检测范围
  - `lose_target_range`: 50.0 - 失去目标范围（更大，避免频繁切换）

#### 2. Decision 阶段（决策）

- **职责**：根据检测结果更新 AI 状态
- **文件**：`ai/enemy_ai.rs`
- **核心组件**：`EnemyAIState`
- **状态转换图**：

```
    ┌──────┐
    │ Idle │ ◄────────────┐
    └───┬──┘              │
        │                 │
        │ PlayerDetected  │ PlayerLost
        │                 │
        ▼                 │
    ┌─────────┐           │
    │ Chasing │───────────┘
    └─────────┘
```

#### 3. Execution 阶段（执行）

- **职责**：根据当前状态执行具体行为
- **文件**：`ai/behavior.rs`
- **核心系统**：
  - `chase_behavior_system` - 追击行为
  - `idle_behavior_system` - 闲置行为

## 核心组件

### 1. EnemyAIState（状态机）

定义敌人的 AI 状态：

```rust
pub enum EnemyAIState {
    Idle,       // 闲置：等待玩家接近
    Chasing,    // 追击：持续追踪玩家
    // 未来扩展：
    // Patrol,   // 巡逻
    // Attacking, // 攻击
    // Searching, // 搜索
}
```

### 2. DetectionConfig（检测配置）

控制敌人的检测行为：

```rust
pub struct DetectionConfig {
    pub detection_range: f32,      // 35.0 - 初始检测范围
    pub lose_target_range: f32,    // 50.0 - 失去目标范围
    pub field_of_view: f32,        // 360.0 - 视野角度（未来扩展）
}
```

### 3. EnemyAIConfig（行为配置）

控制敌人的移动行为：

```rust
pub struct EnemyAIConfig {
    pub move_speed: f32,        // 7.0 - 移动速度
    pub stop_distance: f32,     // 1.5 - 停止距离
    pub rotation_speed: f32,    // 10.0 - 旋转速度（未来扩展）
}
```

### 4. EnemyTarget（目标信息）

存储敌人追踪的目标：

```rust
pub struct EnemyTarget {
    pub entity: Option<Entity>,              // 目标实体
    pub last_known_position: Option<Vec3>,   // 最后已知位置
    pub lost_time: Option<f32>,              // 失去目标时间（未来扩展）
}
```

## 关键特性

### 1. "一旦发现就持续追踪"机制

通过不对称的检测范围实现：

- **检测范围**（detection_range）：35.0 单位
- **失去范围**（lose_target_range）：50.0 单位

**行为逻辑**：
1. 玩家进入 35.0 范围 → 敌人发现玩家，进入 Chasing 状态
2. 玩家离开 35.0 但仍在 50.0 内 → 敌人继续追击
3. 玩家离开 50.0 范围 → 敌人失去目标，回到 Idle 状态

这避免了玩家在边界附近时敌人反复切换状态的问题。

### 2. 模块化设计

每个模块职责单一，易于扩展：

- **detection.rs** - 专注于检测逻辑，未来可添加视野角度、障碍物遮挡等
- **enemy_ai.rs** - 状态机核心，未来可添加更多状态（攻击、巡逻等）
- **behavior.rs** - 行为执行，未来可添加更复杂的行为（群体协作等）

### 3. 数据驱动

所有参数可配置：

```rust
// 生成敌人时配置
commands.spawn((
    Enemy,
    EnemyAIState::default(),
    EnemyAIConfig {
        move_speed: 10.0,      // 自定义速度
        stop_distance: 2.0,
        rotation_speed: 15.0,
    },
    DetectionConfig {
        detection_range: 40.0,  // 自定义检测范围
        lose_target_range: 60.0,
        field_of_view: 180.0,
    },
    // ...
));
```

## 系统执行顺序

AI 系统通过 SystemSet 确保正确的执行顺序：

```rust
app.configure_sets(
    Update,
    (
        BehaviorSystemSet::Detection,   // 1. 检测
        BehaviorSystemSet::Decision,    // 2. 决策
        BehaviorSystemSet::Execution,   // 3. 执行
    ).chain()
);
```

这确保了：
1. 先检测玩家位置
2. 再根据检测结果更新状态
3. 最后根据状态执行行为

## 未来扩展方向

### 1. 新增状态

```rust
pub enum EnemyAIState {
    Idle,
    Patrol,      // 巡逻：在区域内移动
    Chasing,
    Attacking,   // 攻击：在攻击范围内
    Searching,   // 搜索：朝向最后已知位置
    Retreating,  // 撤退：低血量时逃跑
}
```

### 2. 增强检测

- 视野角度限制（FOV）
- 障碍物遮挡检测
- 声音检测范围
- 不同敌人类型的检测能力差异

### 3. 群体行为

- 警报机制（一个敌人发现玩家，通知附近敌人）
- 包围战术
- 协同攻击

### 4. 行为树

对于更复杂的 AI，可以集成行为树系统：

```rust
pub enum AIBehavior {
    Sequence(Vec<AIBehavior>),
    Selector(Vec<AIBehavior>),
    Condition(Box<dyn Fn() -> bool>),
    Action(Box<dyn Fn()>),
}
```

## 调试工具

### 日志输出

AI 系统提供详细的日志输出：

```
[AI] 敌人发现玩家！
[AI] 状态转换: 闲置 -> 追击
[AI] 敌人失去玩家！
[AI] 状态转换: 追击 -> 闲置
```

### 可视化调试（未来）

可以添加调试绘制功能：

```rust
// 绘制检测范围
gizmos.circle(enemy_pos, detection_range, Color::GREEN);
gizmos.circle(enemy_pos, lose_target_range, Color::RED);

// 绘制视线
gizmos.line(enemy_pos, player_pos, Color::YELLOW);
```

## 性能考虑

### SystemSet 链式执行

通过 `.chain()` 确保系统按顺序执行，但在同一阶段内仍可并行：

```rust
.add_systems(
    Update,
    (
        chase_behavior_system,
        idle_behavior_system,
    ).in_set(BehaviorSystemSet::Execution),
);
```

### 查询优化

使用 `With`/`Without` 过滤器减少查询开销：

```rust
Query<(&Transform, &EnemyAIConfig), With<Enemy>>
```

### 事件驱动（未来优化）

当前每帧检测，可改为事件驱动：

```rust
pub enum AIEvent {
    PlayerEnterRange(Entity),
    PlayerLeaveRange(Entity),
}
```

## 与其他系统的集成

### 1. 存档系统

AI 配置会保存到存档：

```rust
pub struct EnemySaveData {
    // 基础信息
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    // AI 配置
    pub move_speed: f32,
    pub stop_distance: f32,
    pub detection_range: f32,
    pub lose_target_range: f32,
}
```

### 2. 碰撞系统

AI 移动结果会被碰撞系统处理：

```
AI 移动 → 碰撞检测 → 分离处理
```

### 3. 动画系统（未来）

状态转换可触发动画：

```rust
fn update_animation(
    query: Query<(&EnemyAIState, &mut AnimationPlayer), Changed<EnemyAIState>>,
) {
    for (state, mut animation) in query.iter_mut() {
        match state {
            EnemyAIState::Idle => animation.play("idle"),
            EnemyAIState::Chasing => animation.play("run"),
            // ...
        }
    }
}
```

## 测试

### 单元测试

每个模块都包含单元测试：

```bash
cargo test --package vigilant-doodle-game --lib ai
```

测试覆盖：
- 状态转换逻辑
- 检测范围计算
- 目标管理

### 集成测试

测试 AI 系统与其他系统的集成：

```rust
#[test]
fn test_enemy_chase_player() {
    let mut app = App::new();
    app.add_plugins(EnemyAIPlugin);
    // ...
}
```

## 总结

新的 AI 系统具有以下优势：

1. **模块化**：职责清晰，易于理解和维护
2. **可扩展**：新增状态和行为无需修改核心代码
3. **数据驱动**：参数可配置，支持不同类型的敌人
4. **性能优化**：使用 SystemSet 和查询过滤器
5. **易于调试**：详细的日志输出和状态跟踪

未来可以基于此架构扩展更复杂的 AI 行为，如巡逻、群体协作、行为树等。

---

最后更新：2025-10-25
版本：v1.0
