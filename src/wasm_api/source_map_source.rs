use source::SourceTrait;
use source_map_source::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_api::{StringVec, _MSourceNode, _SourceListMap};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct _SourceMapSource {
    val: Rc<RefCell<SourceMapSource>>,
}

#[wasm_bindgen]
impl _SourceMapSource {
    pub fn _new_string_string_map(
        value: String,
        name: String,
        map_sources: StringVec,
        map_sources_content: StringVec,
        map_mappings: String,
        map_names: StringVec,
    ) -> _SourceMapSource {
        _SourceMapSource {
            val: Rc::new(RefCell::new(SourceMapSource::new(
                value,
                name,
                map_sources.get_raw(),
                map_sources_content.get_raw(),
                map_mappings,
                map_names.get_raw(),
            ))),
        }
    }

    pub fn _source(&mut self) -> String {
        self.val.borrow_mut().source()
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

impl _SourceMapSource {
    pub fn get_raw(&self) -> Rc<RefCell<SourceMapSource>> {
        Rc::clone(&self.val)
    }
}
