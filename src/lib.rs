#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate source_list_map;
extern crate source_map;
extern crate wasm_bindgen;

mod source;
mod raw_source;
mod original_source;
mod replace_source;
mod prefix_source;
mod concat_source;
mod line_to_line_mapped_source;
// mod wasm_api;

// pub use wasm_api::*;
