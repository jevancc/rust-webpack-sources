use source::{Source, SourceTrait};
use source_list_map::{types::Node as SlmNode, SourceListMap};
use source_map::{types::Node as SmNode, SourceNode};
use types::string_slice::*;
use types::string_cat::*;

#[derive(Debug)]
pub struct ConcatSource {
    children: Vec<Source>,
}

impl ConcatSource {
    pub fn new() -> ConcatSource {
        ConcatSource {
            children: Vec::new(),
        }
    }

    pub fn add(&mut self, item: Source) {
        if let Source::Concat(cs) = item {
            self.children.append(&mut cs.borrow().children.clone());
        } else {
            self.children.push(item);
        }
    }
}

impl SourceTrait for ConcatSource {
    fn source(&mut self) -> StringCat {
        let mut result = StringCat::default();
        for child in &mut self.children {
            result.cat(child.source());
        }
        result
    }

    fn size(&mut self) -> usize {
        let mut ret = 0;
        for child in &mut self.children {
            ret += child.size();
        }
        ret
    }

    fn list_map(&mut self, columns: bool, module: bool) -> SourceListMap {
        let mut map = SourceListMap::new(None, None, None);
        for child in &mut self.children {
            map.add(
                if let Source::SString(s) = child {
                    // TODO: Check why error occurs when returning SlmNode::NString
                    SlmNode::NString(s.clone())
                } else {
                    SlmNode::NSourceListMap(child.list_map(columns, module))
                },
                None,
                None,
            );
        }
        map
    }

    fn node(&mut self, columns: bool, module: bool) -> SourceNode {
        SourceNode::new(
            None,
            None,
            None,
            Some(SmNode::NNodeVec(
                self.children
                    .iter_mut()
                    .map(|child| {
                        if let Source::SString(s) = child {
                            SmNode::NString(s.clone())
                        } else {
                            SmNode::NSourceNode(child.node(columns, module))
                        }
                    })
                    .collect(),
            )),
        )
    }
}
