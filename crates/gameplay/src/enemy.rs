//! 敌人基础组件
//!
//! 定义敌人实体的基础属性和生成逻辑
//! AI 组件由 game crate 统一添加

use bevy::prelude::*;

use vigilant_doodle_assets::GameAssets;
use vigilant_doodle_core::state::GameState;
use crate::movement::CollisionRadius;

/// 敌人标记组件
///
/// 用于标识一个实体是敌人
#[derive(Component)]
pub struct Enemy;

/// 敌人属性
///
/// 存储敌人的基础属性（生命值、攻击力等）
#[derive(Component)]
pub struct EnemyStats {
    /// 最大生命值
    pub max_health: f32,
    /// 当前生命值
    pub current_health: f32,
    /// 攻击力（未来扩展）
    #[allow(dead_code)]
    pub attack_power: f32,
}

impl Default for EnemyStats {
    fn default() -> Self {
        Self {
            max_health: 100.0,
            current_health: 100.0,
            attack_power: 10.0,
        }
    }
}

/// 敌人插件
///
/// 负责敌人实体的生成
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::AssetLoading),
            spawn_enemies.after(vigilant_doodle_assets::load_assets),
        );
    }
}

/// 生成敌人系统
fn spawn_enemies(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    use rand::Rng;
    let mut rng = rand::rng();

    info!("[Enemy] 开始生成敌人...");

    // 生成 3 个敌人（适配扩大后的地图范围）
    for i in 0..3 {
        let x = rng.random_range(-60.0..60.0);
        let z = rng.random_range(-30.0..30.0);

        commands.spawn((
            // 使用 glb 模型
            SceneRoot(assets.enemy_model.clone()),
            Transform::from_xyz(x, 0.0, z),
            // 敌人标记
            Enemy,
            // 敌人属性
            EnemyStats::default(),
            // 碰撞检测
            CollisionRadius::new(0.6),
            // AI 组件将由 game crate 的系统添加
            // 调试名称
            Name::new(format!("Enemy_{}", i)),
        ));

        debug!("[Enemy] 生成敌人 {} at ({:.1}, {:.1})", i, x, z);
    }

    info!("[Enemy] 敌人生成完成（共 3 个，使用模型）");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enemy_stats_default() {
        let stats = EnemyStats::default();
        assert_eq!(stats.max_health, 100.0);
        assert_eq!(stats.current_health, 100.0);
        assert_eq!(stats.attack_power, 10.0);
    }
}
