use std::rc::Rc;
use std::cell::RefCell;
use concat_source::*;
use wasm_bindgen::prelude::*;
use wasm_api::{_MSourceNode, _SourceListMap};
use wasm_api::{_RawSource, _OriginalSource, _ReplaceSource, _PrefixSource, _LineToLineMappedSource, _SourceMapSource};
use source::{SourceTrait, Source};
use wasm_api::clog;

#[wasm_bindgen]
pub struct _ConcatSource {
    val: Rc<RefCell<ConcatSource>>,
}

#[wasm_bindgen]
impl _ConcatSource {
    pub fn _new() -> _ConcatSource {
        _ConcatSource {
            val: Rc::new(RefCell::new(ConcatSource::new())),
        }
    }

    pub fn _add_string(&mut self, item: String) {
        self.val.borrow_mut().add(Source::SString(Rc::new(item)));
    }
    pub fn _add_raw_source(&mut self, item: &_RawSource) {
        self.val.borrow_mut().add(Source::Raw(item.get_raw()))
    }
    pub fn _add_original_source(&mut self, item: &_OriginalSource) {
        self.val.borrow_mut().add(Source::Original(item.get_raw()))
    }
    pub fn _add_replace_source(&mut self, item: &_ReplaceSource) {
        self.val.borrow_mut().add(Source::Replace(item.get_raw()))
    }
    pub fn _add_prefix_source(&mut self, item: &_PrefixSource) {
        self.val.borrow_mut().add(Source::Prefix(item.get_raw()))
    }
    pub fn _add_concat_source(&mut self, item: &_ConcatSource) {
        self.val.borrow_mut().add(Source::Concat(item.get_raw()))
    }
    pub fn _add_line_to_line_mapped_source(&mut self, item: &_LineToLineMappedSource) {
        self.val.borrow_mut().add(Source::LineToLineMapped(item.get_raw()))
    }
    pub fn _add_source_map_source(&mut self, item: &_SourceMapSource) {
        self.val.borrow_mut().add(Source::SourceMapSource(item.get_raw()))
    }

    pub fn _source(&mut self) -> String {
        self.val.borrow_mut().source()
    }

    pub fn _size(&mut self) -> u32 {
        self.val.borrow_mut().size() as u32
    }

    pub fn _list_map_bool_bool(&mut self, columns: bool, module: bool) -> _SourceListMap {
        _SourceListMap::new(self.val.borrow_mut().list_map(columns, module))
    }

    pub fn _node_bool_bool(&mut self, columns: bool, module: bool) -> _MSourceNode {
        _MSourceNode::new(self.val.borrow_mut().node(columns, module))
    }
}

impl _ConcatSource {
    pub fn get_raw(&self) -> Rc<RefCell<ConcatSource>> {
        Rc::clone(&self.val)
    }
}
