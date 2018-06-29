use concat_source::*;
use wasm_bindgen::prelude::*;
use wasm_api::{_MSourceNode, _SourceListMap};
use wasm_api::{_RawSource, _OriginalSource, _ReplaceSource, _PrefixSource, _LineToLineMappedSource};
use source::{SourceTrait, Source};

#[wasm_bindgen]
pub struct _ConcatSource {
    val: Box<ConcatSource>,
}

#[wasm_bindgen]
impl _ConcatSource {
    pub fn _new() -> _ConcatSource {
        _ConcatSource {
            val: Box::new(ConcatSource::new()),
        }
    }

    pub fn _add_string(&mut self, item: String) {
        self.val.add(Source::SString(Box::new(item)));
    }
    pub fn _add_raw_source(&mut self, item: _RawSource) {
        self.val.add(Source::Raw(item.get_raw()))
    }
    pub fn _add_original_source(&mut self, item: _OriginalSource) {
        self.val.add(Source::Original(item.get_raw()))
    }
    pub fn _add_replace_source(&mut self, item: _ReplaceSource) {
        self.val.add(Source::Replace(item.get_raw()))
    }
    pub fn _add_prefix_source(&mut self, item: _PrefixSource) {
        self.val.add(Source::Prefix(item.get_raw()))
    }
    pub fn _add_concat_source(&mut self, item: _ConcatSource) {
        self.val.add(Source::Concat(item.get_raw()))
    }
    pub fn _add_line_to_line_mapped_source(&mut self, item: _LineToLineMappedSource) {
        self.val.add(Source::LineToLineMapped(item.get_raw()))
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

impl _ConcatSource {
    pub fn get_raw(self) -> Box<ConcatSource> {
        self.val
    }
}
