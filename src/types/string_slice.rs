// use std::ops::{Add, AddAssign, Deref};
use std::ops::Deref;
use std::string::ToString;
use std::cmp::min;
use std::rc::Rc;
use std::slice;
use std::str;
use std::convert::From;
use std::hash;


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
        StringSlice(unsafe {
            self.0.offset(start as isize)
        }, stop - start, self.2.clone())
    }

    pub fn split_at(self, mid: usize) -> (Self, Self) {
        let mid = min(mid, self.1);
        unsafe {(
            StringSlice(self.0, mid, self.2.clone()),
            StringSlice(self.0.offset(mid as isize), self.1 - mid, self.2.clone())
        )}
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.1
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(self.0, self.1))
        }
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.0, self.1)
        }
    }

    pub fn into_string(self) -> String {
        if self.0 == self.2.as_str().as_ptr() && self.1 == self.2.len() {
            Rc::try_unwrap(self.2).unwrap_or_else(|s| (*s).clone())
        } else {
            unsafe {
                str::from_utf8_unchecked(slice::from_raw_parts(self.0, self.1)).to_string()
            }
        }
    }

    pub fn into_rc(self) -> Rc<String> {
        if self.0 == self.2.as_str().as_ptr() && self.1 == self.2.len() {
            self.2.clone()
        } else {
            unsafe {
                Rc::new(
                    str::from_utf8_unchecked(slice::from_raw_parts(self.0, self.1)).to_string()
                )
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
        unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(self.0, self.1)).to_string()
        }
    }
}

impl PartialEq for StringSlice {
    #[inline]
    fn eq(&self, other: &StringSlice) -> bool {
        self.as_bytes() == other.as_bytes()
    }
    #[inline]
    fn ne(&self, other: &StringSlice) -> bool { !(*self).eq(other) }
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

// impl<'a> Add<&'a StringSlice> for String {
//     type Output = String;
//
//     #[inline]
//     fn add(mut self, other: &StringSlice) -> String {
//         self.push_str(other.as_str());
//         self
//     }
// }
//
// impl<'a> AddAssign<&'a StringSlice> for String {
//     #[inline]
//     fn add_assign(&mut self, other: &StringSlice) {
//         self.push_str(other.as_str());
//     }
// }
