use source_map_source::*;
use wasm_bindgen::prelude::*;
use wasm_api::{_MSourceNode, _SourceListMap, StringVec};
use source::SourceTrait;
use serde_json;
use wasm_api::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct _SourceMapSource {
    val: Box<SourceMapSource>,
}

#[wasm_bindgen]
impl _SourceMapSource {
    pub fn _new_string_string_map(
            value: String,
            name: String,
            sources: String,
            sources_content: String,
            mappings: String
        ) -> _SourceMapSource {
        let sources: Vec<String> = serde_json::from_str(&sources).unwrap();
        let sources_content: Vec<String> = serde_json::from_str(&sources_content).unwrap();
        _SourceMapSource {
            val: Box::new(SourceMapSource::new(
                value,
                name,
                sources,
                sources_content,
                mappings
            )),
        }
    }

    pub fn _source(&mut self) -> String {
        self.val.source()
    }

    pub fn _size(&mut self) -> u32 {
        self.val.size() as u32
    }

    pub fn _set_source_map_consumer_string(&mut self, json: String) {
        self.val.set_source_map_consumer(json);
    }

    pub fn _list_map_bool_bool(&mut self, columns: bool, module: bool) -> _SourceListMap {
        _SourceListMap::new(self.val.list_map(columns, module))
    }

    pub fn _node_bool_bool(&mut self, columns: bool, module: bool) -> _MSourceNode {
        _MSourceNode::new(self.val.node(columns, module))
    }
}

impl _SourceMapSource {
    pub fn get_raw(self) -> Box<SourceMapSource> {
        self.val
    }
}
