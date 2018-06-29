use line_to_line_mapped_source::*;
use wasm_bindgen::prelude::*;
use wasm_api::{_MSourceNode, _SourceListMap};
use source::SourceTrait;

#[wasm_bindgen]
pub struct _LineToLineMappedSource {
    val: Box<LineToLineMappedSource>,
}

#[wasm_bindgen]
impl _LineToLineMappedSource {
    pub fn _new_string_string_string(value: String, name: String, original_source: String) -> _LineToLineMappedSource {
        _LineToLineMappedSource {
            val: Box::new(LineToLineMappedSource::new(value, name, original_source)),
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

impl _LineToLineMappedSource {
    pub fn get_raw(self) -> Box<LineToLineMappedSource> {
        self.val
    }
}
