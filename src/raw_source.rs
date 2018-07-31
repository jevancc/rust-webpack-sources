use source::SourceTrait;
use source_list_map::{types::GenCode, types::Node as SlmNode, SourceListMap};
use source_map::{types::Node as SmNode, SourceNode};
use types::string_slice::*;
use types::string_cat::*;

#[derive(Debug)]
pub struct RawSource {
    value: StringSlice,
}

impl RawSource {
    pub fn new(value: String) -> RawSource {
        RawSource {
            value: StringSlice::from(value),
        }
    }
}

impl SourceTrait for RawSource {
    fn source(&mut self) -> StringCat {
        StringCat::from(&self.value)
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
            None,
            None,
        )
    }
}
