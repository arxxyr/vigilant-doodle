//! 翻译文件打包工具
//!
//! 将 JSON 翻译文件转换为加密的二进制格式（AES-256-GCM）
//!
//! 用法：
//!   cargo run --bin pack_translations

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// ============================================================================
// 数据结构（与 localization.rs 保持一致）
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
struct MenuTranslations {
    title: String,
    new_game: String,
    resume: String,
    save_game: String,
    load_game: String,
    settings: String,
    back_to_menu: String,
    quit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
struct SettingsTranslations {
    title: String,
    graphics: String,
    audio: String,
    controls: String,
    language: String,
    back: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
struct GameplayTranslations {
    paused: String,
    game_over: String,
    victory: String,
    score: String,
    health: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
struct Translations {
    menu: MenuTranslations,
    settings: SettingsTranslations,
    gameplay: GameplayTranslations,
}

// ============================================================================
// 加密函数（与 crypto.rs 保持一致）
// ============================================================================

const ENCRYPTION_KEY: &[u8; 32] = b"VigilantDoodle_AES256_Key_2025!!";
const MAGIC_NUMBER: u32 = 0x56444232; // VDB2
const NONCE_SIZE: usize = 12;

#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode)]
struct EncryptedFile {
    magic: u32,
    version: u32,
    nonce: [u8; NONCE_SIZE],
    data: Vec<u8>,
}

fn encrypt(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // 生成随机 Nonce
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    use aes_gcm::aead::rand_core::RngCore;
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 创建 AES-256-GCM 加密器
    let cipher = Aes256Gcm::new_from_slice(ENCRYPTION_KEY)
        .map_err(|e| format!("密钥错误: {:?}", e))?;

    // 加密数据
    let encrypted_data = cipher
        .encrypt(nonce, data)
        .map_err(|e| format!("加密失败: {:?}", e))?;

    // 构建加密文件
    let encrypted_file = EncryptedFile {
        magic: MAGIC_NUMBER,
        version: 2,
        nonce: nonce_bytes,
        data: encrypted_data,
    };

    // 使用 bincode 2.0 API 序列化
    Ok(bincode::encode_to_vec(&encrypted_file, bincode::config::standard())?)
}

// ============================================================================
// 主函数
// ============================================================================

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 翻译文件打包工具（AES-256-GCM）===\n");

    // 定义输入输出路径
    let languages = vec![
        ("zh_CN", "assets/localization/zh_CN.json"),
        ("en_US", "assets/localization/en_US.json"),
    ];

    for (lang_code, json_path) in languages {
        println!("处理 {} ...", lang_code);

        // 检查文件是否存在
        if !Path::new(json_path).exists() {
            println!("  ⚠ 跳过（文件不存在）: {}", json_path);
            continue;
        }

        // 读取 JSON
        let json_content = fs::read_to_string(json_path)?;
        println!("  ✓ 读取 JSON: {} 字节", json_content.len());

        // 解析 JSON（验证格式）
        let translations: Translations = serde_json::from_str(&json_content)?;
        println!("  ✓ 验证 JSON 格式");

        // 序列化为二进制（使用 bincode 2.0 API）
        let binary_data = bincode::encode_to_vec(&translations, bincode::config::standard())?;
        println!("  ✓ 序列化为二进制: {} 字节", binary_data.len());

        // 加密
        let encrypted_data = encrypt(&binary_data)?;
        println!("  ✓ 加密: {} 字节", encrypted_data.len());

        // 计算压缩率
        let encrypted_size = encrypted_data.len();
        let ratio = (encrypted_size as f64 / json_content.len() as f64) * 100.0;

        // 输出路径
        let output_path = format!("assets/localization/{}.lang", lang_code);
        fs::write(&output_path, encrypted_data)?;
        println!("  ✓ 保存到: {}", output_path);

        // 显示压缩率
        println!(
            "  压缩率: {:.1}% (JSON: {} → 加密: {} 字节)\n",
            ratio,
            json_content.len(),
            encrypted_size
        );
    }

    println!("=== 完成 ===");
    println!("\n发布提示：");
    println!("1. 将 assets/localization/*.lang 文件包含在发布包中");
    println!("2. 不要包含 *.json 源文件（防止泄露）");
    println!("3. 修改代码从 .lang 文件加载翻译");

    Ok(())
}
