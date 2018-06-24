use source_map::*;
use wasm_bindgen::prelude::*;
use serde_json;

#[wasm_bindgen]
pub struct _MSourceNode {
    val: SourceNode,
}

#[wasm_bindgen]
impl _MSourceNode {
    pub fn _new_number_number_string_null(line: u32, column: u32, source: String) -> _MSourceNode {
        _MSourceNode {
            val: SourceNode::new(Some((line as usize, column as usize)), Some(StringPtr::Str(source)), None),
        }
    }

    pub fn _new_null_null_null_null() -> _MSourceNode {
        _MSourceNode {
            val: SourceNode::new(None, None, None),
        }
    }

    pub fn _to_string_with_source_map_string(&mut self, file: String) -> String {
        serde_json::to_string(&self.val.to_string_with_source_map(Some(StringPtr::Str(file)), None)).unwrap()
    }
}

impl _MSourceNode {
    pub fn get(&self) -> &SourceNode {
        &self.val
    }

    pub fn get_mut(&mut self) -> &mut SourceNode {
        &mut self.val
    }
}
