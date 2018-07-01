use raw_source::RawSource;
use original_source::OriginalSource;
use replace_source::ReplaceSource;
use prefix_source::PrefixSource;
use concat_source::ConcatSource;
use line_to_line_mapped_source::LineToLineMappedSource;
use source_map_source::SourceMapSource;

// use std::rc::Rc;
use source_map::SourceNode;
use source_list_map::SourceListMap;

#[derive(Debug)]
pub enum Source {
    Raw(Box<RawSource>),
    Original(Box<OriginalSource>),
    Replace(Box<ReplaceSource>),
    Prefix(Box<PrefixSource>),
    Concat(Box<ConcatSource>),
    LineToLineMapped(Box<LineToLineMappedSource>),
    SourceMapSource(Box<SourceMapSource>),
    SString(Box<String>),
}

impl SourceTrait for Source {
    #[inline]
    fn source(&mut self) -> String {
        match self {
            Source::Raw(s) => s.source(),
            Source::Original(s) => s.source(),
            Source::Replace(s) => s.source(),
            Source::Prefix(s) => s.source(),
            Source::Concat(s) => s.source(),
            Source::LineToLineMapped(s) => s.source(),
            Source::SourceMapSource(s) => s.source(),
            Source::SString(s) => (**s).clone()
        }
    }

    #[inline]
    fn size(&mut self) -> usize {
        match self {
            Source::Raw(s) => s.size(),
            Source::Original(s) => s.size(),
            Source::Replace(s) => s.size(),
            Source::Prefix(s) => s.size(),
            Source::Concat(s) => s.size(),
            Source::LineToLineMapped(s) => s.size(),
            Source::SourceMapSource(s) => s.size(),
            Source::SString(s) => s.len()
        }
    }

    #[inline]
    fn list_map(&mut self, columns: bool, module: bool) -> SourceListMap {
        match self {
            Source::Raw(s) => s.list_map(columns, module),
            Source::Original(s) => s.list_map(columns, module),
            Source::Replace(s) => s.list_map(columns, module),
            Source::Prefix(s) => s.list_map(columns, module),
            Source::Concat(s) => s.list_map(columns, module),
            Source::LineToLineMapped(s) => s.list_map(columns, module),
            Source::SourceMapSource(s) => s.list_map(columns, module),
            Source::SString(_) => panic!()
        }
    }

    #[inline]
    fn node(&mut self, columns: bool, module: bool) -> SourceNode {
        match self {
            Source::Raw(s) => s.node(columns, module),
            Source::Original(s) => s.node(columns, module),
            Source::Replace(s) => s.node(columns, module),
            Source::Prefix(s) => s.node(columns, module),
            Source::Concat(s) => s.node(columns, module),
            Source::LineToLineMapped(s) => s.node(columns, module),
            Source::SourceMapSource(s) => s.node(columns, module),
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
