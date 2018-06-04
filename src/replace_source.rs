use SourceNode;
use source_list_map::*;

struct ReplaceMappingFunction<'a> {
    pub current_idx: usize,
    pub replacement_idx: usize,
    pub remove_chars: usize,
    pub replacements: &'a Vec<(usize, usize, String, usize)>,
}

impl<'a> ReplaceMappingFunction<'a> {
    pub fn new(replacements: &'a Vec<(usize, usize, String, usize)>) -> ReplaceMappingFunction {
        ReplaceMappingFunction {
            current_idx: 0,
            replacement_idx: replacements.len() - 1,
            remove_chars: 0,
            replacements
        }
    }
}

impl<'a> MappingFunction for ReplaceMappingFunction<'a> {
    fn map(&mut self, code: String) -> String {
        String::new()
    }
}

pub struct ReplaceSource {
    pub source: String,
    pub name: String,
    pub replacements: Vec<(usize, usize, String, usize)>,
    is_sorted: bool,
}

impl ReplaceSource {
    pub fn new(source: String, name: String) -> ReplaceSource {
        ReplaceSource {
            source,
            name,
            replacements: Vec::new(),
            is_sorted: true,
        }
    }

    pub fn replace(&mut self, start: usize, end: usize, new_value: String) {
        let len = self.replacements.len();
        self.replacements.push((start, end, new_value, len));
        self.is_sorted = false;
    }

    pub fn insert(&mut self, pos: usize, new_value: String) {
        let len = self.replacements.len();
        self.replacements.push((pos, pos.saturating_sub(1), new_value, len));
        self.is_sorted = false;
    }

    fn sort_replacements(&mut self) {
        if !self.is_sorted {
            self.is_sorted = true;
            self.replacements.sort_by(|a, b| {
                if a.1 != b.1 {
                    b.1.cmp(&a.1)
                } else if a.0 != b.0 {
                    b.0.cmp(&a.0)
                } else {
                    b.3.cmp(&a.3)
                }
            });
        }
    }

    fn replace_string(&self, s: &str) -> String {
        let mut results: Vec<&str> = vec![s];

        self.sort_replacements();
        for repl in &self.replacements {
            let rem_source = results.pop().unwrap();
            let splitted1 = rem_source.split_at(repl.1 + 1);
            let splitted2 = splitted1.0.split_at(repl.0);
            results.push(splitted1.1);
            results.push(&repl.2);
            results.push(splitted2.0);
        }
        results.reverse();
        results.join("")
    }

    // fn list_map(&mut self, map: SourceListMap) {
    //     let mut current_idx: usize = 0;
    //     let mut replacement_idx: usize = self.replacements.len() - 1;
    //     let mut remove_chars: usize = 0;
    //
    //     map = map.map_generated_code(
    //
    // }
}
