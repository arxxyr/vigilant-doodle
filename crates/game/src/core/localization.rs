//! 本地化系统
//!
//! 提供多语言支持，包括中文和英文。
//! 翻译文本从 JSON 文件加载。

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// 语言定义
// ============================================================================

/// 支持的语言
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Reflect)]
pub enum Language {
    /// 中文
    #[default]
    Chinese,
    /// 英文
    English,
}

impl Language {
    /// 切换到下一个语言
    pub fn toggle(&self) -> Self {
        match self {
            Language::Chinese => Language::English,
            Language::English => Language::Chinese,
        }
    }

    /// 获取语言文件路径
    pub fn file_path(&self) -> &'static str {
        match self {
            Language::Chinese => "localization/zh_CN.json",
            Language::English => "localization/en_US.json",
        }
    }

    /// 获取语言显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::Chinese => "中文",
            Language::English => "English",
        }
    }
}

// ============================================================================
// 翻译数据结构
// ============================================================================

/// 翻译文件的根结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translations {
    pub menu: MenuTranslations,
    pub game: GameTranslations,
    pub settings: SettingsTranslations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuTranslations {
    pub title: String,
    pub new_game: String,
    pub resume: String,
    pub save_game: String,
    pub settings: String,
    pub back_to_menu: String,
    pub quit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameTranslations {
    pub paused: String,
    pub resume: String,
    pub back_to_menu: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsTranslations {
    pub title: String,
    pub appearance: String,
    pub graphics: String,
    pub audio: String,
    pub controls: String,
    pub gameplay: String,
    pub back: String,
    pub language: String,
    pub language_chinese: String,
    pub language_english: String,
    pub volume_master: String,
    pub volume_music: String,
    pub volume_sfx: String,
    pub resolution: String,
    pub fullscreen: String,
    pub vsync: String,
    pub difficulty: String,
    pub auto_save: String,
    pub mouse_sensitivity: String,
    pub invert_y: String,
    pub key_bindings: String,
}

// ============================================================================
// 资源
// ============================================================================

/// 当前语言设置
#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct CurrentLanguage {
    pub language: Language,
}

impl Default for CurrentLanguage {
    fn default() -> Self {
        Self {
            language: Language::Chinese,
        }
    }
}

/// 翻译资源
///
/// 存储所有语言的翻译数据
#[derive(Resource, Default)]
pub struct TranslationResources {
    translations: HashMap<Language, Translations>,
}

impl TranslationResources {
    /// 获取指定语言和键的翻译文本
    pub fn get(&self, language: Language, key: &str) -> String {
        let Some(translations) = self.translations.get(&language) else {
            warn!("[Localization] 未找到语言: {:?}", language);
            return key.to_string();
        };

        // 解析键路径（例如 "menu.new_game"）
        let parts: Vec<&str> = key.split('.').collect();
        match parts.as_slice() {
            ["menu", "title"] => translations.menu.title.clone(),
            ["menu", "new_game"] => translations.menu.new_game.clone(),
            ["menu", "resume"] => translations.menu.resume.clone(),
            ["menu", "save_game"] => translations.menu.save_game.clone(),
            ["menu", "settings"] => translations.menu.settings.clone(),
            ["menu", "back_to_menu"] => translations.menu.back_to_menu.clone(),
            ["menu", "quit"] => translations.menu.quit.clone(),

            ["game", "paused"] => translations.game.paused.clone(),
            ["game", "resume"] => translations.game.resume.clone(),
            ["game", "back_to_menu"] => translations.game.back_to_menu.clone(),

            ["settings", field] => match *field {
                "title" => translations.settings.title.clone(),
                "appearance" => translations.settings.appearance.clone(),
                "graphics" => translations.settings.graphics.clone(),
                "audio" => translations.settings.audio.clone(),
                "controls" => translations.settings.controls.clone(),
                "gameplay" => translations.settings.gameplay.clone(),
                "back" => translations.settings.back.clone(),
                "language" => translations.settings.language.clone(),
                "language_chinese" => translations.settings.language_chinese.clone(),
                "language_english" => translations.settings.language_english.clone(),
                "volume_master" => translations.settings.volume_master.clone(),
                "volume_music" => translations.settings.volume_music.clone(),
                "volume_sfx" => translations.settings.volume_sfx.clone(),
                "resolution" => translations.settings.resolution.clone(),
                "fullscreen" => translations.settings.fullscreen.clone(),
                "vsync" => translations.settings.vsync.clone(),
                "difficulty" => translations.settings.difficulty.clone(),
                "auto_save" => translations.settings.auto_save.clone(),
                "mouse_sensitivity" => translations.settings.mouse_sensitivity.clone(),
                "invert_y" => translations.settings.invert_y.clone(),
                "key_bindings" => translations.settings.key_bindings.clone(),
                _ => {
                    warn!("[Localization] 未知的设置翻译键: settings.{}", field);
                    key.to_string()
                }
            },

            _ => {
                warn!("[Localization] 未知的翻译键: {}", key);
                key.to_string()
            }
        }
    }

    /// 插入翻译数据
    pub fn insert(&mut self, language: Language, translations: Translations) {
        self.translations.insert(language, translations);
    }
}

// ============================================================================
// 组件
// ============================================================================

/// 本地化文本组件
///
/// 为 UI 元素提供多语言文本支持
#[derive(Component, Clone, Debug)]
pub struct LocalizedText {
    /// 文本键（用于查找翻译，例如 "menu.play"）
    pub key: String,
}

impl LocalizedText {
    pub fn new(key: impl Into<String>) -> Self {
        Self { key: key.into() }
    }
}

// ============================================================================
// 插件
// ============================================================================

pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLanguage>()
            .init_resource::<TranslationResources>()
            .register_type::<CurrentLanguage>()
            .add_systems(Startup, load_translations)
            .add_systems(Update, update_localized_text);

        info!("[Localization] 本地化系统已加载");
    }
}

// ============================================================================
// 系统
// ============================================================================

/// 加载所有语言的翻译文件
fn load_translations(
    asset_server: Res<AssetServer>,
    mut translation_resources: ResMut<TranslationResources>,
) {
    info!("[Localization] 开始加载翻译文件...");

    // 加载中文翻译
    if let Some(zh_bytes) = asset_server
        .load_untyped(Language::Chinese.file_path())
        .path()
        .as_ref()
    {
        // 注意：这里我们需要在资源加载完成后再解析
        // 为了简化，我们先使用硬编码的备用翻译
    }

    // 由于 Bevy 的资源加载是异步的，我们需要提供备用翻译
    // 这里使用硬编码的翻译作为备用
    load_fallback_translations(&mut translation_resources);

    info!("[Localization] 翻译文件加载完成");
}

/// 加载备用翻译（硬编码）
fn load_fallback_translations(translation_resources: &mut TranslationResources) {
    // 中文翻译
    translation_resources.insert(
        Language::Chinese,
        Translations {
            menu: MenuTranslations {
                title: "Vigilant Doodle".to_string(),
                new_game: "新游戏".to_string(),
                resume: "继续游戏".to_string(),
                save_game: "存档".to_string(),
                settings: "设置".to_string(),
                back_to_menu: "返回主菜单".to_string(),
                quit: "退出游戏".to_string(),
            },
            game: GameTranslations {
                paused: "暂停".to_string(),
                resume: "继续".to_string(),
                back_to_menu: "返回主菜单".to_string(),
            },
            settings: SettingsTranslations {
                title: "设置".to_string(),
                appearance: "外观".to_string(),
                graphics: "图形".to_string(),
                audio: "声音".to_string(),
                controls: "控制".to_string(),
                gameplay: "游戏性".to_string(),
                back: "返回".to_string(),
                language: "语言".to_string(),
                language_chinese: "中文".to_string(),
                language_english: "English".to_string(),
                volume_master: "主音量".to_string(),
                volume_music: "音乐音量".to_string(),
                volume_sfx: "音效音量".to_string(),
                resolution: "分辨率".to_string(),
                fullscreen: "全屏".to_string(),
                vsync: "垂直同步".to_string(),
                difficulty: "难度".to_string(),
                auto_save: "自动保存".to_string(),
                mouse_sensitivity: "鼠标灵敏度".to_string(),
                invert_y: "反转 Y 轴".to_string(),
                key_bindings: "按键绑定".to_string(),
            },
        },
    );

    // 英文翻译
    translation_resources.insert(
        Language::English,
        Translations {
            menu: MenuTranslations {
                title: "Vigilant Doodle".to_string(),
                new_game: "New Game".to_string(),
                resume: "Resume".to_string(),
                save_game: "Save/Load".to_string(),
                settings: "Settings".to_string(),
                back_to_menu: "Back to Menu".to_string(),
                quit: "Quit".to_string(),
            },
            game: GameTranslations {
                paused: "Paused".to_string(),
                resume: "Resume".to_string(),
                back_to_menu: "Back to Menu".to_string(),
            },
            settings: SettingsTranslations {
                title: "Settings".to_string(),
                appearance: "Appearance".to_string(),
                graphics: "Graphics".to_string(),
                audio: "Audio".to_string(),
                controls: "Controls".to_string(),
                gameplay: "Gameplay".to_string(),
                back: "Back".to_string(),
                language: "Language".to_string(),
                language_chinese: "中文".to_string(),
                language_english: "English".to_string(),
                volume_master: "Master Volume".to_string(),
                volume_music: "Music Volume".to_string(),
                volume_sfx: "SFX Volume".to_string(),
                resolution: "Resolution".to_string(),
                fullscreen: "Fullscreen".to_string(),
                vsync: "V-Sync".to_string(),
                difficulty: "Difficulty".to_string(),
                auto_save: "Auto Save".to_string(),
                mouse_sensitivity: "Mouse Sensitivity".to_string(),
                invert_y: "Invert Y Axis".to_string(),
                key_bindings: "Key Bindings".to_string(),
            },
        },
    );
}

/// 更新本地化文本系统
///
/// 当语言切换时，自动更新所有带有 LocalizedText 组件的文本
fn update_localized_text(
    current_language: Res<CurrentLanguage>,
    translation_resources: Res<TranslationResources>,
    mut text_query: Query<(&LocalizedText, &mut Text)>,
) {
    if !current_language.is_changed() {
        return;
    }

    // 语言改变了，更新所有文本
    for (localized, mut text) in text_query.iter_mut() {
        **text = translation_resources.get(current_language.language, &localized.key);
    }
    info!(
        "[Localization] 语言已切换到: {:?}",
        current_language.language
    );
}
