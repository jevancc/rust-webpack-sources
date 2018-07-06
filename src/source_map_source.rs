use source::{SourceTrait};
use source_map;
use source_map::{SourceNode};
use source_list_map;
use source_list_map::{SourceListMap, types::GenCode, types::Node as SlmNode};
use types::{StringPtr};
use std::rc::Rc;

#[derive(Debug)]
pub struct SourceMapSource {
    value: Rc<String>,
    name: Rc<String>,
    map_sources: Vec<Rc<String>>,
    map_sources_content: Vec<Rc<String>>,
    map_names: Vec<Rc<String>>,
    map_mappings: Rc<String>,
}

impl SourceMapSource {
    pub fn new(
            value: String,
            name: String,
            map_sources: Vec<String>,
            map_sources_content: Vec<String>,
            map_mappings: String,
            map_names: Vec<String>
        ) -> SourceMapSource {
        SourceMapSource {
            value: Rc::new(value),
            name: Rc::new(name),
            map_sources: map_sources.into_iter().map(|s| Rc::new(s)).collect(),
            map_sources_content: map_sources_content.into_iter().map(|s| Rc::new(s)).collect(),
            map_names: map_names.into_iter().map(|s| Rc::new(s)).collect(),
            map_mappings: Rc::new(map_mappings),
        }
    }
}

impl SourceTrait for SourceMapSource {
    fn source(&mut self) -> String {
        (*self.value).clone()
    }

    fn size(&mut self) -> usize {
        self.value.len()
    }

    fn node(&mut self, _columns: bool, _module: bool) -> SourceNode {
        source_map::from_string_with_source_map(
            StringPtr::Ptr(self.value.clone()),
            self.map_sources.iter().cloned().map(|sp| StringPtr::Ptr(sp)).collect(),
            self.map_sources_content.iter().cloned().map(|sp| StringPtr::Ptr(sp)).collect(),
            StringPtr::Ptr(self.map_mappings.clone()),
            self.map_names.iter().cloned().map(|sp| StringPtr::Ptr(sp)).collect(),
            None,
            None
        )
    }

    fn list_map(&mut self, _columns: bool, module: bool) -> SourceListMap {
        if !module {
            SourceListMap::new(
                Some(GenCode::Code(SlmNode::NRcString(self.value.clone()))),
                Some(StringPtr::Ptr(self.name.clone())),
                Some(StringPtr::Ptr(self.value.clone()))
            )
        } else {
            source_list_map::from_string_with_source_map(
                StringPtr::Ptr(self.value.clone()),
                self.map_sources.iter().cloned().map(|sp| StringPtr::Ptr(sp)).collect(),
                self.map_sources_content.iter().cloned().map(|sp| StringPtr::Ptr(sp)).collect(),
                StringPtr::Ptr(self.map_mappings.clone())
            )
        }
    }
}
