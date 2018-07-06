use serde_json;
use source_map::types::*;
use source_map::*;
use types::StringPtr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct _MSourceNode {
    val: SourceNode,
}

#[wasm_bindgen]
impl _MSourceNode {
    pub fn _new_number_number_string_null(line: u32, column: u32, source: String) -> _MSourceNode {
        _MSourceNode {
            val: SourceNode::new(
                Some((line as usize, column as usize)),
                Some(StringPtr::Str(source)),
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
        self.val.add(Node::NString(chunk));
    }

    pub fn _add_sourcenode(&mut self, chunk: _MSourceNode) {
        self.val.add(Node::NSourceNode(chunk.val));
    }

    pub fn _to_string_with_source_map_string(&mut self, file: String) -> String {
        serde_json::to_string(
            &self
                .val
                .to_string_with_source_map(Some(StringPtr::Str(file)), None),
        ).unwrap()
    }

    pub fn _to_string_with_source_map_null(&mut self) -> String {
        serde_json::to_string(&self.val.to_string_with_source_map(None, None)).unwrap()
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
