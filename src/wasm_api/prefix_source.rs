use prefix_source::*;
use source::{Source, SourceTrait};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_api::{
    _ConcatSource, _LineToLineMappedSource, _OriginalSource, _RawSource, _ReplaceSource,
    _SourceMapSource,
};
use wasm_api::{_MSourceNode, _SourceListMap};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct _PrefixSource {
    val: Rc<RefCell<PrefixSource>>,
}

#[wasm_bindgen]
impl _PrefixSource {
    pub fn _new_string_string(prefix: String, source: String) -> _PrefixSource {
        _PrefixSource {
            val: Rc::new(RefCell::new(PrefixSource::new(
                prefix,
                Source::SString(Rc::new(source)),
            ))),
        }
    }
    pub fn _new_string_raw_source(prefix: String, source: &_RawSource) -> _PrefixSource {
        _PrefixSource {
            val: Rc::new(RefCell::new(PrefixSource::new(
                prefix,
                Source::Raw(source.get_raw()),
            ))),
        }
    }
    pub fn _new_string_original_source(prefix: String, source: &_OriginalSource) -> _PrefixSource {
        _PrefixSource {
            val: Rc::new(RefCell::new(PrefixSource::new(
                prefix,
                Source::Original(source.get_raw()),
            ))),
        }
    }
    pub fn _new_string_replace_source(prefix: String, source: &_ReplaceSource) -> _PrefixSource {
        _PrefixSource {
            val: Rc::new(RefCell::new(PrefixSource::new(
                prefix,
                Source::Replace(source.get_raw()),
            ))),
        }
    }
    pub fn _new_string_prefix_source(prefix: String, source: &_PrefixSource) -> _PrefixSource {
        _PrefixSource {
            val: Rc::new(RefCell::new(PrefixSource::new(
                prefix,
                Source::Prefix(source.get_raw()),
            ))),
        }
    }
    pub fn _new_string_concat_source(prefix: String, source: &_ConcatSource) -> _PrefixSource {
        _PrefixSource {
            val: Rc::new(RefCell::new(PrefixSource::new(
                prefix,
                Source::Concat(source.get_raw()),
            ))),
        }
    }
    pub fn _new_string_line_to_line_mapped_source(
        prefix: String,
        source: &_LineToLineMappedSource,
    ) -> _PrefixSource {
        _PrefixSource {
            val: Rc::new(RefCell::new(PrefixSource::new(
                prefix,
                Source::LineToLineMapped(source.get_raw()),
            ))),
        }
    }
    pub fn _new_string_source_map_source(
        prefix: String,
        source: &_SourceMapSource,
    ) -> _PrefixSource {
        _PrefixSource {
            val: Rc::new(RefCell::new(PrefixSource::new(
                prefix,
                Source::SourceMapSource(source.get_raw()),
            ))),
        }
    }

    pub fn _source(&mut self) -> String {
        (*self.val.borrow_mut().source()).clone()
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

impl _PrefixSource {
    pub fn get_raw(&self) -> Rc<RefCell<PrefixSource>> {
        Rc::clone(&self.val)
    }
}
