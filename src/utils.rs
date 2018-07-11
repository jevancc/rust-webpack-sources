#[inline]
pub fn split_str(s: &str, pos: i32, single_byte_char_only: bool) -> (&str, &str, bool, bool) {
    let pos: usize = if pos < 0 {
        0
    } else if pos as usize >= s.len() {
        s.len()
    } else {
        pos as usize
    };

    let byte_pos = if single_byte_char_only {
        pos
    } else {
        s.char_indices().skip(pos).next().unwrap().0
    };
    let (ls, rs) = s.split_at(byte_pos);
    (ls, rs, byte_pos == pos, single_byte_char_only)
}
