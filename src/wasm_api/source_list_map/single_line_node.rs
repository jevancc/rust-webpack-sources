use source_list_map::*;
use types::StringPtr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct _SingleLineNode {
    val: SingleLineNode,
}

#[wasm_bindgen]
impl _SingleLineNode {
    pub fn _new_string_null_null_number(
        generated_code: String,
        starting_line: u32,
    ) -> _SingleLineNode {
        _SingleLineNode {
            val: SingleLineNode::new(generated_code, None, None, starting_line as usize),
        }
    }

    pub fn _new_string_string_string_number(
        generated_code: String,
        source: String,
        original_source: String,
        starting_line: u32,
    ) -> _SingleLineNode {
        _SingleLineNode {
            val: SingleLineNode::new(
                generated_code,
                Some(StringPtr::Str(source)),
                Some(StringPtr::Str(original_source)),
                starting_line as usize,
            ),
        }
    }

    pub fn _clone(&self) -> _SingleLineNode {
        _SingleLineNode {
            val: self.val.clone(),
        }
    }
}

impl _SingleLineNode {
    pub fn get(&self) -> &SingleLineNode {
        &self.val
    }

    pub fn get_mut(&mut self) -> &mut SingleLineNode {
        &mut self.val
    }
}
