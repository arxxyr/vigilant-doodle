//! 存档系统基础设施
//!
//! 提供存档管理的基础功能，不包含具体的游戏实体序列化逻辑

use bevy::prelude::*;
use std::fs;
use std::path::PathBuf;

/// 存档管理器资源
#[derive(Resource, Default)]
pub struct SaveManager {
    /// 当前存档槽位（0-2，支持3个存档）
    pub current_slot: usize,
    /// 是否有待保存的数据
    pub pending_save: bool,
    /// 是否有待加载的数据
    pub pending_load: bool,
}

impl SaveManager {
    /// 获取存档文件路径
    pub fn get_save_path(slot: usize) -> PathBuf {
        let path = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("vigilant-doodle");

        // 确保目录存在
        fs::create_dir_all(&path).ok();

        path.join(format!("save_{}.sav", slot))
    }

    /// 请求保存游戏
    pub fn request_save(&mut self) {
        self.pending_save = true;
        info!("[SaveManager] 请求保存游戏到槽位 {}", self.current_slot);
    }

    /// 请求加载游戏
    pub fn request_load(&mut self) {
        self.pending_load = true;
        info!("[SaveManager] 请求加载游戏从槽位 {}", self.current_slot);
    }

    /// 检查存档是否存在
    pub fn save_exists(slot: usize) -> bool {
        Self::get_save_path(slot).exists()
    }
}

/// 存档系统插件
pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SaveManager>();
        info!("[SavePlugin] 存档管理器已加载");
    }
}
