use source::SourceTrait;
use source_list_map::{types::GenCode, types::Node as SlmNode, SourceListMap};
use source_map::{types::Node as SmNode, SourceNode};
use types::StringPtr;

#[derive(Debug)]
pub struct LineToLineMappedSource {
    value: String,
    name: String,
    original_source: String,
}

impl LineToLineMappedSource {
    pub fn new(value: String, name: String, original_source: String) -> LineToLineMappedSource {
        LineToLineMappedSource {
            value,
            name,
            original_source,
        }
    }
}

impl SourceTrait for LineToLineMappedSource {
    fn source(&mut self) -> String {
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
                Some(StringPtr::Str(self.name.clone())),
                None,
                Some(SmNode::NString(line)),
            )));
        }
        let mut node = SourceNode::new(None, None, None, Some(SmNode::NNodeVec(chunks)));
        node.set_source_content(
            StringPtr::Str(self.name.clone()),
            StringPtr::Str(self.original_source.clone()),
        );
        node
    }

    fn list_map(&mut self, _columns: bool, _module: bool) -> SourceListMap {
        SourceListMap::new(
            Some(GenCode::Code(SlmNode::NString(self.value.clone()))),
            Some(StringPtr::Str(self.name.clone())),
            Some(StringPtr::Str(self.original_source.clone())),
        )
    }
}
