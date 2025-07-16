pub fn normalize_type(s: &str) -> String {
    s.chars()
        .filter(|&c| c != '\u{fe0f}' && c != '\u{fe0e}')
        .collect()
}
