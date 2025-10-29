//! Fluent i18n 本地化系统
//!
//! 基于 Project Fluent 提供强大的多语言支持。
//!
//! ## 特性
//! - 使用 Fluent (.ftl) 格式的翻译文件
//! - 支持参数化翻译（`{ $param }` 风格）
//! - 支持复数、性别等高级语言特性
//! - 类型安全的键常量
//! - 自动文本更新
//!
//! ## 目录结构
//! ```
//! assets/i18n/
//! ├── zh-Hans/
//! │   ├── main.ftl
//! │   ├── menu.ftl
//! │   └── settings.ftl
//! └── en/
//!     ├── main.ftl
//!     ├── menu.ftl
//!     └── settings.ftl
//! ```

use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use bevy::prelude::*;
use bevy::tasks::ConditionalSendFuture;
use fluent::{FluentArgs, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;
use std::collections::HashMap;
use unic_langid::{langid, LanguageIdentifier};

// ============================================================================
// 语言定义
// ============================================================================

/// 支持的语言
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Reflect)]
pub enum Language {
    /// 简体中文
    #[default]
    SimplifiedChinese,
    /// 英文
    English,
}

impl Language {
    /// 切换到下一个语言
    pub fn toggle(&self) -> Self {
        match self {
            Language::SimplifiedChinese => Language::English,
            Language::English => Language::SimplifiedChinese,
        }
    }

    /// 获取语言目录名
    pub fn dir_name(&self) -> &'static str {
        match self {
            Language::SimplifiedChinese => "zh-Hans",
            Language::English => "en",
        }
    }

    /// 获取语言标识符（用于 Fluent）
    pub fn lang_id(&self) -> LanguageIdentifier {
        match self {
            Language::SimplifiedChinese => langid!("zh-Hans"),
            Language::English => langid!("en"),
        }
    }

    /// 获取语言显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::SimplifiedChinese => "中文",
            Language::English => "English",
        }
    }

    /// 获取所有翻译文件路径
    pub fn translation_files(&self) -> Vec<String> {
        let dir = self.dir_name();
        vec![
            format!("i18n/{}/main.ftl", dir),
            format!("i18n/{}/menu.ftl", dir),
            format!("i18n/{}/settings.ftl", dir),
        ]
    }
}

// ============================================================================
// Fluent 资源（作为 Bevy Asset）
// ============================================================================

/// Fluent 资源文件（单个 .ftl 文件）
#[derive(Debug, Clone, Asset, TypePath)]
pub struct FluentAsset {
    /// 原始 Fluent 源文本
    pub source: String,
}

/// Fluent 资源加载器
#[derive(Default)]
pub struct FluentAssetLoader;

impl AssetLoader for FluentAssetLoader {
    type Asset = FluentAsset;
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
            let source = String::from_utf8(bytes)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

            Ok(FluentAsset { source })
        }
    }

    fn extensions(&self) -> &[&str] {
        &["ftl"]
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
            language: Language::SimplifiedChinese,
        }
    }
}

/// 翻译资源（管理所有语言的 Fluent Bundle）
#[derive(Resource)]
pub struct TranslationResources {
    /// Fluent 文件句柄
    handles: HashMap<Language, Vec<Handle<FluentAsset>>>,
    /// Fluent Bundles（每个语言一个）
    bundles: HashMap<Language, fluent::bundle::FluentBundle<FluentResource, IntlLangMemoizer>>,
}

impl Default for TranslationResources {
    fn default() -> Self {
        Self {
            handles: HashMap::new(),
            bundles: HashMap::new(),
        }
    }
}

impl TranslationResources {
    /// 设置语言的翻译文件句柄
    pub fn set_handles(&mut self, language: Language, handles: Vec<Handle<FluentAsset>>) {
        self.handles.insert(language, handles);
    }

    /// 从 Assets 中更新 Fluent Bundles
    pub fn update_from_assets(&mut self, assets: &Assets<FluentAsset>) {
        for (language, handles) in &self.handles {
            // 收集该语言的所有 Fluent 资源
            let mut resources = Vec::new();
            let mut all_loaded = true;

            for handle in handles {
                if let Some(asset) = assets.get(handle) {
                    match FluentResource::try_new(asset.source.clone()) {
                        Ok(resource) => resources.push(resource),
                        Err((resource, errors)) => {
                            for error in &errors {
                                error!("[i18n] Fluent 解析错误: {:?}", error);
                            }
                            // 即使有错误，也使用部分解析的资源
                            resources.push(resource);
                        }
                    }
                } else {
                    all_loaded = false;
                }
            }

            // 如果所有资源都已加载，创建或更新 Bundle
            if all_loaded && !resources.is_empty() {
                let lang_id = language.lang_id();
                let mut bundle: fluent::bundle::FluentBundle<FluentResource, IntlLangMemoizer> =
                    fluent::bundle::FluentBundle::new_concurrent(vec![lang_id.clone()]);

                // 添加所有资源到 bundle
                for resource in resources {
                    if let Err(errors) = bundle.add_resource(resource) {
                        for error in errors {
                            warn!("[i18n] Fluent 资源添加失败: {:?}", error);
                        }
                    }
                }

                self.bundles.insert(*language, bundle);
            }
        }
    }

    /// 检查所有翻译是否已加载
    pub fn all_loaded(&self) -> bool {
        self.bundles.len() == self.handles.len() && !self.handles.is_empty()
    }

    /// 获取翻译文本
    ///
    /// # 示例
    /// ```
    /// let text = resources.get(Language::SimplifiedChinese, "menu-title");
    /// ```
    pub fn get(&self, language: Language, key: &str) -> String {
        let Some(bundle) = self.bundles.get(&language) else {
            warn!("[i18n] 未找到语言: {:?}", language);
            return key.to_string();
        };

        let Some(msg) = bundle.get_message(key) else {
            warn!("[i18n] 未找到翻译键: {}", key);
            return key.to_string();
        };

        let Some(pattern) = msg.value() else {
            warn!("[i18n] 消息没有值: {}", key);
            return key.to_string();
        };

        let mut errors = vec![];
        let value = bundle.format_pattern(pattern, None, &mut errors);

        for error in errors {
            warn!("[i18n] 格式化错误: {:?}", error);
        }

        value.to_string()
    }

    /// 获取带参数的翻译文本
    ///
    /// # 示例
    /// ```
    /// let mut args = FluentArgs::new();
    /// args.set("name", "Alice");
    /// let text = resources.t(Language::SimplifiedChinese, "greeting", Some(&args));
    /// ```
    pub fn t(&self, language: Language, key: &str, args: Option<&FluentArgs>) -> String {
        let Some(bundle) = self.bundles.get(&language) else {
            warn!("[i18n] 未找到语言: {:?}", language);
            return key.to_string();
        };

        let Some(msg) = bundle.get_message(key) else {
            warn!("[i18n] 未找到翻译键: {}", key);
            return key.to_string();
        };

        let Some(pattern) = msg.value() else {
            warn!("[i18n] 消息没有值: {}", key);
            return key.to_string();
        };

        let mut errors = vec![];
        let value = bundle.format_pattern(pattern, args, &mut errors);

        for error in errors {
            warn!("[i18n] 格式化错误: {:?}", error);
        }

        value.to_string()
    }

    /// 检查键是否存在
    pub fn has_message(&self, language: Language, key: &str) -> bool {
        if let Some(bundle) = self.bundles.get(&language) {
            bundle.get_message(key).is_some()
        } else {
            false
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
    /// Fluent 消息 ID（例如 "menu-title"）
    pub key: String,
    /// 可选的参数（用于参数化翻译）
    pub args: Option<Vec<(String, String)>>,
}

impl LocalizedText {
    /// 创建简单的本地化文本（无参数）
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            args: None,
        }
    }

    /// 创建带参数的本地化文本
    pub fn with_args(key: impl Into<String>, args: Vec<(&str, &str)>) -> Self {
        Self {
            key: key.into(),
            args: Some(
                args.into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect(),
            ),
        }
    }

    /// 格式化为实际文本
    pub fn format(&self, resources: &TranslationResources, language: Language) -> String {
        if let Some(args_vec) = &self.args {
            let mut fluent_args = FluentArgs::new();
            for (key, value) in args_vec {
                fluent_args.set(key, value.clone());
            }
            resources.t(language, &self.key, Some(&fluent_args))
        } else {
            resources.get(language, &self.key)
        }
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
            .init_asset::<FluentAsset>()
            .init_asset_loader::<FluentAssetLoader>()
            .add_systems(Startup, load_translation_handles)
            .add_systems(Update, (update_translation_data, update_localized_text).chain());

        info!("[i18n] Fluent 本地化系统已加载");
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
    info!("[i18n] 开始加载 Fluent 翻译文件...");

    // 加载简体中文翻译
    let zh_handles: Vec<Handle<FluentAsset>> = Language::SimplifiedChinese
        .translation_files()
        .iter()
        .map(|path| asset_server.load(path.clone()))
        .collect();
    translation_resources.set_handles(Language::SimplifiedChinese, zh_handles);

    // 加载英文翻译
    let en_handles: Vec<Handle<FluentAsset>> = Language::English
        .translation_files()
        .iter()
        .map(|path| asset_server.load(path.clone()))
        .collect();
    translation_resources.set_handles(Language::English, en_handles);

    info!(
        "[i18n] Fluent 文件句柄已创建: {} 个语言",
        2
    );
}

/// 更新翻译数据
///
/// 从 Assets 中提取已加载的 Fluent 资源并构建 Bundle
fn update_translation_data(
    fluent_assets: Res<Assets<FluentAsset>>,
    mut translation_resources: ResMut<TranslationResources>,
) {
    // 仅在翻译资源变化时更新
    if fluent_assets.is_changed() {
        let was_loaded = translation_resources.all_loaded();
        translation_resources.update_from_assets(&fluent_assets);
        let now_loaded = translation_resources.all_loaded();

        // 首次完成加载时记录日志
        if !was_loaded && now_loaded {
            info!("[i18n] 所有 Fluent 翻译文件加载完成");
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
        **text = localized.format(&*translation_resources, current_language.language);
    }

    if current_language.is_changed() {
        info!(
            "[i18n] 语言已切换到: {:?}",
            current_language.language
        );
    }
}

// ============================================================================
// 类型安全的翻译键常量（将在 .ftl 文件创建后更新）
// ============================================================================

/// 类型安全的翻译键常量
///
/// 使用这些常量可以避免拼写错误，并获得 IDE 自动补全
pub mod keys {
    // 主菜单
    pub const MENU_TITLE: &str = "menu-title";
    pub const MENU_NEW_GAME: &str = "menu-new-game";
    pub const MENU_RESUME: &str = "menu-resume";
    pub const MENU_SAVE_GAME: &str = "menu-save-game";
    pub const MENU_SETTINGS: &str = "menu-settings";
    pub const MENU_BACK_TO_MENU: &str = "menu-back-to-menu";
    pub const MENU_QUIT: &str = "menu-quit";

    // 游戏内
    pub const GAME_PAUSED: &str = "game-paused";
    pub const GAME_RESUME: &str = "game-resume";
    pub const GAME_BACK_TO_MENU: &str = "game-back-to-menu";

    // 设置
    pub const SETTINGS_TITLE: &str = "settings-title";
    pub const SETTINGS_APPEARANCE: &str = "settings-appearance";
    pub const SETTINGS_GRAPHICS: &str = "settings-graphics";
    pub const SETTINGS_AUDIO: &str = "settings-audio";
    pub const SETTINGS_CONTROLS: &str = "settings-controls";
    pub const SETTINGS_GAMEPLAY: &str = "settings-gameplay";
    pub const SETTINGS_BACK: &str = "settings-back";
    pub const SETTINGS_LANGUAGE: &str = "settings-language";
    pub const SETTINGS_LANGUAGE_CHINESE: &str = "settings-language-chinese";
    pub const SETTINGS_LANGUAGE_ENGLISH: &str = "settings-language-english";
    pub const SETTINGS_VOLUME_MASTER: &str = "settings-volume-master";
    pub const SETTINGS_VOLUME_MUSIC: &str = "settings-volume-music";
    pub const SETTINGS_VOLUME_SFX: &str = "settings-volume-sfx";
    pub const SETTINGS_RESOLUTION: &str = "settings-resolution";
    pub const SETTINGS_FULLSCREEN: &str = "settings-fullscreen";
    pub const SETTINGS_VSYNC: &str = "settings-vsync";
    pub const SETTINGS_DIFFICULTY: &str = "settings-difficulty";
    pub const SETTINGS_AUTO_SAVE: &str = "settings-auto-save";
    pub const SETTINGS_MOUSE_SENSITIVITY: &str = "settings-mouse-sensitivity";
    pub const SETTINGS_INVERT_Y: &str = "settings-invert-y";
    pub const SETTINGS_KEY_BINDINGS: &str = "settings-key-bindings";
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_toggle() {
        assert_eq!(
            Language::SimplifiedChinese.toggle(),
            Language::English
        );
        assert_eq!(
            Language::English.toggle(),
            Language::SimplifiedChinese
        );
    }

    #[test]
    fn test_language_dir_name() {
        assert_eq!(Language::SimplifiedChinese.dir_name(), "zh-Hans");
        assert_eq!(Language::English.dir_name(), "en");
    }

    #[test]
    fn test_localized_text_creation() {
        let text = LocalizedText::new("test-key");
        assert_eq!(text.key, "test-key");
        assert!(text.args.is_none());

        let text_with_args =
            LocalizedText::with_args("test-key", vec![("name", "Alice"), ("score", "100")]);
        assert_eq!(text_with_args.key, "test-key");
        assert!(text_with_args.args.is_some());
    }
}
