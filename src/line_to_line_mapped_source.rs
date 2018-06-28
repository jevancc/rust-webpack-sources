use source_map::{SourceNode, StringPtr as SMStringPtr, Node as SMNode};
use source_list_map::{SourceListMap, GenCode, StringPtr as SLMStringPtr, Node as SLMNode};
use source::{SourceTrait};

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
        let mut chunks = Vec::<SMNode>::new();
        while let Some((idx, line)) = lines.next() {
            let line = String::from(line) + if lines.peek().is_none() { "\n" } else { "" };
            chunks.push(SMNode::NSourceNode(SourceNode::new(
                Some((idx + 1, 0)),
                Some(SMStringPtr::Str(self.name.clone())),
                None,
                Some(SMNode::NString(line))
            )));
        }
        let mut node = SourceNode::new(None, None, None, Some(SMNode::NNodeVec(chunks)));
        node.set_source_content(
            SMStringPtr::Str(self.name.clone()),
            SMStringPtr::Str(self.original_source.clone())
        );
        node
    }

    fn list_map(&mut self, _columns: bool, _module: bool) -> SourceListMap {
        SourceListMap::new(
            Some(GenCode::Code(SLMNode::NString(self.value.clone()))),
            Some(SLMStringPtr::Str(self.name.clone())),
            Some(SLMStringPtr::Str(self.original_source.clone()))
        )
    }
}
