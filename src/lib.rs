#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate source_list_map;
extern crate serde;
extern crate serde_json;

mod original_source;
mod replace_source;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "source-map", version = "*")]
extern {
    pub type SourceNode;

    #[wasm_bindgen(constructor)]
    pub fn new_number_number_string(line: u32, column: u32, source: &str) -> SourceNode;
    #[wasm_bindgen(constructor)]
    pub fn new_null_null_string(source: &str) -> SourceNode;
    #[wasm_bindgen(constructor)]
    pub fn new_null_null_null() -> SourceNode;

    #[wasm_bindgen(method, js_name  = add)]
    pub fn add_string(this: &SourceNode, chunk: &str);
    #[wasm_bindgen(method, js_name  = add)]
    pub fn add_sourcenode(this: &SourceNode, chunk: &SourceNode);

    #[wasm_bindgen(method)]
    pub fn setSourceContent(this: &SourceNode, sourceFile: &str, sourceContent: &str);

    #[wasm_bindgen(method)]
    pub fn toStringWithSourceMap_mapToString(this: &SourceNode, options_file: &str) -> String;
}
