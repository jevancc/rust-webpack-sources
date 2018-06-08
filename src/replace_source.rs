use serde_json;
use source_list_map::*;
use std::cmp;

pub struct ReplaceSource {
    // pub source: Source,   // stored in JS
    // pub name: String,     // stored in JS
    pub replacements: Vec<(i32, i32, String, usize)>,
    is_sorted: bool,
}

impl ReplaceSource {
    pub fn new() -> ReplaceSource {
        ReplaceSource {
            replacements: Vec::new(),
            is_sorted: true,
        }
    }

    pub fn replace(&mut self, start: i32, end: i32, new_value: String) {
        let len = self.replacements.len();
        self.replacements.push((start, end, new_value, len));
        self.is_sorted = false;
    }

    pub fn insert(&mut self, pos: i32, new_value: String) {
        let len = self.replacements.len();
        self.replacements.push((pos, pos - 1, new_value, len));
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

    pub fn replace_string(&mut self, s: &str) -> String {
        let mut results: Vec<&str> = vec![s];

        self.sort_replacements();
        for repl in &self.replacements {
            let rem_source = results.pop().unwrap();
            let splitted1 = split_string(rem_source, repl.1 + 1);
            let splitted2 = split_string(splitted1.0, repl.0);
            results.push(splitted1.1);
            results.push(&repl.2);
            results.push(splitted2.0);
        }
        results.reverse();
        results.join("")
    }

    pub fn list_map(&mut self, map: SourceListMap) -> SourceListMap {
        let mut mf = ReplaceMappingFunction::new(&self.replacements);
        let mut map = map.map_generated_code(&mut mf);

        let mut extra_code = String::new();
        while mf.replacement_idx >= 0 {
            extra_code += &self.replacements[mf.replacement_idx as usize].2;
            mf.replacement_idx -= 1;
        }

        if !extra_code.is_empty() {
            map.add(Node::NString(extra_code), None, None);
        }
        map
    }

    pub fn replacements_to_string(&mut self) -> String {
        self.sort_replacements();
        serde_json::to_string(&self.replacements).unwrap()
    }
}

struct ReplaceMappingFunction<'a> {
    pub current_idx: i32,
    pub replacement_idx: i32,
    pub remove_chars: i32,
    pub replacements: &'a Vec<(i32, i32, String, usize)>,
}

impl<'a> ReplaceMappingFunction<'a> {
    pub fn new(replacements: &'a Vec<(i32, i32, String, usize)>) -> ReplaceMappingFunction {
        ReplaceMappingFunction {
            current_idx: 0,
            replacement_idx: replacements.len() as i32 - 1,
            remove_chars: 0,
            replacements,
        }
    }
}

impl<'a> MappingFunction for ReplaceMappingFunction<'a> {
    // TODO: Enhance performance
    fn map(&mut self, mut code: String) -> String {
        let code_len = code.len() as i32;
        let new_current_idx = self.current_idx + code_len;

        if self.remove_chars > code_len {
            self.remove_chars -= code_len;
            self.current_idx = new_current_idx;
            String::new()
        } else {
            if self.remove_chars > 0 {
                code = if self.remove_chars as usize >= code.len() {
                    String::new()
                } else {
                    code.split_off(self.remove_chars as usize)
                };
                self.current_idx += self.remove_chars;
                self.remove_chars = 0;
            }
            let mut final_str = String::new();
            while self.replacement_idx >= 0
                && self.replacements[self.replacement_idx as usize].0 < new_current_idx
            {
                let repl = &self.replacements[self.replacement_idx as usize];
                let start = repl.0;
                let end = repl.1 + 1;
                let mut before;
                if start - self.current_idx <= 0 {
                    before = String::new();
                } else if (start - self.current_idx) as usize >= code.len() {
                    before = code.clone();
                } else {
                    before = code.clone();
                    before.split_off((start - self.current_idx) as usize);
                }

                final_str += &(before + &repl.2);
                if end <= new_current_idx {
                    code = if end - self.current_idx <= 0 {
                        code
                    } else if (end - self.current_idx) as usize >= code.len() {
                        String::new()
                    } else {
                        code.split_off((end - self.current_idx) as usize)
                    };

                    self.current_idx = cmp::max(self.current_idx, end);
                } else {
                    code = String::new();
                    self.remove_chars = end - new_current_idx;
                }

                self.replacement_idx -= 1;
            }
            self.current_idx = new_current_idx;
            final_str + &code
        }
    }
}

#[inline]
fn split_string(s: &str, pos: i32) -> (&str, &str) {
    if pos <= 0 {
        ("", s)
    } else if pos as usize >= s.len() {
        (s, "")
    } else {
        s.split_at(pos as usize)
    }
}
