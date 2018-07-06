#[inline]
pub fn number_of_lines(s: &str) -> usize {
    s.split('\n').count().saturating_sub(1)
}

#[inline]
pub fn get_unfinished_lines(s: &str) -> usize {
    let s_len = s.len();
    s.rfind('\n').map_or(s_len, |res| s_len - res - 1)
}
