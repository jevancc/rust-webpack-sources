use bytecount;

#[inline]
pub fn number_of_lines(s: &str) -> usize {
    bytecount::count(s.as_bytes(), b'\n')
}

#[inline]
pub fn get_unfinished_lines(s: &str) -> usize {
    let s_len = s.len();
    s.rfind('\n').map_or(s_len, |res| s_len - res - 1)
}
