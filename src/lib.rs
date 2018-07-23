#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate linked_hash_map;
extern crate serde;
extern crate serde_json;
extern crate source_map_mappings;
extern crate vlq;
extern crate wasm_bindgen;

mod concat_source;
mod line_to_line_mapped_source;
mod original_source;
mod prefix_source;
mod raw_source;
mod replace_source;
mod source;
mod source_list_map;
mod source_map;
mod source_map_source;
mod types;
mod utils;

mod wasm_api;

pub use wasm_api::*;
