use original_source::*;
use wasm_bindgen::prelude::*;
use SourceNode;

#[wasm_bindgen]
pub struct _OriginalSource {
    val: OriginalSource,
}

#[wasm_bindgen]
impl _OriginalSource {
    pub fn _new_string_string(source_code: String, name: String) -> _OriginalSource {
        _OriginalSource {
            val: OriginalSource::new(source_code, name),
        }
    }

    pub fn _source(&self) -> String {
        self.val.source()
    }

    pub fn _size(&self) -> u32 {
        self.val.size() as u32
    }

    pub fn _name(&self) -> String {
        self.val.name.clone()
    }

    pub fn _node_bool(&self, columns: bool) -> SourceNode {
        self.val.node(columns)
    }
}
