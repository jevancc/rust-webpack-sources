use source::SourceTrait;
use source_list_map;
use source_list_map::{types::GenCode, types::Node as SlmNode, SourceListMap};
use source_map;
use source_map::{SourceMapGenerator, SourceNode};
use types::string_slice::*;

#[derive(Debug, Clone)]
struct SourceMapContent {
    pub sources: Vec<i32>,
    pub sources_content: Vec<i32>,
    pub names: Vec<i32>,
    pub mappings: StringSlice,
}

#[derive(Debug)]
struct SourceMap {
    pub data: Option<SourceMapContent>,
    pub generator: Option<SourceMapGenerator>,
}

impl SourceMap {
    pub fn new() -> Self {
        Self {
            data: None,
            generator: None,
        }
    }

    pub fn set_data(
        &mut self,
        sources: Vec<i32>,
        sources_content: Vec<i32>,
        mappings: String,
        names: Vec<i32>,
    ) {
        let data = SourceMapContent {
            sources,
            sources_content,
            mappings: StringSlice::from(mappings),
            names,
        };
        self.data = Some(data);
    }

    pub fn set_generator(&mut self, generator: SourceMapGenerator) {
        self.generator = Some(generator);
    }

    pub fn get_data(&mut self) -> &mut SourceMapContent {
        if self.data.is_none() {
            let map = self.generator.as_mut().unwrap().to_source_map();
            self.set_data(map.sources, map.sources_content, map.mappings, map.names);
        }
        self.data.as_mut().unwrap()
    }

    pub fn get_generator(&mut self) -> &mut SourceMapGenerator {
        if self.generator.is_none() {
            let data = self.data.clone().unwrap();
            self.set_generator(SourceMapGenerator::from_source_map(
                data.sources,
                data.sources_content,
                data.mappings,
                data.names,
                None,
                None,
                true,
            ));
        }
        self.generator.as_mut().unwrap()
    }
}

#[derive(Debug)]
pub struct SourceMapSource {
    value: StringSlice,
    value_idx: i32,
    name: i32,
    map: SourceMap,
    original_source: Option<i32>,
    innermap: Option<SourceMap>,
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
        let mut map = SourceMap::new();
        map.set_data(
            map_sources,
            map_sources_content,
            map_mappings,
            map_names,
        );
        if let Some(generator) = map_generator {
            map.set_generator(generator);
        }

        SourceMapSource {
            value: StringSlice::from(value),
            value_idx,
            name,
            map,
            original_source: None,
            innermap: None,
            innermap_applied: false,
        }
    }

    pub fn new_with_generator(
        value: String,
        value_idx: i32,
        name: i32,
        map_generator: SourceMapGenerator,
    ) -> SourceMapSource {
        let mut map = SourceMap::new();
        map.set_generator(map_generator);
        SourceMapSource {
            value: StringSlice::from(value),
            value_idx,
            name,
            map,
            original_source: None,
            innermap: None,
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
        let mut map = SourceMap::new();
        map.set_data(
            map_sources,
            map_sources_content,
            map_mappings,
            map_names,
        );
        if let Some(generator) = map_generator {
            map.set_generator(generator);
        }
        self.innermap = Some(map);
        self.innermap_applied = false;
    }

    pub fn set_inner_source_map_generator(&mut self, map_generator: SourceMapGenerator) {
        let mut map = SourceMap::new();
        map.set_generator(map_generator);
        self.innermap = Some(map);
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
        let generator = self.map.get_generator();
        if let Some(ref mut innermap) = self.innermap {
            let inner_generator = innermap.get_generator();
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
                Some(self.name),
                Some(self.value_idx),
            )
        } else {
            let map = self.map.get_data();
            source_list_map::from_string_with_source_map(
                self.value.clone(),
                map.sources.clone(),
                map.sources_content.clone(),
                map.mappings.clone(),
            )
        }
    }
}
