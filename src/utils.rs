use std::cmp;

#[inline]
pub fn split_str(s: &str, pos: i32, single_byte_char_only: bool) -> (&str, &str, bool, bool) {
    if pos <= 0 {
        ("", s, true, single_byte_char_only)
    } else {
        let s_bytes = s.len();
        let split_pos = if single_byte_char_only {
            cmp::min(pos as usize, s_bytes)
        } else {
            s.char_indices()
            .skip(pos as usize)
            .next()
            .map_or(s_bytes, |(p, _)| p)
        };

        let (ls, rs) = s.split_at(split_pos);
        (ls, rs, split_pos == pos as usize, single_byte_char_only)
    }
}
