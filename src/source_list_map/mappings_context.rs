use super::types::Node;
use linked_hash_map::LinkedHashMap;

#[derive(Debug)]
pub struct MappingsContext {
    pub sources: LinkedHashMap<Option<i32>, (usize, Option<Node>)>,
    pub has_source_content: bool,
    pub current_original_line: usize,
    pub current_source: usize,
    pub unfinished_generated_line: usize,
}

impl MappingsContext {
    pub fn new() -> Self {
        MappingsContext {
            sources: LinkedHashMap::new(),
            has_source_content: false,
            current_original_line: 1,
            current_source: 0,
            unfinished_generated_line: 0,
        }
    }

    pub fn ensure_source(&mut self, source: Option<i32>, original_source: Option<Node>) -> usize {
        if self.sources.contains_key(&source) {
            self.sources.get(&source).unwrap().0
        } else {
            let sources_indices_len = self.sources.len();
            match original_source {
                Some(Node::NString(_)) => {
                    self.has_source_content = true;
                }
                Some(Node::NStringIdx(_)) => {
                    self.has_source_content = true;
                }
                _ => {}
            }

            self.sources
                .insert(source, (sources_indices_len, original_source));
            sources_indices_len
        }
    }

    pub fn get_arrays(&self) -> (Vec<Option<i32>>, Vec<i32>) {
        let mut sources: Vec<Option<i32>> = Vec::new();
        let mut sources_content: Vec<i32> = Vec::new();
        for (key, val) in &self.sources {
            sources.push(key.clone());
            if let Some(Node::NStringIdx(idx)) = val.1 {
                sources_content.push(idx);
            }
        }
        (sources, sources_content)
    }
}
