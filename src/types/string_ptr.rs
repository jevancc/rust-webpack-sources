use std::rc::Rc;
use std::slice;
use std::str;

#[derive(Debug)]
pub enum _StringPtr {
    Str(String),
    Ptr(Rc<String>),
    UnsafePtr((*const u8, usize)),
}

impl Clone for _StringPtr {
    fn clone(&self) -> _StringPtr {
        match self {
            _StringPtr::Str(s) => _StringPtr::Ptr(Rc::new(s.clone())),
            _StringPtr::Ptr(p) => _StringPtr::Ptr(p.clone()),
            _StringPtr::UnsafePtr(up) => _StringPtr::UnsafePtr(up.clone()),
        }
    }
}

impl _StringPtr {
    pub fn to_string(&self) -> String {
        match self {
            _StringPtr::Str(s) => s.clone(),
            _StringPtr::Ptr(p) => p.as_str().to_string(),
            _StringPtr::UnsafePtr((p, l)) => unsafe {
                String::from_utf8(slice::from_raw_parts(*p, *l).to_vec()).unwrap()
            }
        }
    }

    pub fn into_string(self) -> String {
        match self {
            _StringPtr::Str(s) => s,
            _StringPtr::Ptr(p) => p.as_str().to_string(),
            _StringPtr::UnsafePtr((p, l)) => unsafe {
                String::from_utf8(slice::from_raw_parts(p, l).to_vec()).unwrap()
            }
        }
    }

    pub fn into_ptr(self) -> Rc<String> {
        match self {
            _StringPtr::Str(s) => Rc::new(s),
            _StringPtr::Ptr(p) => p,
            _StringPtr::UnsafePtr((p, l)) => unsafe {
                Rc::new(String::from_utf8(slice::from_raw_parts(p, l).to_vec()).unwrap())
            }
        }
    }

    pub fn as_ref(&self) -> &str {
        match self {
            _StringPtr::Str(s) => s.as_str(),
            _StringPtr::Ptr(p) => p.as_str(),
            _StringPtr::UnsafePtr((p, l)) => unsafe {
                 str::from_utf8(
                     slice::from_raw_parts(*p, *l)
                ).unwrap()
            }
        }
    }
}


pub trait ToStringPtr {
    fn to_string_ptr(self) -> _StringPtr;
}

pub trait AsUnsafePtr {
    fn as_unsafe_ptr(&self) -> _StringPtr;
}

impl ToStringPtr for String {
    fn to_string_ptr(self) -> _StringPtr {
        _StringPtr::Str(self)
    }
}
impl AsUnsafePtr for String {
    fn as_unsafe_ptr(&self) -> _StringPtr {
        _StringPtr::UnsafePtr((
            self.as_str().as_ptr(),
            self.len(),
        ))
    }
}

impl ToStringPtr for Rc<String> {
    fn to_string_ptr(self) -> _StringPtr {
        _StringPtr::Ptr(self)
    }
}
impl AsUnsafePtr for Rc<String> {
    fn as_unsafe_ptr(&self) -> _StringPtr {
        _StringPtr::UnsafePtr((
            self.as_str().as_ptr(),
            self.len(),
        ))
    }
}

impl AsUnsafePtr for str {
    fn as_unsafe_ptr(&self) -> _StringPtr {
        _StringPtr::UnsafePtr((
            self.as_ptr(),
            self.len(),
        ))
    }
}
