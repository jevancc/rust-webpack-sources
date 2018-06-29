use raw_source::*;
use wasm_bindgen::prelude::*;
use wasm_api::{_MSourceNode, _SourceListMap};
use source::SourceTrait;

#[wasm_bindgen]
pub struct _RawSource {
    val: Box<RawSource>,
}

#[wasm_bindgen]
impl _RawSource {
    pub fn _new_string(value: String) -> _RawSource {
        _RawSource {
            val: Box::new(RawSource::new(value)),
        }
    }

    pub fn _source(&mut self) -> String {
        self.val.source()
    }

    pub fn _size(&mut self) -> u32 {
        self.val.size() as u32
    }

    pub fn _list_map_bool_bool(&mut self, columns: bool, module: bool) -> _SourceListMap {
        _SourceListMap::new(self.val.list_map(columns, module))
    }

    pub fn _node_bool_bool(&mut self, columns: bool, module: bool) -> _MSourceNode {
        _MSourceNode::new(self.val.node(columns, module))
    }
}

impl _RawSource {
    pub fn get_raw(self) -> Box<RawSource> {
        self.val
    }
}
