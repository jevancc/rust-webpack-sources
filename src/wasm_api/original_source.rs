use original_source::*;
use wasm_bindgen::prelude::*;
use wasm_api::{_MSourceNode, _SourceListMap};
use source::SourceTrait;

#[wasm_bindgen]
#[derive(Debug)]
pub struct _OriginalSource {
    val: Box<OriginalSource>,
}

#[wasm_bindgen]
impl _OriginalSource {
    pub fn _new_string_string(source_code: String, name: String) -> _OriginalSource {
        _OriginalSource {
            val: Box::new(OriginalSource::new(source_code, name)),
        }
    }

    pub fn _source(&mut self) -> String {
        self.val.source()
    }

    pub fn _size(&mut self) -> u32 {
        self.val.size() as u32
    }

    pub fn _name(&mut self) -> String {
        self.val.name.clone()
    }

    pub fn _list_map_bool_bool(&mut self, columns: bool, module: bool) -> _SourceListMap {
        _SourceListMap::new(self.val.list_map(columns, module))
    }

    pub fn _node_bool_bool(&mut self, columns: bool, module: bool) -> _MSourceNode {
        _MSourceNode::new(self.val.node(columns, module))
    }
}

impl _OriginalSource {
    pub fn get_raw(self) -> Box<OriginalSource> {
        self.val
    }
}
