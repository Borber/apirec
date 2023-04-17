/// 名称合法性检测
/// Name validity check
pub fn is_valid(name: &str) -> bool {
    // 名称需要是字母数字或下划线
    // Name must be alphanumeric or underscore
    let chars = name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c.eq(&'_'));
    // 名称长度需要在 1 到 32 之间
    // Name length must be between 1 and 32
    let len = !name.is_empty() && name.len() <= 32;
    chars && len
}
