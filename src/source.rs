use raw_source::RawSource;
use original_source::OriginalSource;
use replace_source::ReplaceSource;
use prefix_source::PrefixSource;
use concat_source::ConcatSource;
use line_to_line_mapped_source::LineToLineMappedSource;
// use std::rc::Rc;
use source_map::SourceNode;
use source_list_map::SourceListMap;

pub enum Source {
    Raw(Box<RawSource>),
    Original(Box<OriginalSource>),
    Replace(Box<ReplaceSource>),
    Prefix(Box<PrefixSource>),
    Concat(Box<ConcatSource>),
    LineToLineMapped(Box<LineToLineMappedSource>),
    SString(Box<String>),
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
//     Source::SString(s) =>
// }
