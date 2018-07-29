use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct StringCache {
    set: HashMap<Rc<String>, usize>,
    array: Vec<Rc<String>>,
}

impl StringCache {
    pub fn new() -> Self {
        StringCache::default()
    }

    pub fn add(&mut self, s: &str) -> i32 {
        let s = Rc::new(s.to_string());
        let mut idx = self.set.len();
        if self.set.contains_key(&s) {
            idx = *self.set.get(&s).unwrap()
        } else {
            self.array.push(s.clone());
            self.set.insert(s, idx);
        }
        idx as i32
    }
}
