/// 名称合法性检测
/// Name validity check
pub fn is_valid(name: &str) -> bool {
    // 名称合法字符: 字母,数字,下划线,连字符,点,波浪线
    // Name valid characters: letters, numbers, underscores, hyphens, dots, tildes
    let chars = name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c.eq(&'_') || c.eq(&'-') || c.eq(&'.') || c.eq(&'~'));
    // 名称长度需要在 1 到 16 之间
    // Name length must be between 1 and 16
    let len = !name.is_empty() && name.len() <= 16;
    chars && len
}
