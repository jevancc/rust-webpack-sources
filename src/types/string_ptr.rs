use std::rc::Rc;

#[derive(Clone)]
pub enum StringPtr {
    Str(String),
    Ptr(Rc<String>),
}

impl StringPtr {
    pub fn to_ptr(self) -> Rc<String> {
        match self {
            StringPtr::Str(s) => Rc::new(s),
            StringPtr::Ptr(p) => p,
        }
    }

    pub fn get(&self) -> &str {
        match self {
            StringPtr::Str(s) => &s,
            StringPtr::Ptr(p) => p,
        }
    }
}
