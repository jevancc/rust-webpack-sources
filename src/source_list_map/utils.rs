use bytecount;
use types::string_slice::*;

#[inline]
pub fn number_of_lines(s: &str) -> usize {
    bytecount::count(s.as_bytes(), b'\n')
}

#[inline]
pub fn get_unfinished_lines(strs: &[StringSlice]) -> usize {
    let mut suffix_len = 0;
    for s in strs.iter().rev() {
        match s.rfind('\n') {
            Some(res) => {
                return suffix_len + s.len() - res - 1;
            }
            None => suffix_len += s.len(),
        }
    }
    suffix_len
}

pub trait IntoString {
    fn into_string(self, len: usize) -> String;
}

impl IntoString for Vec<StringSlice> {
    fn into_string(mut self, len: usize) -> String {
        if self.len() == 1 {
            self.pop().unwrap().into_string()
        } else {
            let mut combine = String::with_capacity(len);
            for s in self {
                combine.push_str(&s);
            }
            combine
        }
    }
}
