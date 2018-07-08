#[inline]
pub fn split_str(s: &str, pos: i32, s_len: Option<usize>) -> (&str, &str, usize, usize) {
    let s_len = s_len.map_or(s.chars().count(), |l| l);

    if pos <= 0 {
        ("", s, 0, s_len)
    } else if pos >= s_len as i32 {
        (s, "", s_len, 0)
    } else {
        let byte_pos = s.char_indices().skip(pos as usize).next().unwrap().0;
        let (ls, rs) = s.split_at(byte_pos);
        (ls, rs, pos as usize, s_len - pos as usize)
    }
}
