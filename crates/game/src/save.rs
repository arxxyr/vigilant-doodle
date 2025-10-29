//! 存档系统
//!
//! 负责游戏状态的保存和加载

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

use vigilant_doodle_core::save::SaveManager;

// ============================================================================
// 存档数据结构
// ============================================================================

/// 存档数据
#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct SaveData {
    /// 存档版本
    pub version: u32,
    /// 保存时间（Unix 时间戳）
    pub timestamp: u64,
    /// 玩家数据
    pub player: PlayerSaveData,
    /// 敌人数据列表
    pub enemies: Vec<EnemySaveData>,
    /// 游戏进度标记
    pub has_active_game: bool,
}

/// 玩家存档数据
#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct PlayerSaveData {
    pub position: [f32; 3],
    pub rotation: [f32; 4], // 四元数
    pub speed: f32,
}

/// 敌人存档数据
#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct EnemySaveData {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    // AI 配置
    pub move_speed: f32,
    pub stop_distance: f32,
    // 检测配置
    pub detection_range: f32,
    pub lose_target_range: f32,
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            version: 1,
            timestamp: 0,
            player: PlayerSaveData {
                position: [0.0, 0.5, 0.0],
                rotation: [0.0, 0.0, 0.0, 1.0],
                speed: 10.0,
            },
            enemies: Vec::new(),
            has_active_game: false,
        }
    }
}

// ============================================================================
// 存档系统插件
// ============================================================================

/// 游戏存档插件（具体实现）
pub struct GameSavePlugin;

impl Plugin for GameSavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_save_request,
                handle_load_request,
            )
        );

        info!("[GameSavePlugin] 游戏存档系统已加载");
    }
}

// ============================================================================
// 系统实现
// ============================================================================

/// 处理保存请求
fn handle_save_request(
    mut save_manager: ResMut<SaveManager>,
    player_query: Query<(&Transform, &vigilant_doodle_gameplay::Player), Without<vigilant_doodle_gameplay::Enemy>>,
    enemy_query: Query<
        (
            &Transform,
            &vigilant_doodle_ai::EnemyAIConfig,
            &vigilant_doodle_ai::DetectionConfig,
        ),
        With<vigilant_doodle_gameplay::Enemy>,
    >,
    game_progress: Res<vigilant_doodle_core::state::GameProgress>,
) {
    if !save_manager.pending_save {
        return;
    }
    save_manager.pending_save = false;

    // 收集玩家数据
    let player_data = if let Ok((transform, player)) = player_query.single() {
        PlayerSaveData {
            position: transform.translation.to_array(),
            rotation: transform.rotation.to_array(),
            speed: player.speed,
        }
    } else {
        warn!("[SaveManager] 无法找到玩家实体");
        return;
    };

    // 收集敌人数据
    let enemies_data: Vec<EnemySaveData> = enemy_query
        .iter()
        .map(|(transform, ai_config, detection_config)| EnemySaveData {
            position: transform.translation.to_array(),
            rotation: transform.rotation.to_array(),
            move_speed: ai_config.move_speed,
            stop_distance: ai_config.stop_distance,
            detection_range: detection_config.detection_range,
            lose_target_range: detection_config.lose_target_range,
        })
        .collect();

    // 创建存档数据
    let save_data = SaveData {
        version: 1,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        player: player_data,
        enemies: enemies_data,
        has_active_game: game_progress.has_active_game,
    };

    // 保存到文件（二进制加密格式）
    let save_path = SaveManager::get_save_path(save_manager.current_slot);

    // 序列化为二进制（bincode 2.0 API）
    let serialized = match bincode::encode_to_vec(&save_data, bincode::config::standard()) {
        Ok(data) => data,
        Err(e) => {
            error!("[SaveManager] 序列化失败: {}", e);
            return;
        }
    };

    // 加密
    let encrypted = match vigilant_doodle_core::crypto::encrypt(&serialized) {
        Ok(data) => data,
        Err(e) => {
            error!("[SaveManager] 加密失败: {}", e);
            return;
        }
    };

    // 写入文件
    if let Err(e) = fs::write(&save_path, encrypted) {
        error!("[SaveManager] 保存失败: {}", e);
    } else {
        info!("[SaveManager] 游戏已保存到: {:?}", save_path);
    }
}

/// 处理加载请求
fn handle_load_request(
    mut save_manager: ResMut<SaveManager>,
    mut player_query: Query<(&mut Transform, &mut vigilant_doodle_gameplay::Player), Without<vigilant_doodle_gameplay::Enemy>>,
    mut enemy_query: Query<
        (
            &mut Transform,
            &mut vigilant_doodle_ai::EnemyAIConfig,
            &mut vigilant_doodle_ai::DetectionConfig,
        ),
        With<vigilant_doodle_gameplay::Enemy>,
    >,
    mut game_progress: ResMut<vigilant_doodle_core::state::GameProgress>,
) {
    if !save_manager.pending_load {
        return;
    }
    save_manager.pending_load = false;

    let save_path = SaveManager::get_save_path(save_manager.current_slot);

    // 检查文件是否存在
    if !save_path.exists() {
        warn!("[SaveManager] 存档不存在: {:?}", save_path);
        return;
    }

    // 读取文件
    let encrypted = match fs::read(&save_path) {
        Ok(data) => data,
        Err(e) => {
            error!("[SaveManager] 读取存档失败: {}", e);
            return;
        }
    };

    // 解密
    let decrypted = match vigilant_doodle_core::crypto::decrypt(&encrypted) {
        Ok(data) => data,
        Err(e) => {
            error!("[SaveManager] 解密存档失败（文件可能已损坏或被篡改）: {}", e);
            return;
        }
    };

    // 反序列化（bincode 2.0 API）
    let (save_data, _): (SaveData, _) = match bincode::decode_from_slice(&decrypted, bincode::config::standard()) {
        Ok(data) => data,
        Err(e) => {
            error!("[SaveManager] 解析存档失败: {}", e);
            return;
        }
    };

    info!("[SaveManager] 正在加载存档（版本: {}, 时间戳: {}）",
          save_data.version, save_data.timestamp);

    // 恢复玩家数据
    if let Ok((mut transform, mut player)) = player_query.single_mut() {
        transform.translation = Vec3::from_array(save_data.player.position);
        transform.rotation = Quat::from_array(save_data.player.rotation);
        player.speed = save_data.player.speed;
        info!("[SaveManager] 玩家数据已恢复");
    } else {
        warn!("[SaveManager] 无法找到玩家实体");
    }

    // 恢复敌人数据
    let mut enemy_iter = enemy_query.iter_mut();
    for enemy_save in &save_data.enemies {
        if let Some((mut transform, mut ai_config, mut detection_config)) = enemy_iter.next() {
            transform.translation = Vec3::from_array(enemy_save.position);
            transform.rotation = Quat::from_array(enemy_save.rotation);
            ai_config.move_speed = enemy_save.move_speed;
            ai_config.stop_distance = enemy_save.stop_distance;
            detection_config.detection_range = enemy_save.detection_range;
            detection_config.lose_target_range = enemy_save.lose_target_range;
        } else {
            warn!("[SaveManager] 敌人数量不匹配");
            break;
        }
    }
    info!("[SaveManager] 敌人数据已恢复（{}个）", save_data.enemies.len());

    // 恢复游戏进度
    game_progress.has_active_game = save_data.has_active_game;

    info!("[SaveManager] 存档加载完成");
}
