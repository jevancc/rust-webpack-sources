use source::SourceTrait;
use source_list_map;
use source_list_map::{types::GenCode, types::Node as SlmNode, SourceListMap};
use source_map;
use source_map::{SourceMapGenerator, SourceNode};
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

    original_source: Option<i32>,

    innermap_sources: Option<Vec<i32>>,
    innermap_sources_content: Option<Vec<i32>>,
    innermap_names: Option<Vec<i32>>,
    innermap_mappings: Option<Rc<String>>,
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
            original_source: None,
            innermap_sources: None,
            innermap_sources_content: None,
            innermap_names: None,
            innermap_mappings: None,
        }
    }

    pub fn set_original_source(&mut self, source: i32) {
        self.original_source = Some(source);
    }

    pub fn set_inner_source_map(
        &mut self,
        map_sources: Vec<i32>,
        map_sources_content: Vec<i32>,
        map_mappings: String,
        map_names: Vec<i32>,
    ) {
        self.innermap_sources = Some(map_sources);
        self.innermap_sources_content = Some(map_sources_content);
        self.innermap_mappings = Some(Rc::new(map_mappings));
        self.innermap_names = Some(map_names);
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
        let code = StringPtr::Ptr(self.value.clone());
        let mut generator = SourceMapGenerator::from_source_map(
            self.map_sources.clone(),
            self.map_sources_content.clone(),
            StringPtr::Ptr(self.map_mappings.clone()),
            self.map_names.clone(),
            None,
            None,
            true,
        );
        if self.innermap_mappings.is_some() {
            let inner_generator = SourceMapGenerator::from_source_map(
                self.innermap_sources.clone().unwrap(),
                self.innermap_sources_content.clone().unwrap(),
                StringPtr::Ptr(self.innermap_mappings.clone().unwrap()),
                self.innermap_names.clone().unwrap(),
                None,
                None,
                true,
            );
            if self.original_source.is_some() {
                generator.set_source_content(self.name, self.original_source);
            }
            generator.apply_source_map_generator(inner_generator, Some(self.name));
        }
        source_map::from_string_with_source_map_generator(code, generator)
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
