use super::mapping_functions::{IdenticalFunction, PrefixMappingFunction, TestMappingFunction};
use replace_source::ReplaceMappingFunction;
use serde_json;
use source_list_map::types::*;
use source_list_map::*;
use types::StringPtr;
use wasm_api::NodeVec;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct _SourceListMap {
    val: SourceListMap,
}

#[wasm_bindgen]
impl _SourceListMap {
    pub fn _new() -> _SourceListMap {
        _SourceListMap {
            val: SourceListMap::new(None, None, None),
        }
    }

    pub fn _new_nodes(nv: NodeVec) -> _SourceListMap {
        _SourceListMap {
            val: SourceListMap::new(Some(GenCode::CodeVec(nv.get_raw())), None, None),
        }
    }

    pub fn _add_node(&mut self, nv: NodeVec) {
        self.val.add(nv.get_raw_first(), None, None);
    }

    pub fn _add_node_string_string(
        &mut self,
        nv: NodeVec,
        source: String,
        original_source: String,
    ) {
        self.val.add(
            nv.get_raw_first(),
            Some(StringPtr::Str(source)),
            Some(StringPtr::Str(original_source)),
        );
    }

    pub fn _prepend_node(&mut self, nv: NodeVec) {
        self.val.prepend(nv.get_raw_first(), None, None);
    }

    pub fn _prepend_node_string_string(
        &mut self,
        nv: NodeVec,
        source: String,
        original_source: String,
    ) {
        self.val.prepend(
            nv.get_raw_first(),
            Some(StringPtr::Str(source)),
            Some(StringPtr::Str(original_source)),
        );
    }

    pub fn _to_string(&self) -> String {
        self.val.to_string()
    }

    pub fn _to_string_with_source_map(&mut self) -> JsSrcMap {
        let srcmap = self.val.to_string_with_source_map(None);
        JsSrcMap {
            source: srcmap.source,
            map_sources_content: srcmap.map.sources_content,
            map_sources: srcmap.map.sources,
            mappings: srcmap.map.mappings,
        }
    }
}

#[wasm_bindgen]
pub fn _sourcelistmap_map_generated_code_identical(slp: _SourceListMap) -> _SourceListMap {
    let mut mf = IdenticalFunction {};

    _SourceListMap {
        val: slp.val.map_generated_code(&mut mf),
    }
}

#[wasm_bindgen]
pub fn _sourcelistmap_map_generated_code_test(slp: _SourceListMap) -> _SourceListMap {
    let mut mf = TestMappingFunction {};

    _SourceListMap {
        val: slp.val.map_generated_code(&mut mf),
    }
}

#[wasm_bindgen]
pub fn _sourcelistmap_map_generated_code_prefix(
    slp: _SourceListMap,
    prefix: String,
) -> _SourceListMap {
    let mut mf = PrefixMappingFunction { prefix };

    _SourceListMap {
        val: slp.val.map_generated_code(&mut mf),
    }
}

#[wasm_bindgen]
pub fn _sourcelistmap_map_generated_code_replace(
    slp: _SourceListMap,
    replacements: String,
) -> _SourceListMap {
    let replacements: Vec<(i64, i64, String, usize)> = serde_json::from_str(&replacements).unwrap();
    let mut mf = ReplaceMappingFunction::new(&replacements);

    let mut map = slp.val.map_generated_code(&mut mf);
    let mut extra_code = String::new();
    while mf.replacement_idx >= 0 {
        extra_code += &replacements[mf.replacement_idx as usize].2;
        mf.replacement_idx -= 1;
    }

    if !extra_code.is_empty() {
        map.add(Node::NString(extra_code), None, None);
    }

    _SourceListMap { val: map }
}

impl _SourceListMap {
    pub fn new(slp: SourceListMap) -> _SourceListMap {
        _SourceListMap { val: slp }
    }

    pub fn get(&self) -> &SourceListMap {
        &self.val
    }

    pub fn get_mut(&mut self) -> &mut SourceListMap {
        &mut self.val
    }

    pub fn get_raw(self) -> SourceListMap {
        self.val
    }
}

#[wasm_bindgen]
pub struct JsSrcMap {
    source: String,
    map_sources: Vec<String>,
    map_sources_content: Vec<String>,
    mappings: String,
}

#[wasm_bindgen]
impl JsSrcMap {
    pub fn get_source(&self) -> String {
        self.source.clone()
    }

    pub fn get_map_contents_len(&self) -> i32 {
        self.map_sources_content.len() as i32
    }

    pub fn get_map_sources_len(&self) -> i32 {
        self.map_sources.len() as i32
    }

    pub fn get_map_contents_nth(&self, idx: i32) -> String {
        self.map_sources_content.get(idx as usize).unwrap().clone()
    }

    pub fn get_map_sources_nth(&self, idx: i32) -> String {
        self.map_sources.get(idx as usize).unwrap().clone()
    }

    pub fn get_mappings(&self) -> String {
        self.mappings.clone()
    }
}
