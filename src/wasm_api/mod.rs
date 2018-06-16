mod original_source;
mod replace_source;
mod source_list_map;
mod wasm_containers;

pub use wasm_api::original_source::*;
pub use wasm_api::replace_source::*;
pub use wasm_api::source_list_map::*;
pub use wasm_api::wasm_containers::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "source-map", version = "*")]
extern {
    pub type SourceNode;

    #[wasm_bindgen(constructor)]
    pub fn new_number_number_string(line: u32, column: u32, source: &str) -> SourceNode;
    #[wasm_bindgen(constructor)]
    pub fn new_null_null_null() -> SourceNode;

    #[wasm_bindgen(method, js_name  = add)]
    pub fn add_string(this: &SourceNode, chunk: &str);
    #[wasm_bindgen(method, js_name  = add)]
    pub fn add_sourcenode(this: &SourceNode, chunk: &SourceNode);

    #[wasm_bindgen(method)]
    pub fn setSourceContent(this: &SourceNode, sourceFile: &str, sourceContent: &str);
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console, js_name=log)]
    pub fn clog(s: &str);
}
