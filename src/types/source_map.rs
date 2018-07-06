#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StringWithSourceMap {
    pub source: String,
    pub map: SourceMap,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceMap {
    pub version: i32,
    pub file: Option<String>,
    pub source_root: Option<String>,
    pub sources: Vec<String>,
    pub sources_content: Vec<String>,
    pub names: Vec<String>,
    pub mappings: String,
}
