use types::string_slice::*;

pub fn split_str(
    s: &str,
    pos: i32,
    single_byte_char_only: bool,
) -> Result<(&str, &str, bool, bool), (i32, &str, bool)> {
    if pos <= 0 {
        Ok(("", s, true, single_byte_char_only))
    } else {
        let mut pos = pos as usize;
        let s_bytes = s.len();
        let mut split_pos: usize = 0;

        if single_byte_char_only {
            if pos < s_bytes {
                split_pos = pos as usize;
            } else {
                return Err(((pos - s_bytes) as i32, s, single_byte_char_only));
            }
        } else {
            for (bp, c) in s.char_indices() {
                pos -= 1;
                if pos == 0 {
                    split_pos = bp + c.len_utf8();
                    break;
                }
            }
            if pos > 0 {
                return Err((pos as i32, s, single_byte_char_only));
            }
        }

        let (ls, rs) = s.split_at(split_pos);
        Ok((ls, rs, split_pos == pos as usize, single_byte_char_only))
    }
}

pub fn split_string_slice(
    s: StringSlice,
    pos: i32,
    single_byte_char_only: bool,
) -> Result<(StringSlice, StringSlice, bool, bool), (i32, StringSlice, bool)> {
    if pos <= 0 {
        Ok((StringSlice::new(), s, true, single_byte_char_only))
    } else {
        let mut pos = pos as usize;
        let s_bytes = s.len();
        let mut split_pos: usize = 0;

        if single_byte_char_only {
            if pos < s_bytes {
                split_pos = pos as usize;
            } else {
                return Err(((pos - s_bytes) as i32, s, single_byte_char_only));
            }
        } else {
            for (bp, c) in s.char_indices() {
                pos -= 1;
                if pos == 0 {
                    split_pos = bp + c.len_utf8();
                    break;
                }
            }
            if pos > 0 {
                return Err((pos as i32, s, single_byte_char_only));
            }
        }

        let (ls, rs) = s.split_at(split_pos);
        Ok((ls, rs, split_pos == pos as usize, single_byte_char_only))
    }
}
