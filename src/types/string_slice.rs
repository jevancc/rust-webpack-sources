// use std::ops::{Add, AddAssign, Deref};
use std::cmp::min;
use std::convert::From;
use std::hash;
use std::mem;
use std::ops::Deref;
use std::rc::Rc;
use std::slice;
use std::str;
use std::string::ToString;

#[derive(Debug, Clone)]
pub struct StringSlice(*const u8, usize, Rc<String>);
impl StringSlice {
    pub fn new() -> Self {
        let s = Rc::new(String::new());
        StringSlice(s.as_str().as_ptr(), 0, s)
    }

    #[inline]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }

    pub fn substr(&self, start: usize, stop: usize) -> Self {
        let start = min(start, self.1);
        let stop = min(stop, self.1);
        StringSlice(
            unsafe { self.0.offset(start as isize) },
            stop - start,
            self.2.clone(),
        )
    }

    pub fn split_at(self, mid: usize) -> (Self, Self) {
        let mid = min(mid, self.1);
        unsafe {
            (
                StringSlice(self.0, mid, self.2.clone()),
                StringSlice(self.0.offset(mid as isize), self.1 - mid, self.2),
            )
        }
    }

    pub fn split(&self, pat: u8) -> Split {
        Split::new(self.clone(), pat, false)
    }

    pub fn split_keep_seperator(&self, pat: u8) -> Split {
        Split::new(self.clone(), pat, true)
    }

    pub fn offset(self, l: isize) -> Self {
        unsafe { StringSlice(self.0.offset(l), (self.1 as isize - l) as usize, self.2) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.1
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(self.0, self.1)) }
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.0, self.1) }
    }

    pub fn into_string(self) -> String {
        if self.0 == self.2.as_str().as_ptr() && self.1 == self.2.len() {
            Rc::try_unwrap(self.2).unwrap_or_else(|s| (*s).clone())
        } else {
            unsafe { str::from_utf8_unchecked(slice::from_raw_parts(self.0, self.1)).to_string() }
        }
    }

    pub fn into_rc(self) -> Rc<String> {
        if self.0 == self.2.as_str().as_ptr() && self.1 == self.2.len() {
            self.2.clone()
        } else {
            unsafe {
                Rc::new(str::from_utf8_unchecked(slice::from_raw_parts(self.0, self.1)).to_string())
            }
        }
    }
}

impl Default for StringSlice {
    #[inline]
    fn default() -> StringSlice {
        StringSlice::new()
    }
}

impl AsRef<str> for StringSlice {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl ToString for StringSlice {
    #[inline]
    fn to_string(&self) -> String {
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(self.0, self.1)).to_string() }
    }
}

impl PartialEq for StringSlice {
    #[inline]
    fn eq(&self, other: &StringSlice) -> bool {
        self.as_bytes() == other.as_bytes()
    }
    #[inline]
    fn ne(&self, other: &StringSlice) -> bool {
        !(*self).eq(other)
    }
}

impl Eq for StringSlice {}

impl<'a> From<&'a str> for StringSlice {
    fn from(s: &str) -> StringSlice {
        let s: Rc<String> = Rc::new(s.to_string());
        StringSlice(s.as_str().as_ptr(), s.len(), s)
    }
}

impl From<String> for StringSlice {
    fn from(s: String) -> StringSlice {
        let s: Rc<String> = Rc::new(s);
        StringSlice(s.as_str().as_ptr(), s.len(), s)
    }
}

impl From<Rc<String>> for StringSlice {
    fn from(s: Rc<String>) -> StringSlice {
        StringSlice(s.as_str().as_ptr(), s.len(), s)
    }
}

impl<'a> From<&'a Rc<String>> for StringSlice {
    fn from(s: &Rc<String>) -> StringSlice {
        StringSlice(s.as_str().as_ptr(), s.len(), s.clone())
    }
}

impl Deref for StringSlice {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl hash::Hash for StringSlice {
    #[inline]
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        self.as_bytes().hash(hasher)
    }
}

pub struct Split {
    pub is_next: bool,
    pub rest: Option<StringSlice>,
    pat: u8,
    keep_seperator: bool,
}

impl Split {
    pub fn new(s: StringSlice, pat: u8, keep_seperator: bool) -> Self {
        Split {
            rest: Some(s),
            pat,
            keep_seperator,
            is_next: true,
        }
    }

    pub fn rest(&mut self) -> Option<StringSlice> {
        let mut rest: Option<StringSlice> = None;
        mem::swap(&mut rest, &mut self.rest);
        self.is_next = false;
        rest
    }
}

impl Iterator for Split {
    type Item = StringSlice;
    fn next(&mut self) -> Option<StringSlice> {
        if self.is_next {
            let mut rest: Option<StringSlice> = None;
            mem::swap(&mut self.rest, &mut rest);
            let s = rest.unwrap();
            if let Some(pos) = s.as_bytes().iter().position(|&b| b == self.pat) {
                if self.keep_seperator {
                    let (s, r) = s.split_at(pos + 1);
                    self.rest = Some(r);
                    Some(s)
                } else {
                    let (s, sep_r) = s.split_at(pos);
                    self.rest = Some(sep_r.offset(1 as isize));
                    Some(s)
                }
            } else {
                self.is_next = false;
                self.rest = None;
                Some(s)
            }
        } else {
            None
        }
    }
}
