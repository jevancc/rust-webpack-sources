use line_to_line_mapped_source::*;
use source::SourceTrait;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_api::{_MSourceNode, _SourceListMap};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct _LineToLineMappedSource {
    val: Rc<RefCell<LineToLineMappedSource>>,
}

#[wasm_bindgen]
impl _LineToLineMappedSource {
    pub fn _new_string_sidx_sidx(
        value: String,
        name: i32,
        original_source: i32,
    ) -> _LineToLineMappedSource {
        _LineToLineMappedSource {
            val: Rc::new(RefCell::new(LineToLineMappedSource::new(
                value,
                name,
                original_source,
            ))),
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

impl _LineToLineMappedSource {
    pub fn get_raw(&self) -> Rc<RefCell<LineToLineMappedSource>> {
        Rc::clone(&self.val)
    }
}
