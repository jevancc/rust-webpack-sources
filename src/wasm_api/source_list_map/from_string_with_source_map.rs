use source_list_map::*;
use wasm_api::_SourceListMap;
use wasm_api::wasm_containers::StringVec;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn _from_string_with_source_map(
    code: &str,
    sources: StringVec,
    sources_content: StringVec,
    mappings: &str,
) -> _SourceListMap {
    let sources = sources.get();
    let sources_content = sources_content.get();

    _SourceListMap::new(from_string_with_source_map(
        code,
        &sources.iter().map(|s| s.as_str()).collect(),
        &sources_content.iter().map(|s| s.as_str()).collect(),
        mappings,
    ))
}
