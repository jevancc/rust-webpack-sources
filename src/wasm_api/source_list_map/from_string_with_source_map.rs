use source_list_map::*;
use types::StringPtr;
use wasm_api::_SourceListMap;
use wasm_api::wasm_containers::StringVec;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn _from_string_with_source_map(
    code: String,
    sources: &[i32],
    sources_content: &[i32],
    mappings: String,
) -> _SourceListMap {
    let sources = sources.to_vec();
    let sources_content = sources_content.to_vec();

    _SourceListMap::new(from_string_with_source_map(
        StringPtr::Str(code),
        sources,
        sources_content,
        StringPtr::Str(mappings),
    ))
}
