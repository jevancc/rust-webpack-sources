use source_list_map::types::*;
use source_map::SourceMapGenerator;
use types::*;
use wasm_api::{_CodeNode, _SingleLineNode, _SourceListMap, _SourceNode};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct JsStringWithSourceMap {
    s: String,
    map: Option<SourceMap>,
    generator: Option<SourceMapGenerator>,
}

#[wasm_bindgen]
impl JsStringWithSourceMap {
    pub fn s(&self) -> String {
        self.s.clone()
    }

    pub fn version(&mut self) -> i32 {
        self.map().version
    }

    pub fn file(&mut self) -> i32 {
        self.map().file.unwrap_or(-1)
    }

    pub fn sources(&mut self) -> Box<[i32]> {
        self.map().sources.clone().into_boxed_slice()
    }

    pub fn sources_content(&mut self) -> Box<[i32]> {
        self.map().sources_content.clone().into_boxed_slice()
    }

    pub fn names(&mut self) -> Box<[i32]> {
        self.map().names.clone().into_boxed_slice()
    }

    pub fn mappings(&mut self) -> String {
        self.map().mappings.clone()
    }
}

impl JsStringWithSourceMap {
    #[inline]
    fn map(&mut self) -> &SourceMap {
        if self.map.is_none() {
            self.map = Some(self.generator.as_mut().unwrap().to_source_map());
        }
        self.map.as_ref().unwrap()
    }

    pub fn from(smap: StringWithSourceMap) -> JsStringWithSourceMap {
        let s = smap.source;
        let map = smap.map;
        JsStringWithSourceMap {
            s,
            map: Some(map),
            generator: None,
        }
    }

    pub fn from_with_generator(smapgen: StringWithSourceMapGenerator) -> JsStringWithSourceMap {
        let s = smapgen.source;
        let generator = smapgen.generator;

        JsStringWithSourceMap {
            s,
            map: None,
            generator: Some(generator),
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
