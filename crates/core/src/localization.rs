//! 本地化系统
//!
//! 提供多语言支持，包括中文和英文。
//! 翻译文本从 JSON 文件异步加载。

use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use bevy::prelude::*;
use bevy::tasks::ConditionalSendFuture;
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

/// 翻译文件的根结构（作为 Bevy Asset）
#[derive(Debug, Clone, Serialize, Deserialize, Asset, TypePath)]
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
// Asset Loader
// ============================================================================

/// JSON 翻译文件的 AssetLoader
#[derive(Default)]
pub struct TranslationsLoader;

impl AssetLoader for TranslationsLoader {
    type Asset = Translations;
    type Settings = ();
    type Error = std::io::Error;

    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let translations: Translations = serde_json::from_slice(&bytes)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

            Ok(translations)
        }
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
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
#[derive(Resource)]
pub struct TranslationResources {
    /// 翻译文件的句柄
    handles: HashMap<Language, Handle<Translations>>,
    /// 缓存的翻译数据（从 Assets 中提取）
    translations: HashMap<Language, Translations>,
}

impl Default for TranslationResources {
    fn default() -> Self {
        Self {
            handles: HashMap::new(),
            translations: HashMap::new(),
        }
    }
}

impl TranslationResources {
    /// 设置翻译文件句柄
    pub fn set_handle(&mut self, language: Language, handle: Handle<Translations>) {
        self.handles.insert(language, handle);
    }

    /// 从 Assets 中更新翻译数据
    pub fn update_from_assets(&mut self, assets: &Assets<Translations>) {
        for (language, handle) in &self.handles {
            if let Some(translations) = assets.get(handle) {
                self.translations.insert(*language, translations.clone());
            }
        }
    }

    /// 检查所有翻译是否已加载
    pub fn all_loaded(&self) -> bool {
        self.translations.len() == self.handles.len() && !self.handles.is_empty()
    }

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
            .init_asset::<Translations>()
            .init_asset_loader::<TranslationsLoader>()
            .add_systems(Startup, load_translation_handles)
            .add_systems(Update, (update_translation_data, update_localized_text).chain());

        info!("[Localization] 本地化系统已加载");
    }
}

// ============================================================================
// 系统
// ============================================================================

/// 加载翻译文件句柄
fn load_translation_handles(
    asset_server: Res<AssetServer>,
    mut translation_resources: ResMut<TranslationResources>,
) {
    info!("[Localization] 开始加载翻译文件...");

    // 加载中文翻译
    let zh_handle: Handle<Translations> = asset_server.load(Language::Chinese.file_path());
    translation_resources.set_handle(Language::Chinese, zh_handle);

    // 加载英文翻译
    let en_handle: Handle<Translations> = asset_server.load(Language::English.file_path());
    translation_resources.set_handle(Language::English, en_handle);

    info!(
        "[Localization] 翻译文件句柄已创建: {:?}, {:?}",
        Language::Chinese.file_path(),
        Language::English.file_path()
    );
}

/// 更新翻译数据
///
/// 从 Assets 中提取已加载的翻译数据到缓存
fn update_translation_data(
    translations_assets: Res<Assets<Translations>>,
    mut translation_resources: ResMut<TranslationResources>,
) {
    // 仅在翻译资源变化时更新
    if translations_assets.is_changed() {
        let was_loaded = translation_resources.all_loaded();
        translation_resources.update_from_assets(&translations_assets);
        let now_loaded = translation_resources.all_loaded();

        // 首次完成加载时记录日志
        if !was_loaded && now_loaded {
            info!("[Localization] 所有翻译文件加载完成");
        }
    }
}

/// 更新本地化文本系统
///
/// 当语言切换时，自动更新所有带有 LocalizedText 组件的文本
fn update_localized_text(
    current_language: Res<CurrentLanguage>,
    translation_resources: Res<TranslationResources>,
    mut text_query: Query<(&LocalizedText, &mut Text)>,
) {
    // 检查语言是否改变 或 翻译资源是否刚加载完成
    if !current_language.is_changed() && !translation_resources.is_changed() {
        return;
    }

    // 确保翻译已加载
    if !translation_resources.all_loaded() {
        return;
    }

    // 语言改变了或翻译刚加载，更新所有文本
    for (localized, mut text) in text_query.iter_mut() {
        **text = translation_resources.get(current_language.language, &localized.key);
    }

    if current_language.is_changed() {
        info!(
            "[Localization] 语言已切换到: {:?}",
            current_language.language
        );
    }
}
