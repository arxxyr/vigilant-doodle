//! 游戏平衡性配置系统
//!
//! 从 JSON 配置文件加载游戏平衡参数，包括：
//! - 玩家移动速度、跳跃力等
//! - 敌人 AI 参数（移动速度、检测范围等）
//!
//! 配置文件位置：`assets/balance.json`

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// 游戏平衡配置
///
/// 从 `assets/balance.json` 加载
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct BalanceConfig {
    pub player: PlayerBalance,
    pub enemy: EnemyBalance,
}

impl Default for BalanceConfig {
    fn default() -> Self {
        Self {
            player: PlayerBalance::default(),
            enemy: EnemyBalance::default(),
        }
    }
}

/// 玩家平衡参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerBalance {
    /// 移动速度
    pub speed: f32,
    /// 旋转速度（弧度/秒）
    pub rotation_speed: f32,
    /// 跳跃力度
    pub jump_force: f32,
    /// 重力加速度
    pub gravity: f32,
    /// 敌人检测范围（多少距离内会朝向敌人）
    pub detection_range: f32,
}

impl Default for PlayerBalance {
    fn default() -> Self {
        Self {
            speed: 7.5,
            rotation_speed: 10.0,
            jump_force: 7.5,
            gravity: -30.0,
            detection_range: 10.0,
        }
    }
}

/// 敌人平衡参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyBalance {
    /// AI 行为参数
    pub ai: EnemyAIBalance,
    /// 检测参数
    pub detection: EnemyDetectionBalance,
}

impl Default for EnemyBalance {
    fn default() -> Self {
        Self {
            ai: EnemyAIBalance::default(),
            detection: EnemyDetectionBalance::default(),
        }
    }
}

/// 敌人 AI 行为参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyAIBalance {
    /// 移动速度
    pub move_speed: f32,
    /// 停止距离（到目标多近时停止）
    pub stop_distance: f32,
    /// 旋转速度（朝向目标的速度）
    pub rotation_speed: f32,
}

impl Default for EnemyAIBalance {
    fn default() -> Self {
        Self {
            move_speed: 3.5,
            stop_distance: 1.5,
            rotation_speed: 10.0,
        }
    }
}

/// 敌人检测参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyDetectionBalance {
    /// 初始检测范围（敌人首次发现玩家的距离）
    pub detection_range: f32,
    /// 失去目标范围（一旦发现后，需要这个距离才会失去目标）
    pub lose_target_range: f32,
    /// 视野角度（度数，未来扩展用）
    pub field_of_view: f32,
}

impl Default for EnemyDetectionBalance {
    fn default() -> Self {
        Self {
            detection_range: 17.5,
            lose_target_range: 25.0,
            field_of_view: 360.0,
        }
    }
}

/// 平衡配置插件
pub struct BalancePlugin;

impl Plugin for BalancePlugin {
    fn build(&self, app: &mut App) {
        // 立即插入默认配置作为后备（保证资源始终存在）
        app.insert_resource(BalanceConfig::default());

        // 在 Startup 阶段尝试从文件加载配置
        app.add_systems(Startup, load_balance_config);
    }
}

/// 加载平衡配置系统
fn load_balance_config(mut config: ResMut<BalanceConfig>) {
    // 尝试从文件加载配置
    match load_balance_from_file() {
        Ok(loaded_config) => {
            info!("[Balance] 成功加载平衡配置文件");
            *config = loaded_config;
        }
        Err(e) => {
            warn!("[Balance] 无法加载平衡配置文件，使用默认值: {}", e);
            // config 已经是默认值，无需修改
        }
    };

    // 输出关键参数到日志
    info!("[Balance] 玩家速度: {}", config.player.speed);
    info!("[Balance] 敌人速度: {}", config.enemy.ai.move_speed);
    info!(
        "[Balance] 敌人检测范围: {}",
        config.enemy.detection.detection_range
    );
}

/// 从文件加载平衡配置
fn load_balance_from_file() -> Result<BalanceConfig, Box<dyn std::error::Error>> {
    // 获取 assets 目录路径
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let balance_path = std::path::Path::new(manifest_dir)
        .parent()
        .ok_or("无法获取父目录")?
        .parent()
        .ok_or("无法获取父目录")?
        .join("assets")
        .join("balance.json");

    // 读取文件
    let content = std::fs::read_to_string(&balance_path)
        .map_err(|e| format!("读取文件失败: {}", e))?;

    // 解析 JSON
    let config: BalanceConfig =
        serde_json::from_str(&content).map_err(|e| format!("解析 JSON 失败: {}", e))?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_balance() {
        let config = BalanceConfig::default();
        assert_eq!(config.player.speed, 7.5);
        assert_eq!(config.enemy.ai.move_speed, 3.5);
        assert_eq!(config.enemy.detection.detection_range, 17.5);
    }

    #[test]
    fn test_serialization() {
        let config = BalanceConfig::default();
        let json = serde_json::to_string_pretty(&config).unwrap();
        assert!(json.contains("player"));
        assert!(json.contains("enemy"));
    }
}
