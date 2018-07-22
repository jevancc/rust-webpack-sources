use source_list_map::types::*;
use types::StringWithSourceMap;
use wasm_api::{_CodeNode, _SingleLineNode, _SourceListMap, _SourceNode};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct JsStringWithSourceMap {
    s: String,
    pub version: i32,
    pub file: i32,
    sources: Vec<i32>,
    sources_content: Vec<i32>,
    names: Vec<i32>,
    mappings: String,
}

#[wasm_bindgen]
impl JsStringWithSourceMap {
    pub fn sources(&self) -> Box<[i32]> {
        self.sources.clone().into_boxed_slice()
    }

    pub fn sources_content(&self) -> Box<[i32]> {
        self.sources_content.clone().into_boxed_slice()
    }

    pub fn names(&self) -> Box<[i32]> {
        self.names.clone().into_boxed_slice()
    }

    pub fn s(&self) -> String {
        self.s.clone()
    }

    pub fn mappings(&self) -> String {
        self.mappings.clone()
    }
}

impl JsStringWithSourceMap {
    pub fn from(smap: StringWithSourceMap) -> JsStringWithSourceMap {
        let s = smap.source;
        let map = smap.map;
        let version = map.version;
        let file = map.file.unwrap_or(-1);
        let mappings = map.mappings;

        let sources = map.sources;
        let sources_content = map.sources_content;
        let names = map.names;
        JsStringWithSourceMap {
            s,
            version,
            file,
            sources,
            sources_content,
            names,
            mappings,
        }
    }
}

#[wasm_bindgen]
pub struct StringVec {
    val: Vec<String>,
}

#[wasm_bindgen]
impl StringVec {
    pub fn new() -> StringVec {
        StringVec { val: Vec::new() }
    }

    pub fn push_string(&mut self, s: String) {
        self.val.push(s);
    }
}

impl StringVec {
    pub fn get(&self) -> &Vec<String> {
        &self.val
    }

    pub fn get_mut(&mut self) -> &mut Vec<String> {
        &mut self.val
    }

    pub fn get_raw(self) -> Vec<String> {
        self.val
    }
}

#[wasm_bindgen]
pub struct NodeVec {
    val: Vec<Node>,
}

#[wasm_bindgen]
impl NodeVec {
    pub fn new() -> NodeVec {
        NodeVec { val: Vec::new() }
    }

    pub fn push_string(&mut self, s: String) {
        self.val.push(Node::NString(s));
    }

    pub fn push_sourcenode(&mut self, sn: &_SourceNode) {
        self.val.push(Node::NSourceNode(sn.get().clone()));
    }

    pub fn push_codenode(&mut self, cn: &_CodeNode) {
        self.val.push(Node::NCodeNode(cn.get().clone()));
    }

    pub fn push_singlelinenode(&mut self, sln: &_SingleLineNode) {
        self.val.push(Node::NSingleLineNode(sln.get().clone()));
    }

    pub fn push_sourcelistmap(&mut self, slp: &_SourceListMap) {
        self.val.push(Node::NSourceListMap(slp.get().clone()));
    }
}

impl NodeVec {
    pub fn get(&self) -> &Vec<Node> {
        &self.val
    }

    pub fn get_mut(&mut self) -> &mut Vec<Node> {
        &mut self.val
    }

    pub fn get_raw(self) -> Vec<Node> {
        self.val
    }

    pub fn get_raw_first(mut self) -> Node {
        self.val.pop().unwrap()
    }
}
