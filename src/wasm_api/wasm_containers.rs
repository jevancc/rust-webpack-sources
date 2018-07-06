use source_list_map::types::*;
use wasm_api::{_CodeNode, _SingleLineNode, _SourceListMap, _SourceNode};
use wasm_bindgen::prelude::*;

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
