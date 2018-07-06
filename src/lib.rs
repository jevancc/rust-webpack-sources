#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate linked_hash_map;
extern crate vlq;
extern crate source_map_mappings;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod types;
mod source_map;
mod source_list_map;
mod source;
mod raw_source;
mod original_source;
mod replace_source;
mod prefix_source;
mod concat_source;
mod line_to_line_mapped_source;
mod source_map_source;

mod wasm_api;

pub use wasm_api::*;
