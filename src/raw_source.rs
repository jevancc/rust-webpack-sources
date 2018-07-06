use source_map::{SourceNode, types::Node as SmNode};
use source_list_map::{SourceListMap, types::GenCode, types::Node as SlmNode};
use source::{SourceTrait};

#[derive(Debug)]
pub struct RawSource {
    value: String,
}

impl RawSource {
    pub fn new(value: String) -> RawSource {
        RawSource {
            value
        }
    }
}

impl SourceTrait for RawSource {
    fn source(&mut self) -> String {
        self.value.clone()
    }

    fn size(&mut self) -> usize {
        self.value.len()
    }

    fn node(&mut self, _columns: bool, _module: bool) -> SourceNode {
        SourceNode::new(None, None, None, Some(SmNode::NString(self.value.clone())))
    }

    fn list_map(&mut self, _columns: bool, _module: bool) -> SourceListMap {
        SourceListMap::new(
            Some(GenCode::Code(SlmNode::NString(self.value.clone()))),
        None, None)
    }
}
