use source_list_map::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct _SourceNode {
    val: SourceNode,
}

#[wasm_bindgen]
impl _SourceNode {
    pub fn _new_string_null_null_number(generated_code: String, starting_line: u32) -> _SourceNode {
        _SourceNode {
            val: SourceNode::new(generated_code, None, None, starting_line as usize),
        }
    }

    pub fn _new_string_string_string_number(
        generated_code: String,
        source: String,
        original_source: String,
        starting_line: u32,
    ) -> _SourceNode {
        _SourceNode {
            val: SourceNode::new(
                generated_code,
                Some(source),
                Some(original_source),
                starting_line as usize,
            ),
        }
    }

    pub fn _clone(&self) -> _SourceNode {
        _SourceNode {
            val: self.val.clone(),
        }
    }
}

impl _SourceNode {
    pub fn get(&self) -> &SourceNode {
        &self.val
    }

    pub fn get_mut(&mut self) -> &mut SourceNode {
        &mut self.val
    }
}
