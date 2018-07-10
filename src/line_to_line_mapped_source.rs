use source::SourceTrait;
use source_list_map::{types::GenCode, types::Node as SlmNode, SourceListMap};
use source_map::{types::Node as SmNode, SourceNode};
use std::rc::Rc;
use types::StringPtr;

#[derive(Debug)]
pub struct LineToLineMappedSource {
    value: Rc<String>,
    name: i32,
    original_source: i32,
}

impl LineToLineMappedSource {
    pub fn new(value: String, name: i32, original_source: i32) -> LineToLineMappedSource {
        LineToLineMappedSource {
            value: Rc::new(value),
            name,
            original_source,
        }
    }
}

impl SourceTrait for LineToLineMappedSource {
    fn source(&mut self) -> Rc<String> {
        self.value.clone()
    }

    fn size(&mut self) -> usize {
        self.value.len()
    }

    fn node(&mut self, _columns: bool, _module: bool) -> SourceNode {
        let mut lines = self.value.split('\n').enumerate().peekable();
        let mut chunks = Vec::<SmNode>::new();
        while let Some((idx, line)) = lines.next() {
            let line = String::from(line) + if lines.peek().is_none() { "\n" } else { "" };
            chunks.push(SmNode::NSourceNode(SourceNode::new(
                Some((idx + 1, 0)),
                Some(self.name.clone()),
                None,
                Some(SmNode::NString(line)),
            )));
        }
        let mut node = SourceNode::new(None, None, None, Some(SmNode::NNodeVec(chunks)));
        node.set_source_content(self.name.clone(), self.original_source.clone());
        node
    }

    fn list_map(&mut self, _columns: bool, _module: bool) -> SourceListMap {
        SourceListMap::new(
            Some(GenCode::Code(SlmNode::NRcString(self.value.clone()))),
            Some(self.name.clone()),
            Some(self.original_source.clone()),
        )
    }
}
