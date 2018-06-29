use prefix_source::*;
use wasm_bindgen::prelude::*;
use wasm_api::{_MSourceNode, _SourceListMap};
use wasm_api::{_RawSource, _OriginalSource, _ReplaceSource, _ConcatSource, _LineToLineMappedSource};
use source::{Source, SourceTrait};

#[wasm_bindgen]
pub struct _PrefixSource {
    val: Box<PrefixSource>,
}

#[wasm_bindgen]
impl _PrefixSource {
    pub fn _new_string_string(prefix: String, source: String) -> _PrefixSource {
        _PrefixSource {
            val: Box::new(PrefixSource::new(prefix, Source::SString(Box::new(source)))),
        }
    }
    pub fn _new_string_raw_source(prefix: String, source: _RawSource) -> _PrefixSource {
        _PrefixSource {
            val: Box::new(PrefixSource::new(prefix, Source::Raw(source.get_raw()))),
        }
    }
    pub fn _new_string_original_source(prefix: String, source: _OriginalSource) -> _PrefixSource {
        _PrefixSource {
            val: Box::new(PrefixSource::new(prefix, Source::Original(source.get_raw()))),
        }
    }
    pub fn _new_string_replace_source(prefix: String, source: _ReplaceSource) -> _PrefixSource {
        _PrefixSource {
            val: Box::new(PrefixSource::new(prefix, Source::Replace(source.get_raw()))),
        }
    }
    pub fn _new_string_prefix_source(prefix: String, source: _PrefixSource) -> _PrefixSource {
        _PrefixSource {
            val: Box::new(PrefixSource::new(prefix, Source::Prefix(source.get_raw()))),
        }
    }
    pub fn _new_string_concat_source(prefix: String, source: _ConcatSource) -> _PrefixSource {
        _PrefixSource {
            val: Box::new(PrefixSource::new(prefix, Source::Concat(source.get_raw()))),
        }
    }
    pub fn _new_string_line_to_line_mapped_source(prefix: String, source: _LineToLineMappedSource) -> _PrefixSource {
        _PrefixSource {
            val: Box::new(PrefixSource::new(prefix, Source::LineToLineMapped(source.get_raw()))),
        }
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

impl _PrefixSource {
    pub fn get_raw(self) -> Box<PrefixSource> {
        self.val
    }
}
