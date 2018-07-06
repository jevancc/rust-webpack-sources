#[derive(Deserialize)]
pub struct SourceMapConsumer {
    // (generatedLine, generatedColumn), source, name, (originalLine, originalColumn)
    pub mappings: Vec<((usize, usize), Option<String>, Option<String>, Option<(usize, usize)>)>,
    // source, sourceContent
    pub sources: Vec<(String, Option<String>)>,
}
