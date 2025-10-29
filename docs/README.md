# Vigilant Doodle - 技术文档索引

## 文档说明

本目录包含项目的完整技术文档，遵循 Bevy 0.17.2 最佳实践。

---

## 主文档

### [WORKSPACE_ARCHITECTURE.md](./WORKSPACE_ARCHITECTURE.md) 🔥 **必读**
**Cargo Workspace + 动态库架构设计**

包含内容：
- ✅ Workspace 目录结构
- ✅ 启动器（launcher）+ 游戏库（game）分离
- ✅ 完整的 Cargo.toml 配置
- ✅ 动态链接加速编译
- ✅ 模块化扩展方案
- ✅ CI/CD 配置示例
- ✅ 迁移步骤详解

**阅读对象**：
- 所有开发者（必读）
- 项目架构师

**阅读建议**：
1. **先读这个文档**，了解项目组织结构
2. 理解 launcher 和 game 的职责分离
3. 按照"迁移步骤"创建 workspace

---

### [ARCHITECTURE_BEST_PRACTICE.md](./ARCHITECTURE_BEST_PRACTICE.md)
**游戏逻辑架构设计（game crate 内部）**

包含内容：
- ✅ 完整的模块结构设计
- ✅ 状态机流程图
- ✅ 单相机架构（斜向俯视）
- ✅ 实体生命周期管理
- ✅ UI 系统设计（模糊遮罩 + 菜单叠加）
- ✅ 输入管理与光标控制
- ✅ 游戏玩法系统（玩家、敌人、碰撞）
- ✅ 资源预加载策略
- ✅ 性能优化方案
- ✅ 完整的代码实现示例

**阅读对象**：
- 游戏逻辑开发者
- 代码审查者

**阅读建议**：
1. 在创建 workspace 之后阅读
2. 重点关注 `crates/game/src/` 内部的模块划分
3. 参考具体模块的代码示例实现

---

## 📚 文档阅读顺序

### 第一步：理解项目结构
👉 **[WORKSPACE_ARCHITECTURE.md](./WORKSPACE_ARCHITECTURE.md)**
- Workspace 组织方式
- launcher 和 game 的分离
- 构建与运行命令

### 第二步：了解游戏架构
👉 **[ARCHITECTURE_BEST_PRACTICE.md](./ARCHITECTURE_BEST_PRACTICE.md)**
- 游戏逻辑模块划分
- 状态机设计
- 完整代码示例

### 第三步：开始实现
👉 **本文档（README.md）**
- 快速参考表格
- 实现清单
- 开发规范

---

## 快速参考

### 核心设计决策

| 方面 | 设计方案 |
|------|----------|
| **相机** | 单个 Camera3d，斜向俯视 45°，平滑跟随 |
| **状态** | AssetLoading → MainMenu → Playing |
| **实体** | 预生成，始终可见，状态过滤系统 |
| **菜单** | 游戏场景 + 半透明遮罩 + UI 叠加 |
| **输入** | Playing 锁定光标，Menu 解锁 |
| **资源** | 统一预加载（bevy_asset_loader） |
| **本地化** | 中文/English 双语，动态切换 |

### 关键技术点

1. **单相机架构**：删除 Camera2d，只用 Camera3d
2. **状态驱动**：通过 `.run_if(in_state())` 控制系统执行
3. **实体可见**：删除 show/hide 系统，减少复杂度
4. **菜单叠加**：使用 ZIndex 层级渲染
5. **错误处理**：避免 panic，使用 warn + 返回
6. **本地化支持**：`LocalizedText` 组件自动更新，语言切换实时生效

### 目录结构

```
vigilant-doodle/
├── Cargo.toml                    # Workspace 配置
├── crates/
│   ├── launcher/                 # 启动器（可执行程序）
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── game/                     # 游戏逻辑（动态库）
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── core/             # 状态机、本地化
│           ├── assets/           # 资源加载
│           ├── camera/           # 斜向俯视相机
│           ├── gameplay/         # 玩家、敌人、移动
│           ├── world/            # 地形、生成
│           ├── ui/               # 菜单、HUD
│           ├── input/            # 输入、光标
│           └── audio/            # 音频
├── assets/                       # 游戏资源
│   ├── fonts/                    # 字体文件
│   ├── localization/             # 多语言翻译
│   └── models/                   # 3D 模型
└── docs/                         # 文档
```

---

## 实现清单

### 阶段 1：核心架构 ✅
- [x] `core/state.rs` - 状态定义
- [x] `core/localization.rs` - 本地化系统
- [x] `assets/loader.rs` - 资源预加载
- [x] `camera/isometric.rs` - 斜向俯视相机
- [x] `world/spawning.rs` - 实体生成
- [x] `world/terrain.rs` - 地形生成

### 阶段 2：游戏玩法 ✅
- [x] `gameplay/player.rs` - 玩家移动
- [x] `gameplay/enemy.rs` - 敌人 AI
- [x] `gameplay/movement.rs` - 碰撞检测
- [x] `input/actions.rs` - 输入映射
- [x] `input/cursor.rs` - 光标管理

### 阶段 3：UI 系统 ✅
- [x] `ui/menu.rs` - 主菜单 + 半透明背景
- [x] `ui/settings_menu.rs` - 设置菜单
- [x] `ui/components.rs` - UI 组件
- [x] `ui/styles.rs` - UI 样式
- [x] 语言切换按钮（🌐 中文/English）

### 阶段 4：完善功能 🚧
- [x] `audio/mod.rs` - 音频系统框架
- [ ] `ui/hud/` - 游戏内 HUD（可选）
- [x] 性能测试与优化
- [x] 代码审查与清理

---

## 开发规范

### 命名约定
- **组件/类型**：`UpperCamelCase`（例：`IsometricCamera`）
- **函数/变量**：`snake_case`（例：`spawn_camera`）
- **常量**：`UPPER_SNAKE_CASE`（例：`FLOOR_HALF_LENGTH`）

### 日志格式
```rust
info!("[模块名] 事件描述");
warn!("[模块名] 警告信息");
error!("[模块名] 错误信息");
```

### 系统注册
```rust
app.add_systems(
    Update,
    my_system
        .in_set(GameplaySet::Logic)
        .run_if(in_state(GameState::Playing))
);
```

### 错误处理
```rust
// ✅ 推荐
let Ok(value) = query.get_single() else {
    warn!("[Module] Query failed");
    return;
};

// ❌ 避免
let value = query.get_single().unwrap();
```

---

## 相关资源

### Bevy 官方文档
- [Bevy 0.17 Release Notes](https://bevyengine.org/news/bevy-0-17/)
- [Bevy Examples](https://github.com/bevyengine/bevy/tree/main/examples)
- [Bevy Cheatbook](https://bevy-cheatbook.github.io/)

### 第三方库
- [bevy_asset_loader](https://github.com/NiklasEi/bevy_asset_loader) - 资源加载
- [bevy_kira_audio](https://github.com/NiklasEi/bevy_kira_audio) - 音频播放
- [bevy-inspector-egui](https://github.com/jakobhellermann/bevy-inspector-egui) - 调试工具

### 社区
- [Bevy Discord](https://discord.gg/bevy)
- [Bevy GitHub Discussions](https://github.com/bevyengine/bevy/discussions)

---

## 常见问题

### Q: 为什么只用一个相机？
A: 单相机架构更简单、性能更好，避免相机切换的复杂性。菜单 UI 可以通过 ZIndex 叠加在 3D 场景上。

### Q: 为什么实体始终可见？
A: Menu 状态需要显示游戏场景作为背景，且通过状态过滤系统执行更符合 ECS 理念。

### Q: 如何实现模糊效果？
A: 当前使用半透明黑色遮罩（60% 不透明），未来可升级为真正的高斯模糊后处理。

### Q: 为什么在 AssetLoading 阶段生成实体？
A: 确保资源加载完成后实体立即可用，避免 Menu 状态时场景为空。

---

## 变更日志

### 2025-10-25 - v0.2.0 功能完善
- ✅ 实现完整的本地化系统（中文/English）
- ✅ 添加语言切换按钮（🌐 图标 + 文本）
- ✅ 完善 UI 系统（主菜单、设置菜单）
- ✅ 优化用户体验（按钮显示下一个语言）

### 2025-10-19 - v2.0 架构重构
- ✅ 采用单相机架构（Camera3d）
- ✅ 实体预生成，始终可见
- ✅ 菜单叠加渲染设计
- ✅ 完整的模块化结构
- ✅ 遵循 Bevy 0.17.2 最佳实践

---

## 下一步行动

### 核心功能已完成 ✅
项目已实现所有核心功能，可正常运行和游玩。

### 可选增强功能
1. **游戏内 HUD** - 血条、分数、迷你地图
2. **音频内容** - 添加背景音乐和音效
3. **3D 模型** - 替换方块为真实角色模型
4. **更多语言** - 支持日语、韩语等
5. **CI/CD** - 自动化构建与发布流程

---

最后更新：2025-10-25
维护者：Claude Code
