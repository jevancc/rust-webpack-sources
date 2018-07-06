use std::rc::Rc;
use std::cell::RefCell;
use source_map_source::*;
use wasm_bindgen::prelude::*;
use wasm_api::{_MSourceNode, _SourceListMap};
use source::SourceTrait;
use serde_json;

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
            sources: String,
            sources_content: String,
            mappings: String
        ) -> _SourceMapSource {
        let sources: Vec<String> = serde_json::from_str(&sources).unwrap();
        let sources_content: Vec<String> = serde_json::from_str(&sources_content).unwrap();
        _SourceMapSource {
            val: Rc::new(RefCell::new(SourceMapSource::new(
                value,
                name,
                sources,
                sources_content,
                mappings
            ))),
        }
    }

    pub fn _source(&mut self) -> String {
        self.val.borrow_mut().source()
    }

    pub fn _size(&mut self) -> u32 {
        self.val.borrow_mut().size() as u32
    }

    pub fn _set_source_map_consumer_string(&mut self, json: String) {
        self.val.borrow_mut().set_source_map_consumer(json);
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
