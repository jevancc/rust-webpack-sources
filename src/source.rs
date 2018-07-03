use raw_source::RawSource;
use original_source::OriginalSource;
use replace_source::ReplaceSource;
use prefix_source::PrefixSource;
use concat_source::ConcatSource;
use line_to_line_mapped_source::LineToLineMappedSource;
use source_map_source::SourceMapSource;

use std::rc::Rc;
use std::cell::RefCell;
use source_map::SourceNode;
use source_list_map::SourceListMap;

#[derive(Debug, Clone)]
pub enum Source {
    Raw(Rc<RefCell<RawSource>>),
    Original(Rc<RefCell<OriginalSource>>),
    Replace(Rc<RefCell<ReplaceSource>>),
    Prefix(Rc<RefCell<PrefixSource>>),
    Concat(Rc<RefCell<ConcatSource>>),
    LineToLineMapped(Rc<RefCell<LineToLineMappedSource>>),
    SourceMapSource(Rc<RefCell<SourceMapSource>>),
    SString(Rc<String>),
}

impl SourceTrait for Source {
    #[inline]
    fn source(&mut self) -> String {
        match self {
            Source::Raw(s) => (*s).borrow_mut().source(),
            Source::Original(s) => s.borrow_mut().source(),
            Source::Replace(s) => s.borrow_mut().source(),
            Source::Prefix(s) => s.borrow_mut().source(),
            Source::Concat(s) => s.borrow_mut().source(),
            Source::LineToLineMapped(s) => s.borrow_mut().source(),
            Source::SourceMapSource(s) => s.borrow_mut().source(),
            Source::SString(s) => (**s).clone()
        }
    }

    #[inline]
    fn size(&mut self) -> usize {
        match self {
            Source::Raw(s) => s.borrow_mut().size(),
            Source::Original(s) => s.borrow_mut().size(),
            Source::Replace(s) => s.borrow_mut().size(),
            Source::Prefix(s) => s.borrow_mut().size(),
            Source::Concat(s) => s.borrow_mut().size(),
            Source::LineToLineMapped(s) => s.borrow_mut().size(),
            Source::SourceMapSource(s) => s.borrow_mut().size(),
            Source::SString(s) => s.len()
        }
    }

    #[inline]
    fn list_map(&mut self, columns: bool, module: bool) -> SourceListMap {
        match self {
            Source::Raw(s) => s.borrow_mut().list_map(columns, module),
            Source::Original(s) => s.borrow_mut().list_map(columns, module),
            Source::Replace(s) => s.borrow_mut().list_map(columns, module),
            Source::Prefix(s) => s.borrow_mut().list_map(columns, module),
            Source::Concat(s) => s.borrow_mut().list_map(columns, module),
            Source::LineToLineMapped(s) => s.borrow_mut().list_map(columns, module),
            Source::SourceMapSource(s) => s.borrow_mut().list_map(columns, module),
            Source::SString(_) => panic!()
        }
    }

    #[inline]
    fn node(&mut self, columns: bool, module: bool) -> SourceNode {
        match self {
            Source::Raw(s) => s.borrow_mut().node(columns, module),
            Source::Original(s) => s.borrow_mut().node(columns, module),
            Source::Replace(s) => s.borrow_mut().node(columns, module),
            Source::Prefix(s) => s.borrow_mut().node(columns, module),
            Source::Concat(s) => s.borrow_mut().node(columns, module),
            Source::LineToLineMapped(s) => s.borrow_mut().node(columns, module),
            Source::SourceMapSource(s) => s.borrow_mut().node(columns, module),
            Source::SString(_) => panic!()
        }
    }
}

pub trait SourceTrait {
    fn source(&mut self) -> String;
    fn size(&mut self) -> usize {
        self.source().len()
    }
    fn list_map(&mut self, columns: bool, module: bool) -> SourceListMap;
    fn node(&mut self, columns: bool, module: bool) -> SourceNode;
}

// TODO: implement a marco for simple source matching
// match child {
//     Source::Raw(s) =>
//     Source::Original(s) =>
//     Source::Replace(s) =>
//     Source::Prefix(s) =>
//     Source::Concat(s) =>
//     Source::LineToLineMapped(s) =>
//     Source::SString(s) =>
// }
