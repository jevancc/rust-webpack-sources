use source::SourceTrait;
use source_list_map::{types::GenCode, types::Node as SlmNode, SourceListMap};
use source_map::{types::Node as SmNode, SourceNode};
use types::string_slice::*;
use types::string_cat::*;

#[derive(Debug)]
pub struct LineToLineMappedSource {
    value: StringSlice,
    name: i32,
    original_source: i32,
}

impl LineToLineMappedSource {
    pub fn new(value: String, name: i32, original_source: i32) -> LineToLineMappedSource {
        LineToLineMappedSource {
            value: StringSlice::from(value),
            name,
            original_source,
        }
    }
}

impl SourceTrait for LineToLineMappedSource {
    fn source(&mut self) -> StringCat {
        StringCat::from(&self.value)
    }

    fn size(&mut self) -> usize {
        self.value.len()
    }

    fn node(&mut self, _columns: bool, _module: bool) -> SourceNode {
        let mut chunks = Vec::<SmNode>::with_capacity(16);

        for (i, line) in self.value.split_keep_seperator('\n').enumerate() {
            chunks.push(SmNode::NSourceNode(SourceNode::new(
                Some((i + 1, 0)),
                Some(self.name.clone()),
                None,
                Some(SmNode::NString(line)),
            )));
        }

        let mut node = SourceNode::new(None, None, None, Some(SmNode::NNodeVec(chunks)));
        node.set_source_content(self.name, self.original_source);
        node
    }

    fn list_map(&mut self, _columns: bool, _module: bool) -> SourceListMap {
        SourceListMap::new(
            Some(GenCode::Code(SlmNode::NString(self.value.clone()))),
            Some(self.name),
            Some(self.original_source),
        )
    }
}
