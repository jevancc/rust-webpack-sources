use source_list_map::*;
use types::string_slice::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct _CodeNode {
    val: CodeNode,
}

#[wasm_bindgen]
impl _CodeNode {
    pub fn _new_string(generated_code: String) -> _CodeNode {
        _CodeNode {
            val: CodeNode::new(StringSlice::from(generated_code)),
        }
    }

    pub fn _clone(&self) -> _CodeNode {
        _CodeNode {
            val: self.val.clone(),
        }
    }
}

impl _CodeNode {
    pub fn get(&self) -> &CodeNode {
        &self.val
    }

    pub fn get_mut(&mut self) -> &mut CodeNode {
        &mut self.val
    }
}
