//re-export proc macro crate
pub use labyrinth_macros::*;

/// A helper decryption function meant to decrypt encrypted strings at runtime
///
/// # Parameters
/// - `input`: The encrypted string literal
///
pub fn decrypt_string(encrypted: &str) -> String {
    let key = std::env::var("CRYPTIFY_KEY").unwrap_or_else(|_| "xnasff3wcedj".to_string());
    decrypt_string_with_key(encrypted, &key)
}

pub fn decrypt_string_with_key(encrypted: &str, key: &str) -> String {
    let key = if key.is_empty() { "xnasff3wcedj" } else { key };
    let bytes: Vec<u8> = encrypted
        .chars()
        .map(|character| u8::try_from(character as u32).expect("invalid encrypted string"))
        .zip(key.bytes().cycle())
        .map(|(character, key_byte)| character ^ key_byte)
        .collect();
    String::from_utf8(bytes).expect("invalid decrypted string")
}
