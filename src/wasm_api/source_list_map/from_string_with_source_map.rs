use source_list_map::*;
use types::StringPtr;
use wasm_api::_SourceListMap;
use wasm_api::wasm_containers::StringVec;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn _from_string_with_source_map(
    code: String,
    sources: StringVec,
    sources_content: StringVec,
    mappings: String,
) -> _SourceListMap {
    let sources = sources.get_raw();
    let sources_content = sources_content.get_raw();

    _SourceListMap::new(from_string_with_source_map(
        StringPtr::Str(code),
        sources.into_iter().map(|s| StringPtr::Str(s)).collect(),
        sources_content
            .into_iter()
            .map(|s| StringPtr::Str(s))
            .collect(),
        StringPtr::Str(mappings),
    ))
}
