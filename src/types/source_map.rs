use source_map::SourceMapGenerator;

#[derive(Debug, PartialEq)]
pub struct StringWithSourceMap {
    pub source: String,
    pub map: SourceMap,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceMap {
    pub version: i32,
    pub file: Option<i32>,
    pub source_root: Option<String>,
    pub sources: Vec<i32>,
    pub sources_content: Vec<i32>,
    pub names: Vec<i32>,
    pub mappings: String,
}

#[derive(Debug)]
pub struct StringWithSourceMapGenerator {
    pub source: String,
    pub generator: SourceMapGenerator,
}
