use original_source::*;
use source::SourceTrait;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_api::{_MSourceNode, _SourceListMap};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct _OriginalSource {
    val: Rc<RefCell<OriginalSource>>,
}

#[wasm_bindgen]
impl _OriginalSource {
    pub fn _new_string_sidx_sidx(
        source_code: String,
        source_code_idx: i32,
        name: i32,
    ) -> _OriginalSource {
        _OriginalSource {
            val: Rc::new(RefCell::new(OriginalSource::new(
                source_code,
                source_code_idx,
                name,
            ))),
        }
    }

    pub fn _source(&mut self) -> String {
        self.val.borrow_mut().source().to_string()
    }

    pub fn _size(&mut self) -> u32 {
        self.val.borrow_mut().size() as u32
    }

    pub fn _name(&mut self) -> i32 {
        self.val.borrow_mut().name
    }

    pub fn _list_map_bool_bool(&mut self, columns: bool, module: bool) -> _SourceListMap {
        _SourceListMap::new(self.val.borrow_mut().list_map(columns, module))
    }

    pub fn _node_bool_bool(&mut self, columns: bool, module: bool) -> _MSourceNode {
        _MSourceNode::new(self.val.borrow_mut().node(columns, module))
    }
}

impl _OriginalSource {
    pub fn get_raw(&self) -> Rc<RefCell<OriginalSource>> {
        Rc::clone(&self.val)
    }
}
