use replace_source::*;
use wasm_bindgen::prelude::*;
use wasm_api::{_SourceListMap, _MSourceNode};
use wasm_api::{_RawSource, _OriginalSource, _PrefixSource, _ConcatSource, _LineToLineMappedSource};
use source::{SourceTrait, Source};

#[wasm_bindgen]
pub struct _ReplaceSource {
    val: Box<ReplaceSource>,
}

#[wasm_bindgen]
impl _ReplaceSource {
    // TODO: use macro
    pub fn _new_string(source: String) -> _ReplaceSource {
        _ReplaceSource {
            val: Box::new(ReplaceSource::new(Source::SString(Box::new(source)))),
        }
    }
    pub fn _new_raw_source(source: _RawSource) -> _ReplaceSource {
        _ReplaceSource {
            val: Box::new(ReplaceSource::new(Source::Raw(source.get_raw()))),
        }
    }
    pub fn _new_original_source(source: _OriginalSource) -> _ReplaceSource {
        _ReplaceSource {
            val: Box::new(ReplaceSource::new(Source::Original(source.get_raw()))),
        }
    }
    pub fn _new_replace_source(source: _ReplaceSource) -> _ReplaceSource {
        _ReplaceSource {
            val: Box::new(ReplaceSource::new(Source::Replace(source.get_raw()))),
        }
    }
    pub fn _new_prefix_source(source: _PrefixSource) -> _ReplaceSource {
        _ReplaceSource {
            val:Box::new(ReplaceSource::new(Source::Prefix(source.get_raw()))),
        }
    }
    pub fn _new_concat_source(source: _ConcatSource) -> _ReplaceSource {
        _ReplaceSource {
            val: Box::new(ReplaceSource::new(Source::Concat(source.get_raw()))),
        }
    }
    pub fn _new_line_to_line_mapped_source(source: _LineToLineMappedSource) -> _ReplaceSource {
        _ReplaceSource {
            val: Box::new(ReplaceSource::new(Source::LineToLineMapped(source.get_raw()))),
        }
    }

    pub fn _replace_number_number_string_number_number(
        &mut self,
        start: i32,
        end: i32,
        new_value: String,
        ord_s: i32,
        ord_e: i32,
    ) {
        self.val.replace(start, end, new_value, ord_s, ord_e);
    }

    pub fn _insert_number_string_number(&mut self, pos: i32, new_value: String, ord: i32) {
        self.val.insert(pos, new_value, ord);
    }

    pub fn _replacements_to_string(&mut self) -> String {
        self.val.replacements_to_string()
    }

    pub fn _source(&mut self) -> String {
        self.val.source()
    }

    pub fn _size(&mut self) -> u32 {
        self.val.size() as u32
    }

    pub fn _list_map_bool_bool(&mut self, columns: bool, module: bool) -> _SourceListMap {
        _SourceListMap::new(self.val.list_map(columns, module))
    }

    pub fn _node_bool_bool(&mut self, columns: bool, module: bool) -> _MSourceNode {
        _MSourceNode::new(self.val.node(columns, module))
    }
}

impl _ReplaceSource {
    pub fn get_raw(self) -> Box<ReplaceSource> {
        self.val
    }
}
