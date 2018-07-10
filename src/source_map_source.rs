use source::SourceTrait;
use source_list_map;
use source_list_map::{types::GenCode, types::Node as SlmNode, SourceListMap};
use source_map;
use source_map::SourceNode;
use std::rc::Rc;
use types::StringPtr;

#[derive(Debug)]
pub struct SourceMapSource {
    value: Rc<String>,
    value_idx: i32,
    name: i32,
    map_sources: Vec<i32>,
    map_sources_content: Vec<i32>,
    map_names: Vec<i32>,
    map_mappings: Rc<String>,
}

impl SourceMapSource {
    pub fn new(
        value: String,
        value_idx: i32,
        name: i32,
        map_sources: Vec<i32>,
        map_sources_content: Vec<i32>,
        map_mappings: String,
        map_names: Vec<i32>,
    ) -> SourceMapSource {
        SourceMapSource {
            value: Rc::new(value),
            value_idx,
            name,
            map_sources: map_sources,
            map_sources_content: map_sources_content,
            map_names: map_names,
            map_mappings: Rc::new(map_mappings),
        }
    }
}

impl SourceTrait for SourceMapSource {
    fn source(&mut self) -> Rc<String> {
        self.value.clone()
    }

    fn size(&mut self) -> usize {
        self.value.len()
    }

    fn node(&mut self, _columns: bool, _module: bool) -> SourceNode {
        source_map::from_string_with_source_map(
            StringPtr::Ptr(self.value.clone()),
            self.map_sources.clone(),
            self.map_sources_content.clone(),
            StringPtr::Ptr(self.map_mappings.clone()),
            self.map_names.clone(),
            None,
            None,
        )
    }

    fn list_map(&mut self, _columns: bool, module: bool) -> SourceListMap {
        if !module {
            SourceListMap::new(
                Some(GenCode::Code(SlmNode::NRcString(self.value.clone()))),
                Some(self.name.clone()),
                Some(self.value_idx.clone()),
            )
        } else {
            source_list_map::from_string_with_source_map(
                StringPtr::Ptr(self.value.clone()),
                self.map_sources.clone(),
                self.map_sources_content.clone(),
                StringPtr::Ptr(self.map_mappings.clone()),
            )
        }
    }
}
