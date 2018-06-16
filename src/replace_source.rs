use serde_json;
use source_list_map::*;
use std::cmp;

pub struct ReplaceSource {
    // pub source: Source,   // stored in JS
    // pub name: String,     // stored in JS
    pub replacements: Vec<(i64, i64, String, usize)>,
    is_sorted: bool,
}

impl ReplaceSource {
    pub fn new() -> ReplaceSource {
        ReplaceSource {
            replacements: Vec::new(),
            is_sorted: true,
        }
    }

    pub fn replace(&mut self, start: i32, end: i32, new_value: String, ord_s: i32, ord_e: i32) {
        let len = self.replacements.len();
        let start = ((start as i64) << 4) + ord_s as i64;
        let end = ((end as i64) << 4) + ord_e as i64;
        self.replacements.push((start, end, new_value, len));
        self.is_sorted = false;
    }

    pub fn insert(&mut self, pos: i32, new_value: String, ord: i32) {
        let len = self.replacements.len();
        let pos_s = ((pos as i64) << 4) + ord as i64;
        let pos_e = (((pos - 1) as i64) << 4) + ord as i64;
        self.replacements.push((pos_s, pos_e, new_value, len));
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
            let splitted1 = split_string(rem_source, (repl.1 >> 4) as i32 + 1);
            let splitted2 = split_string(splitted1.0, (repl.0 >> 4) as i32);
            results.push(splitted1.1);
            results.push(&repl.2);
            results.push(splitted2.0);
        }
        results.reverse();
        results.join("")
    }

    pub fn list_map(&mut self, map: SourceListMap) -> SourceListMap {
        self.sort_replacements();
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
        let repls: Vec<(i64, i64, &str, usize)> = self
            .replacements
            .iter()
            .map(|x| (x.0 >> 4, x.1 >> 4, x.2.as_str(), x.3))
            .collect();
        serde_json::to_string(&repls).unwrap()
    }
}

pub struct ReplaceMappingFunction<'a> {
    pub current_idx: i32,
    pub replacement_idx: i32,
    pub remove_chars: i32,
    pub replacements: &'a Vec<(i64, i64, String, usize)>,
}

impl<'a> ReplaceMappingFunction<'a> {
    pub fn new(replacements: &'a Vec<(i64, i64, String, usize)>) -> ReplaceMappingFunction {
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
        let code_len = code.chars().count() as i32;
        let new_current_idx = self.current_idx + code_len as i32;

        if self.remove_chars > code_len {
            self.remove_chars -= code_len;
            self.current_idx = new_current_idx;
            String::new()
        } else {
            if self.remove_chars > 0 {
                if self.remove_chars >= code_len {
                    code = String::new()
                } else {
                    code = code.chars().skip(self.remove_chars as usize).collect()
                };
                self.current_idx += self.remove_chars;
                self.remove_chars = 0;
            }
            let mut final_str = String::new();
            while self.replacement_idx >= 0
                && self.replacements[self.replacement_idx as usize].0
                    < ((new_current_idx as i64) << 4)
            {
                let repl = &self.replacements[self.replacement_idx as usize];
                let start = (repl.0 >> 4) as i32;
                let end = (repl.1 >> 4) as i32 + 1;
                let before: String;
                if start - self.current_idx <= 0 {
                    before = String::new();
                } else if start - self.current_idx >= code_len {
                    before = code.clone();
                } else {
                    before = code
                        .chars()
                        .take((start - self.current_idx) as usize)
                        .collect();
                }

                final_str += &(before + &repl.2);
                if end <= new_current_idx {
                    if end - self.current_idx >= code_len {
                        code = String::new()
                    } else if end - self.current_idx > 0 {
                        code = code
                            .chars()
                            .skip((end - self.current_idx) as usize)
                            .collect();
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
    } else if pos >= s.chars().count() as i32 {
        (s, "")
    } else {
        let pos = s.char_indices().skip(pos as usize).next().unwrap().0;
        s.split_at(pos)
    }
}
