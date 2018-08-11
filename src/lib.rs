#![feature(wasm_custom_section, wasm_import_module, use_extern_macros)]
extern crate linked_hash_map;
extern crate serde;
extern crate source_map_mappings;
extern crate vlq;
#[macro_use]
extern crate lazy_static;
extern crate bytecount;
extern crate regex;
extern crate wasm_bindgen;

pub mod concat_source;
pub mod line_to_line_mapped_source;
pub mod original_source;
pub mod prefix_source;
pub mod raw_source;
pub mod replace_source;
pub mod source;
pub mod source_list_map;
pub mod source_map;
pub mod source_map_source;
pub mod types;
pub mod utils;

mod wasm_api;
pub use wasm_api::*;
