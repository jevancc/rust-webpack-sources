use std::ops::{Add, AddAssign};
use std::convert::From;
use std::mem;
use std::rc::Rc;
use std::slice;
use std::str;
use std::cmp::min;
use std::string::ToString;
use std::collections::VecDeque;
use super::string_slice::*;

#[derive(Debug, Clone)]
pub struct StringCat {
    // [(String, CharLen)]
    pub sub_strs: VecDeque<(StringSlice, Option<usize>)>,
    pub len: usize,
}

impl StringCat {
    fn new(sub_strs: VecDeque<(StringSlice, Option<usize>)>, len: usize) -> Self {
        StringCat {
            sub_strs,
            len
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn to_string_slice(&self) -> StringSlice {
        StringSlice::from(self.to_string())
    }

    #[inline]
    pub fn push_string_slice(&mut self, s: &StringSlice) {
        self.len += s.len();
        self.sub_strs.push_back((s.clone(), None));
    }

    #[inline]
    pub fn push_rc_string(&mut self, s: &Rc<String>) {
        let s = StringSlice::from(s);
        self.len += s.len();
        self.sub_strs.push_back((s, None));
    }

    pub fn split_at(mut self, mid: usize) -> (Self, Self) {
        let mid = min(mid, self.len);
        let mut bytes_last = mid;
        let mut split_pos = 0;
        for s in &self.sub_strs {
            if bytes_last > s.0.len() {
                bytes_last -= s.0.len();
                split_pos += 1;
            } else {
                break;
            }
        }

        let (mut left, mut right) = if split_pos == 0 {
            (VecDeque::new(), self.sub_strs)
        } else if split_pos == self.sub_strs.len() {
            (self.sub_strs, VecDeque::new())
        } else {
            let right = self.sub_strs.split_off(split_pos);
            (self.sub_strs, right)
        };
        if bytes_last != 0 {
            let split_s = right.pop_front().unwrap();
            let (left_char_len, right_char_len) = if let Some(char_len) = split_s.1 {
                if char_len == split_s.0.len() {
                    (Some(bytes_last), Some(split_s.0.len() - bytes_last))
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            };
            let (left_last, right_first) = split_s.0.split_at(bytes_last);
            left.push_back((left_last, left_char_len));
            right.push_front((right_first, right_char_len));
        }
        (Self::new(left, mid), Self::new(right, self.len - mid))
    }

    pub fn split_at_char(mut self, mid: usize) -> (Self, Self) {
        let mut chars_last = mid;
        let mut left_len = 0;
        let mut split_pos = 0;
        let mut split_char_bound = 0;
        let mut char_len = 0;

        'loop_sub_strs: for s in self.sub_strs.iter_mut() {
            if let Some(char_len) = s.1 {
                if char_len <= chars_last {
                    chars_last -= char_len;
                    left_len += s.0.len();
                    split_pos += 1;
                    continue;
                }
            }

            char_len = 0;
            for (p, _) in s.0.char_indices() {
                if chars_last == 0 {
                    split_char_bound = p;
                    break 'loop_sub_strs;
                }
                chars_last -= 1;
                char_len += 1;
            }

            left_len += s.0.len();
            split_pos += 1;
            s.1 = Some(char_len);
        }

        let (mut left, mut right) = if split_pos == 0 {
            (VecDeque::new(), self.sub_strs)
        } else if split_pos == self.sub_strs.len() {
            (self.sub_strs, VecDeque::new())
        } else {
            let right = self.sub_strs.split_off(split_pos);
            (self.sub_strs, right)
        };

        if split_char_bound != 0 {
            let split_s = right.pop_front().unwrap();
            let (left_last, right_first) = split_s.0.split_at(split_char_bound);
            left_len += split_char_bound;
            left.push_back((left_last, None));
            right.push_front((right_first, None));
        }
        (Self::new(left, left_len), Self::new(right, self.len - left_len))
    }

    pub fn cat(&mut self, mut s: StringCat) {
        self.len += s.len();
        self.sub_strs.append(&mut s.sub_strs);
    }
}

impl Default for StringCat {
    #[inline]
    fn default() -> StringCat {
        StringCat {
            sub_strs: VecDeque::new(),
            len: 0,
        }
    }
}

impl ToString for StringCat {
    fn to_string(&self) -> String {
        let mut s = String::with_capacity(self.len);
        for sub_s in &self.sub_strs {
            s.push_str(sub_s.0.as_str());
        }
        s
    }
}

impl From<StringSlice> for StringCat {
    fn from(s: StringSlice) -> StringCat {
        let s_len = s.len();
        let mut sub_strs = VecDeque::new();
        sub_strs.push_back((s, None));
        StringCat {
            sub_strs,
            len: s_len
        }
    }
}

impl<'a> From<&'a StringSlice> for StringCat {
    fn from(s: &StringSlice) -> StringCat {
        let s = s.clone();
        let s_len = s.len();
        let mut sub_strs = VecDeque::new();
        sub_strs.push_back((s, None));
        StringCat {
            sub_strs,
            len: s_len
        }
    }
}

impl From<Rc<String>> for StringCat {
    fn from(s: Rc<String>) -> StringCat {
        let s = StringSlice::from(s);
        let s_len = s.len();
        let mut sub_strs = VecDeque::new();
        sub_strs.push_back((s, None));
        StringCat {
            sub_strs,
            len: s_len
        }
    }
}

impl<'a> From<&'a Rc<String>> for StringCat {
    fn from(s: &Rc<String>) -> StringCat {
        let s = StringSlice::from(s);
        let s_len = s.len();
        let mut sub_strs = VecDeque::new();
        sub_strs.push_back((s, None));
        StringCat {
            sub_strs,
            len: s_len
        }
    }
}

pub trait PushStringCat {
    fn push_string_cat(&mut self, str_cat: &StringCat);
}

impl PushStringCat for String {
    fn push_string_cat(&mut self, str_cat: &StringCat) {
        self.reserve(str_cat.len());
        for sub_s in &str_cat.sub_strs {
            self.push_str(sub_s.0.as_str());
        }
    }
}
