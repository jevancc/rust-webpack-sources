use source::SourceTrait;
use source_list_map;
use source_list_map::{types::GenCode, types::Node as SlmNode, SourceListMap};
use source_map;
use source_map::{SourceMapGenerator, SourceNode};
use types::string_slice::*;

#[derive(Debug)]
pub struct SourceMapSource {
    value: StringSlice,
    value_idx: i32,
    name: i32,
    map_sources: Vec<i32>,
    map_sources_content: Vec<i32>,
    map_names: Vec<i32>,
    map_mappings: StringSlice,
    map_generator: Option<SourceMapGenerator>,

    original_source: Option<i32>,

    innermap_sources: Option<Vec<i32>>,
    innermap_sources_content: Option<Vec<i32>>,
    innermap_names: Option<Vec<i32>>,
    innermap_mappings: Option<StringSlice>,
    innermap_generator: Option<SourceMapGenerator>,
    innermap_applied: bool,
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
        map_generator: Option<SourceMapGenerator>,
    ) -> SourceMapSource {
        SourceMapSource {
            value: StringSlice::from(value),
            value_idx,
            name,
            map_sources,
            map_sources_content,
            map_names,
            map_mappings: StringSlice::from(map_mappings),
            map_generator,
            original_source: None,
            innermap_sources: None,
            innermap_sources_content: None,
            innermap_names: None,
            innermap_mappings: None,
            innermap_generator: None,
            innermap_applied: false,
        }
    }

    pub fn new_with_generator(
        value: String,
        value_idx: i32,
        name: i32,
        mut map_generator: SourceMapGenerator,
    ) -> SourceMapSource {
        let map = map_generator.to_source_map();
        SourceMapSource {
            value: StringSlice::from(value),
            value_idx,
            name,
            map_sources: map.sources,
            map_sources_content: map.sources_content,
            map_names: map.names,
            map_mappings: StringSlice::from(map.mappings),
            map_generator: Some(map_generator),
            original_source: None,
            innermap_sources: None,
            innermap_sources_content: None,
            innermap_names: None,
            innermap_mappings: None,
            innermap_generator: None,
            innermap_applied: false,
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
        map_generator: Option<SourceMapGenerator>,
    ) {
        self.innermap_sources = Some(map_sources);
        self.innermap_sources_content = Some(map_sources_content);
        self.innermap_mappings = Some(StringSlice::from(map_mappings));
        self.innermap_names = Some(map_names);
        self.innermap_generator = map_generator;
        self.innermap_applied = false;
    }

    pub fn set_inner_source_map_generator(&mut self, mut map_generator: SourceMapGenerator) {
        let map = map_generator.to_source_map();
        self.innermap_sources = Some(map.sources);
        self.innermap_sources_content = Some(map.sources_content);
        self.innermap_mappings = Some(StringSlice::from(map.mappings));
        self.innermap_names = Some(map.names);
        self.innermap_generator = Some(map_generator);
        self.innermap_applied = false;
    }
}

impl SourceTrait for SourceMapSource {
    fn source(&mut self) -> StringSlice {
        self.value.clone()
    }

    fn size(&mut self) -> usize {
        self.value.len()
    }

    fn node(&mut self, _columns: bool, _module: bool) -> SourceNode {
        let code = self.value.clone();
        if self.map_generator.is_none() {
            self.map_generator = Some(SourceMapGenerator::from_source_map(
                self.map_sources.clone(),
                self.map_sources_content.clone(),
                self.map_mappings.clone(),
                self.map_names.clone(),
                None,
                None,
                true,
            ));
        }

        let generator = self.map_generator.as_mut().unwrap();
        if self.innermap_mappings.is_some() {
            if self.innermap_generator.is_none() {
                self.innermap_generator = Some(SourceMapGenerator::from_source_map(
                    self.innermap_sources.clone().unwrap(),
                    self.innermap_sources_content.clone().unwrap(),
                    self.innermap_mappings.clone().unwrap(),
                    self.innermap_names.clone().unwrap(),
                    None,
                    None,
                    true,
                ));
            }
            let inner_generator = self.innermap_generator.as_mut().unwrap();
            if self.original_source.is_some() {
                generator.set_source_content(self.name, self.original_source);
            }
            if !self.innermap_applied {
                self.innermap_applied = true;
                generator.apply_source_map_generator(inner_generator, Some(self.name));
            }
        }
        source_map::from_string_with_source_map_generator(code, generator)
    }

    fn list_map(&mut self, _columns: bool, module: bool) -> SourceListMap {
        if !module {
            SourceListMap::new(
                Some(GenCode::Code(SlmNode::NString(self.value.clone()))),
                Some(self.name.clone()),
                Some(self.value_idx.clone()),
            )
        } else {
            source_list_map::from_string_with_source_map(
                self.value.clone(),
                self.map_sources.clone(),
                self.map_sources_content.clone(),
                self.map_mappings.clone(),
            )
        }
    }
}
