use super::mapping_functions::{IdenticalFunction, PrefixMappingFunction, TestMappingFunction};
use super::utils;
use source_list_map::*;
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

    pub fn _to_string_with_source_map(&mut self) -> String {
        let obj = self.val.to_string_with_source_map(None);
        utils::string_with_srcmap_to_json(&obj).to_string()
    }

    pub fn _to_string_with_source_map_string(&mut self, options_file: String) -> String {
        let obj = self.val.to_string_with_source_map(Some(options_file));
        utils::string_with_srcmap_to_json(&obj).to_string()
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
