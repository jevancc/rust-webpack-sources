use raw_source::*;
use source::SourceTrait;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_api::{_MSourceNode, _SourceListMap};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct _RawSource {
    val: Rc<RefCell<RawSource>>,
}

#[wasm_bindgen]
impl _RawSource {
    pub fn _new_string(value: String) -> _RawSource {
        _RawSource {
            val: Rc::new(RefCell::new(RawSource::new(value))),
        }
    }

    pub fn _source(&mut self) -> String {
        (*self.val.borrow_mut().source()).clone()
    }

    pub fn _size(&mut self) -> u32 {
        self.val.borrow_mut().size() as u32
    }

    pub fn _list_map_bool_bool(&mut self, columns: bool, module: bool) -> _SourceListMap {
        _SourceListMap::new(self.val.borrow_mut().list_map(columns, module))
    }

    pub fn _node_bool_bool(&mut self, columns: bool, module: bool) -> _MSourceNode {
        _MSourceNode::new(self.val.borrow_mut().node(columns, module))
    }
}

impl _RawSource {
    pub fn get_raw(&self) -> Rc<RefCell<RawSource>> {
        Rc::clone(&self.val)
    }
}
