use source_map::types::*;
use source_map::*;
use wasm_api::JsStringWithSourceMap;
use types::string_slice::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct _MSourceNode {
    val: SourceNode,
}

#[wasm_bindgen]
impl _MSourceNode {
    pub fn _new_number_number_sidx_null(line: u32, column: u32, source: i32) -> _MSourceNode {
        _MSourceNode {
            val: SourceNode::new(
                Some((line as usize, column as usize)),
                Some(source),
                None,
                None,
            ),
        }
    }

    pub fn _new_null_null_null_null() -> _MSourceNode {
        _MSourceNode {
            val: SourceNode::new(None, None, None, None),
        }
    }

    pub fn _add_string(&mut self, chunk: String) {
        self.val.add(Node::NString(StringSlice::from(chunk)));
    }

    pub fn _add_sourcenode(&mut self, chunk: _MSourceNode) {
        self.val.add(Node::NSourceNode(chunk.val));
    }

    pub fn _to_string_with_source_map_null(&mut self) -> JsStringWithSourceMap {
        JsStringWithSourceMap::from_with_generator(
            self.val.to_string_with_source_map_generator(None, None),
        )
    }
}

impl _MSourceNode {
    pub fn new(msn: SourceNode) -> _MSourceNode {
        _MSourceNode { val: msn }
    }

    pub fn get(&self) -> &SourceNode {
        &self.val
    }

    pub fn get_mut(&mut self) -> &mut SourceNode {
        &mut self.val
    }
}
