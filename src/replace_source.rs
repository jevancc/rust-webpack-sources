use source::{Source, SourceTrait};
use source_list_map::{types::Node as SlmNode, MappingFunction, SourceListMap};
use source_map::{types::Node as SmNode, SourceNode, WalkFunction};
use std::cmp;
use std::rc::Rc;
use std::str;
use types::string_slice::*;
use utils;

#[derive(Debug)]
pub struct ReplaceSource {
    pub source: Source,
    // pub name: String,     // stored in JS
    replacements: Vec<(i64, i64, Rc<String>, usize)>,
    is_sorted: bool,
}

impl ReplaceSource {
    pub fn new(source: Source) -> ReplaceSource {
        ReplaceSource {
            source,
            replacements: Vec::new(),
            is_sorted: true,
        }
    }

    pub fn replace(&mut self, start: i32, end: i32, new_value: String, ord_s: i32, ord_e: i32) {
        let len = self.replacements.len();
        let start = ((start as i64) << 4) + ord_s as i64;
        let end = ((end as i64) << 4) + ord_e as i64;
        self.replacements
            .push((start, end, Rc::new(new_value), len));
        self.is_sorted = false;
    }

    pub fn insert(&mut self, pos: i32, new_value: String, ord: i32) {
        let len = self.replacements.len();
        let pos_s = ((pos as i64) << 4) + ord as i64;
        let pos_e = (((pos - 1) as i64) << 4) + ord as i64;
        self.replacements
            .push((pos_s, pos_e, Rc::new(new_value), len));
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
        let mut results: Vec<(&str, bool)> = vec![(s, false)];

        self.sort_replacements();
        for repl in &self.replacements {
            let rem_source = results.pop().unwrap();
            let splitted1 =
                match utils::split_str(rem_source.0, (repl.1 >> 4) as i32 + 1, rem_source.1) {
                    Ok(sp) => sp,
                    Err((_, s, sbc)) => (s, "", sbc, true),
                };
            let splitted2 = match utils::split_str(splitted1.0, (repl.0 >> 4) as i32, splitted1.2) {
                Ok(sp) => sp,
                Err((_, s, sbc)) => (s, "", sbc, true),
            };
            results.push((splitted1.1, splitted1.3));
            results.push((&repl.2, false));
            results.push((splitted2.0, splitted2.2));
        }
        let mut result_string = Vec::<u8>::with_capacity(s.len() * 2);
        for (s, _) in results.iter().rev() {
            result_string.extend_from_slice(&s.as_bytes());
        }
        unsafe { str::from_utf8_unchecked(&result_string).to_string() }
    }

}

impl SourceTrait for ReplaceSource {
    fn source(&mut self) -> StringSlice {
        let s = self.source.source();
        StringSlice::from(self.replace_string(&s))
    }

    fn node(&mut self, columns: bool, module: bool) -> SourceNode {
        self.sort_replacements();
        // WIP
    }

    fn list_map(&mut self, columns: bool, module: bool) -> SourceListMap {
        self.sort_replacements();
        let mut mf = ReplaceMappingFunction::new(&self.replacements);
        let map = self.source.list_map(columns, module);
        let mut map = map.map_generated_code(&mut mf);

        let mut extra_code = String::with_capacity(80);
        while mf.replacement_idx >= 0 {
            extra_code.push_str(&self.replacements[mf.replacement_idx as usize].2);
            mf.replacement_idx -= 1;
        }

        if !extra_code.is_empty() {
            map.add(SlmNode::NString(StringSlice::from(extra_code)), None, None);
        }
        map
    }
}

struct ReplaceMappingFunction<'a> {
    pub current_idx: i32,
    pub replacement_idx: i32,
    pub remove_chars: i32,
    pub replacements: &'a Vec<(i64, i64, Rc<String>, usize)>,
}

impl<'a> ReplaceMappingFunction<'a> {
    pub fn new(replacements: &'a Vec<(i64, i64, Rc<String>, usize)>) -> ReplaceMappingFunction {
        ReplaceMappingFunction {
            current_idx: 0,
            replacement_idx: replacements.len() as i32 - 1,
            remove_chars: 0,
            replacements,
        }
    }
}

impl<'a> MappingFunction for ReplaceMappingFunction<'a> {
    fn map(&mut self, code: String) -> String {
        let code_len = code.len();

        let mut code_iter_bound = 0;
        let mut code_iter = code.char_indices();
        let mut code_step_n_chars = |n: i32| {
            let mut rem_cnt = 0;
            while rem_cnt < n {
                if let Some((p, c)) = code_iter.next() {
                    code_iter_bound = p + c.len_utf8();
                    rem_cnt += 1;
                } else {
                    return Err((n - rem_cnt, rem_cnt));
                }
            }
            Ok((code_iter_bound, rem_cnt))
        };

        let mut start_bound = 0;
        if self.remove_chars > 0 {
            match code_step_n_chars(self.remove_chars) {
                Ok((p, chars)) => {
                    self.current_idx += chars;
                    self.remove_chars = 0;
                    start_bound = p;
                }
                Err((n, chars)) => {
                    self.current_idx += chars;
                    self.remove_chars = n;
                    return String::new();
                }
            }
        }

        let mut final_str = String::with_capacity(code_len * 2);
        while self.replacement_idx >= 0 {
            let repl = &self.replacements[self.replacement_idx as usize];
            let start = (repl.0 >> 4) as i32;
            let end = (repl.1 >> 4) as i32 + 1;

            match code_step_n_chars(start - self.current_idx) {
                Ok((p, chars)) => {
                    self.current_idx += chars;
                    final_str.push_str(&code[start_bound as usize..p as usize]);
                }
                Err((_, chars)) => {
                    self.current_idx += chars;
                    break;
                }
            }
            final_str.push_str(&repl.2);

            self.replacement_idx -= 1;
            match code_step_n_chars(end - self.current_idx) {
                Ok((p, chars)) => {
                    self.current_idx += chars;
                    start_bound = p;
                }
                Err((n, chars)) => {
                    self.current_idx += chars;
                    self.remove_chars = cmp::max(n, 0);
                    start_bound = code_len;
                    break;
                }
            }
        }
        final_str.push_str(&code[start_bound as usize..]);
        final_str
    }
}

struct ReplaceWalkFunction<'a> {
    pub node: SourceNode,
    pub replacements: &'a Vec<(i64, i64, Rc<String>, usize)>,
    pub position: i32,
}

impl<'a> ReplaceWalkFunction<'a> {
    pub fn new(replacements: &'a Vec<(i64, i64, Rc<String>, usize)>) -> Self {
        ReplaceWalkFunction {
            node: SourceNode::new(None, None, None, None),
            replacements,
            position: 0,
        }
    }
}

impl<'a> WalkFunction for ReplaceWalkFunction<'a> {
    fn process_chunk(
        &mut self,
        chunk: &str,
        original_source: &Option<i32>,
        original_position: &Option<(usize, usize)>,
        original_name: &Option<i32>,
    ) {

    }
    fn process_source_content(&mut self, source: i32, source_content: i32) {
        self.node.set_source_content(source, source_content);
    }
}
