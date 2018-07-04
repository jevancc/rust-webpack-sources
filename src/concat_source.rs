use source::{Source, SourceTrait};
use source_map::{SourceNode, Node as SMNode};
use source_list_map::{SourceListMap, Node as SLMNode};
use wasm_api::clog;
use std::rc::Rc;

#[derive(Debug)]
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
        if let Source::Concat(cs) = item {
            self.children.append(&mut cs.borrow_mut().children);
        } else {
            self.children.push(item);
        }
    }
}

impl SourceTrait for ConcatSource {
    fn source(&mut self) -> String {
        let sources: Vec<String> = self.children.iter_mut().map(|child| {
            child.source()
        }).collect();
        sources.join("")
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
            map.add(if let Source::SString(s) = child {
                // TODO: Check why error occurs when returning SLMNode::NString
                SLMNode::NRcString(s.clone())
            } else {
                SLMNode::NSourceListMap(child.list_map(columns, module))
            }, None, None);
        }
        map
    }

    fn node(&mut self, columns: bool, module: bool) -> SourceNode {
        SourceNode::new(None, None, None, Some(SMNode::NNodeVec(
            self.children.iter_mut().map(|child| {
                if let Source::SString(s) = child {
                    // TODO: Check why error occurs when returning SMNode::NString
                    SMNode::NRcString(s.clone())
                } else {
                    SMNode::NSourceNode(child.node(columns, module))
                }
            }).collect()
        )))
    }
}
