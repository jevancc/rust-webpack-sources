use source::{SourceTrait};
use source_map::{SourceNode};
use source_list_map::{SourceListMap, types::GenCode, from_string_with_source_map, types::Node as SlmNode};
use types::{StringPtr};

#[derive(Debug)]
pub struct SourceMapSource {
    value: String,
    name: String,
    source_map_consumer: String,
    sources: Vec<String>,
    sources_content: Vec<String>,
    mappings: String,
}

impl SourceMapSource {
    pub fn new(
            value: String,
            name: String,
            sources: Vec<String>,
            sources_content: Vec<String>,
            mappings: String
        ) -> SourceMapSource {
        SourceMapSource {
            value,
            name,
            source_map_consumer: String::new(),
            sources,
            sources_content,
            mappings,
        }
    }

    pub fn set_source_map_consumer(&mut self, json: String) {
        self.source_map_consumer = json;
    }
}

impl SourceTrait for SourceMapSource {
    fn source(&mut self) -> String {
        self.value.clone()
    }

    fn size(&mut self) -> usize {
        self.value.len()
    }

    fn node(&mut self, _columns: bool, _module: bool) -> SourceNode {
        SourceNode::from_string_with_source_map(&self.value, &self.source_map_consumer)
    }

    fn list_map(&mut self, _columns: bool, module: bool) -> SourceListMap {
        if !module {
            SourceListMap::new(
                Some(GenCode::Code(SlmNode::NString(self.value.clone()))),
                Some(StringPtr::Str(self.name.clone())),
                Some(StringPtr::Str(self.value.clone()))
            )
        } else {
            from_string_with_source_map(
                &self.value,
                &self.sources.iter().map(|s| s.as_ref()).collect(),
                &self.sources_content.iter().map(|s| s.as_ref()).collect(),
                &self.mappings
            )
        }
    }
}
