use source::{Source, SourceTrait};
use source_map::{SourceNode, Node as SMNode};
use source_list_map::{SourceListMap, Node as SLMNode};

pub struct ConcatSource {
    children: Vec<Source>,
}

impl ConcatSource {
    pub fn new() -> ConcatSource {
        ConcatSource {
            children: Vec::new()
        }
    }
    
    pub fn add(&mut self, item: Source) {
        if let Source::Concat(mut cs) = item {
            self.children.append(&mut cs.children);
        } else {
            self.children.push(item);
        }
    }
}

impl SourceTrait for ConcatSource {
    fn source(&mut self) -> String {
        let sources: Vec<String> = self.children.iter_mut().map(|child| {
            match child {
                Source::Raw(s) => s.source(),
                Source::Original(s) => s.source(),
                Source::Replace(s) => s.source(),
                Source::Prefix(s) => s.source(),
                Source::Concat(s) => s.source(),
                Source::LineToLineMapped(s) => s.source(),
                Source::SString(s) => *s.clone(),
            }
        }).collect();
        sources.join("")
    }

    fn size(&mut self) -> usize {
        let mut ret = 0;
        for child in &mut self.children {
            ret += match child {
                Source::Raw(s) => s.size(),
                Source::Original(s) => s.size(),
                Source::Replace(s) => s.size(),
                Source::Prefix(s) => s.size(),
                Source::Concat(s) => s.size(),
                Source::LineToLineMapped(s) => s.size(),
                Source::SString(s) => s.len(),
            };
        }
        ret
    }

    fn list_map(&mut self, columns: bool, module: bool) -> SourceListMap {
        let mut map = SourceListMap::new(None, None, None);
        for child in &mut self.children {
            map.add(match child {
                Source::Raw(s) => SLMNode::NSourceListMap(s.list_map(columns, module)),
                Source::Original(s) => SLMNode::NSourceListMap(s.list_map(columns, module)),
                Source::Replace(s) => SLMNode::NSourceListMap(s.list_map(columns, module)),
                Source::Prefix(s) => SLMNode::NSourceListMap(s.list_map(columns, module)),
                Source::Concat(s) => SLMNode::NSourceListMap(s.list_map(columns, module)),
                Source::LineToLineMapped(s) => SLMNode::NSourceListMap(s.list_map(columns, module)),
                Source::SString(s) => SLMNode::NString(*s.clone()),
            }, None, None);
        }
        map
    }

    fn node(&mut self, columns: bool, module: bool) -> SourceNode {
        SourceNode::new(None, None, None, Some(SMNode::NNodeVec(
            self.children.iter_mut().map(|child| {
                SMNode::NSourceNode(
                    match child {
                        Source::Raw(s) => s.node(columns, module),
                        Source::Original(s) => s.node(columns, module),
                        Source::Replace(s) => s.node(columns, module),
                        Source::Prefix(s) => s.node(columns, module),
                        Source::Concat(s) => s.node(columns, module),
                        Source::LineToLineMapped(s) => s.node(columns, module),
                        Source::SString(_) => panic!(),
                    }
                )
            }).collect()
        )))
    }
}
