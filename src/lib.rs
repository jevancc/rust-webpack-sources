#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate serde;
extern crate serde_derive;
extern crate source_list_map;
extern crate wasm_bindgen;
#[macro_use]
extern crate serde_json;

mod original_source;
mod replace_source;

mod wasm_api;

pub use wasm_api::*;
