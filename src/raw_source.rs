use source::SourceTrait;
use source_list_map::{types::GenCode, types::Node as SlmNode, SourceListMap};
use source_map::{types::Node as SmNode, SourceNode};
use std::rc::Rc;

#[derive(Debug)]
pub struct RawSource {
    value: Rc<String>,
}

impl RawSource {
    pub fn new(value: String) -> RawSource {
        RawSource { value: Rc::new(value) }
    }
}

impl SourceTrait for RawSource {
    fn source(&mut self) -> Rc<String> {
        self.value.clone()
    }

    fn size(&mut self) -> usize {
        self.value.len()
    }

    fn node(&mut self, _columns: bool, _module: bool) -> SourceNode {
        SourceNode::new(None, None, None, Some(SmNode::NRcString(self.value.clone())))
    }

    fn list_map(&mut self, _columns: bool, _module: bool) -> SourceListMap {
        SourceListMap::new(
            Some(GenCode::Code(SlmNode::NRcString(self.value.clone()))),
            None,
            None,
        )
    }
}
