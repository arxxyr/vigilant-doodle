//! 加密/解密模块
//!
//! 用于保护游戏资源和存档文件
//!
//! 使用 AES-256-GCM 军用级加密

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use serde::{Deserialize, Serialize};
use std::io::{self, ErrorKind};

// ============================================================================
// 常量定义
// ============================================================================

/// 加密密钥（商业发布时应修改此密钥）
/// AES-256 需要 32 字节（256 位）密钥
const ENCRYPTION_KEY: &[u8; 32] = b"VigilantDoodle_AES256_Key_2025!!";

/// 文件魔数（用于验证文件格式）
const MAGIC_NUMBER: u32 = 0x56444232; // "VDB2" (Vigilant Doodle Binary v2)

/// Nonce 长度（AES-GCM 推荐 12 字节）
const NONCE_SIZE: usize = 12;

// ============================================================================
// 数据结构
// ============================================================================

/// 加密文件格式（AES-256-GCM）
#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode)]
struct EncryptedFile {
    /// 魔数（用于验证文件格式）
    magic: u32,
    /// 版本号（当前为 2，使用 AES-256-GCM）
    version: u32,
    /// Nonce（每次加密都不同，12 字节）
    nonce: [u8; NONCE_SIZE],
    /// 加密后的数据（包含认证标签）
    data: Vec<u8>,
}

// ============================================================================
// 公共接口
// ============================================================================

/// 加密数据（使用 AES-256-GCM）
///
/// # 流程
/// 1. 生成随机 Nonce（12 字节）
/// 2. 使用 AES-256-GCM 加密数据
/// 3. 添加魔数、版本号和 Nonce
/// 4. 序列化为二进制格式
///
/// # 参数
/// - `data`: 原始数据（通常是序列化后的结构体）
///
/// # 返回
/// 加密后的二进制数据
///
/// # 安全性
/// - 使用 AES-256-GCM 军用级加密
/// - 自动完整性验证（AEAD）
/// - 每次加密使用不同的 Nonce
pub fn encrypt(data: &[u8]) -> Result<Vec<u8>, io::Error> {
    // 生成随机 Nonce
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    use aes_gcm::aead::rand_core::RngCore;
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from(nonce_bytes);

    // 创建 AES-256-GCM 加密器
    let cipher = Aes256Gcm::new_from_slice(ENCRYPTION_KEY)
        .map_err(|e| io::Error::new(ErrorKind::InvalidInput, format!("密钥错误: {}", e)))?;

    // 加密数据
    let encrypted_data = cipher
        .encrypt(&nonce, data)
        .map_err(|e| io::Error::new(ErrorKind::InvalidData, format!("加密失败: {}", e)))?;

    // 构建加密文件
    let encrypted_file = EncryptedFile {
        magic: MAGIC_NUMBER,
        version: 2,
        nonce: nonce_bytes,
        data: encrypted_data,
    };

    // 序列化为二进制（bincode 2.0 API）
    bincode::encode_to_vec(&encrypted_file, bincode::config::standard())
        .map_err(|e| io::Error::new(ErrorKind::InvalidData, e.to_string()))
}

/// 解密数据（使用 AES-256-GCM）
///
/// # 流程
/// 1. 反序列化二进制数据
/// 2. 验证魔数和版本号
/// 3. 使用 AES-256-GCM 解密数据（自动验证完整性）
///
/// # 参数
/// - `encrypted_data`: 加密的二进制数据
///
/// # 返回
/// 解密后的原始数据
///
/// # 安全性
/// - AES-GCM 自动验证完整性
/// - 如果文件被篡改，解密会失败
/// - 无需额外的校验和
pub fn decrypt(encrypted_data: &[u8]) -> Result<Vec<u8>, io::Error> {
    // 反序列化（bincode 2.0 API）
    let (encrypted_file, _): (EncryptedFile, _) =
        bincode::decode_from_slice(encrypted_data, bincode::config::standard())
            .map_err(|e| io::Error::new(ErrorKind::InvalidData, format!("解密失败: {}", e)))?;

    // 验证魔数
    if encrypted_file.magic != MAGIC_NUMBER {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            format!(
                "文件格式错误（魔数不匹配）: 期望 0x{:08X}, 实际 0x{:08X}",
                MAGIC_NUMBER, encrypted_file.magic
            ),
        ));
    }

    // 验证版本（当前为版本 2：AES-256-GCM）
    if encrypted_file.version != 2 {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            format!(
                "不支持的文件版本: {}（当前支持版本 2）",
                encrypted_file.version
            ),
        ));
    }

    // 创建 AES-256-GCM 解密器
    let cipher = Aes256Gcm::new_from_slice(ENCRYPTION_KEY)
        .map_err(|e| io::Error::new(ErrorKind::InvalidInput, format!("密钥错误: {}", e)))?;

    // 解密数据（自动验证完整性）
    let nonce = Nonce::from(encrypted_file.nonce);
    let decrypted_data = cipher
        .decrypt(&nonce, encrypted_file.data.as_ref())
        .map_err(|e| {
            io::Error::new(
                ErrorKind::InvalidData,
                format!("解密失败（文件可能已损坏或被篡改）: {}", e),
            )
        })?;

    Ok(decrypted_data)
}

// ============================================================================
// 内部实现
// ============================================================================

// AES-256-GCM 加密不需要额外的内部函数
// 所有加密逻辑已在 encrypt() 和 decrypt() 中实现

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let original_data = b"This is sensitive game data!";

        // 加密
        let encrypted = encrypt(original_data).unwrap();
        assert_ne!(encrypted.as_slice(), original_data);

        // 解密
        let decrypted = decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, original_data);
    }

    #[test]
    fn test_encrypt_different_nonces() {
        let data = b"Same data";

        // 加密两次
        let encrypted1 = encrypt(data).unwrap();
        let encrypted2 = encrypt(data).unwrap();

        // 由于 Nonce 不同，加密结果应该不同
        assert_ne!(encrypted1, encrypted2);

        // 但解密结果应该相同
        let decrypted1 = decrypt(&encrypted1).unwrap();
        let decrypted2 = decrypt(&encrypted2).unwrap();
        assert_eq!(decrypted1, data);
        assert_eq!(decrypted2, data);
    }

    #[test]
    fn test_decrypt_invalid_magic() {
        // 构造一个错误的魔数
        let invalid_file = EncryptedFile {
            magic: 0x12345678,
            version: 2,
            nonce: [0u8; NONCE_SIZE],
            data: vec![],
        };
        let invalid_data = bincode::serialize(&invalid_file).unwrap();

        // 应该返回错误
        let result = decrypt(&invalid_data);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("魔数不匹配"));
    }

    #[test]
    fn test_decrypt_invalid_version() {
        // 构造错误的版本号
        let invalid_file = EncryptedFile {
            magic: MAGIC_NUMBER,
            version: 999,
            nonce: [0u8; NONCE_SIZE],
            data: vec![],
        };
        let invalid_data = bincode::serialize(&invalid_file).unwrap();

        // 应该返回错误
        let result = decrypt(&invalid_data);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("不支持的文件版本"));
    }

    #[test]
    fn test_decrypt_tampered_data() {
        let original_data = b"Original data";
        let mut encrypted = encrypt(original_data).unwrap();

        // 篡改加密数据的最后一个字节
        let len = encrypted.len();
        encrypted[len - 1] ^= 0xFF;

        // 解密应该失败（AES-GCM 完整性验证）
        let result = decrypt(&encrypted);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("文件可能已损坏或被篡改"));
    }

    #[test]
    fn test_large_data() {
        // 测试大数据加密
        let large_data = vec![0xAB; 1024 * 100]; // 100 KB

        let encrypted = encrypt(&large_data).unwrap();
        let decrypted = decrypt(&encrypted).unwrap();

        assert_eq!(decrypted, large_data);
    }
}
