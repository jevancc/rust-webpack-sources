use source::{Source, SourceTrait};
use source_list_map::{types::Node as SlmNode, MappingFunction, SourceListMap};
use source_map::{types::Node as SmNode, SourceNode};
use std::cmp;
use std::rc::Rc;
use types::StringPtr;

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
        self.replacements.push((start, end, Rc::new(new_value), len));
        self.is_sorted = false;
    }

    pub fn insert(&mut self, pos: i32, new_value: String, ord: i32) {
        let len = self.replacements.len();
        let pos_s = ((pos as i64) << 4) + ord as i64;
        let pos_e = (((pos - 1) as i64) << 4) + ord as i64;
        self.replacements.push((pos_s, pos_e, Rc::new(new_value), len));
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
            let splitted1 = split_string(rem_source, (repl.1 >> 4) as i32 + 1, None);
            let splitted2 = split_string(splitted1.0, (repl.0 >> 4) as i32, None);
            results.push(splitted1.1);
            results.push(&repl.2);
            results.push(splitted2.0);
        }
        results.reverse();
        results.join("")
    }

    // pub fn replacements_to_string(&mut self) -> String {
    //     self.sort_replacements();
    //     let repls: Vec<(i64, i64, &str, usize)> = self
    //         .replacements
    //         .iter()
    //         .map(|x| (x.0 >> 4, x.1 >> 4, x.2.as_str(), x.3))
    //         .collect();
    //     serde_json::to_string(&repls).unwrap()
    // }
}

impl SourceTrait for ReplaceSource {
    fn source(&mut self) -> Rc<String> {
        let s = self.source.source();
        Rc::new(self.replace_string(&s))
    }

    fn node(&mut self, columns: bool, module: bool) -> SourceNode {
        self.sort_replacements();
        let mut result = Vec::<SmNode>::new();
        result.push(SmNode::NSourceNode(self.source.node(columns, module)));
        for repl in &self.replacements {
            let rem_source = result.pop().unwrap();
            match split_sourcenode(rem_source, (repl.1 >> 4) as i32 + 1) {
                Ok((l1, r1)) => match split_sourcenode(l1, (repl.0 >> 4) as i32) {
                    Ok((l2, r2)) => {
                        result.push(r1);
                        result.push(replacement_to_sourcenode(r2, repl.2.clone()));
                        result.push(l2);
                    }
                    Err((_, l1)) => {
                        result.push(r1.clone());
                        result.push(replacement_to_sourcenode(r1, repl.2.clone()));
                        result.push(l1);
                    }
                },
                Err((_, rem_source)) => match split_sourcenode(rem_source, (repl.0 >> 4) as i32) {
                    Ok((l2, r2)) => {
                        result.push(replacement_to_sourcenode(r2, repl.2.clone()));
                        result.push(l2);
                    }
                    Err((_, rem_source)) => {
                        result.push(SmNode::NRcString(repl.2.clone()));
                        result.push(rem_source);
                    }
                },
            }
        }
        result.reverse();
        SourceNode::new(None, None, None, Some(SmNode::NNodeVec(result)))
    }

    fn list_map(&mut self, columns: bool, module: bool) -> SourceListMap {
        self.sort_replacements();
        let mut mf = ReplaceMappingFunction::new(&self.replacements);
        let map = self.source.list_map(columns, module);
        let mut map = map.map_generated_code(&mut mf);

        let mut extra_code = String::new();
        while mf.replacement_idx >= 0 {
            extra_code += &self.replacements[mf.replacement_idx as usize].2;
            mf.replacement_idx -= 1;
        }

        if !extra_code.is_empty() {
            map.add(SlmNode::NString(extra_code), None, None);
        }
        map
    }
}

pub struct ReplaceMappingFunction<'a> {
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

// TODO: This function is fucking slow.
fn split_sourcenode(
    node: SmNode,
    mut split_position: i32,
) -> Result<(SmNode, SmNode), (i32, SmNode)> {
    match node {
        SmNode::NSourceNode(n) => {
            let mut is_splitted = false;
            let mut left_children = Vec::<SmNode>::new();
            let mut right_children = Vec::<SmNode>::new();
            left_children.reserve(n.children.len());
            right_children.reserve(n.children.len());
            let c_position = n.position;
            let c_source = n.source.map(|sp| StringPtr::Ptr(sp));
            let c_name = n.name.map(|sp| StringPtr::Ptr(sp));
            let c_source_contents = n.source_contents;
            for child in n.children.into_iter() {
                if !is_splitted {
                    match split_sourcenode(child, split_position) {
                        Ok((l, r)) => {
                            left_children.push(l);
                            right_children.push(r);
                            is_splitted = true;
                        }
                        Err((p, n)) => {
                            split_position = p;
                            left_children.push(n);
                        }
                    }
                } else {
                    right_children.push(child);
                }
            }
            if is_splitted {
                let mut left = SourceNode::new(
                    c_position.clone(),
                    c_source.clone(),
                    c_name.clone(),
                    Some(SmNode::NNodeVec(left_children)),
                );
                let right = SourceNode::new(
                    c_position,
                    c_source,
                    c_name,
                    Some(SmNode::NNodeVec(right_children)),
                );
                left.source_contents = c_source_contents;
                Ok((SmNode::NSourceNode(left), SmNode::NSourceNode(right)))
            } else {
                let mut node = SourceNode::new(
                    c_position,
                    c_source,
                    c_name,
                    Some(SmNode::NNodeVec(left_children)),
                );
                node.source_contents = c_source_contents;
                Err((split_position, SmNode::NSourceNode(node)))
            }
        }
        SmNode::NRcString(n) => {
            let n_len = n.chars().count();
            if n_len as i32 <= split_position {
                Err((split_position - n_len as i32, SmNode::NRcString(n)))
            } else {
                let (left, right) = split_string(&n, split_position, Some(n_len));
                let left = Rc::new(String::from(left));
                let right = Rc::new(String::from(right));
                Ok((SmNode::NRcString(left), SmNode::NRcString(right)))
            }
        }
        SmNode::NString(n) => split_sourcenode(SmNode::NRcString(Rc::new(n)), split_position),
        _ => panic!(),
    }
}

#[inline]
fn replacement_to_sourcenode(old_node: SmNode, new_string: Rc<String>) -> SmNode {
    if let SmNode::NSourceNode(node) = old_node {
        let mut map = node.to_source_map_generator(None, None);
        let original_mapping = map.original_position_for(1, 0);
        let position = original_mapping.original;
        let file = original_mapping.source.map(|sp| StringPtr::Ptr(sp));
        let chunks = Some(SmNode::NRcString(new_string.clone()));
        SmNode::NSourceNode(SourceNode::new(position, file, None, chunks))
    } else {
        panic!()
    }
}

#[inline]
fn split_string(s: &str, pos: i32, s_len: Option<usize>) -> (&str, &str) {
    let s_len = s_len.map_or(s.chars().count(), |l| l);

    if pos <= 0 {
        ("", s)
    } else if pos >= s_len as i32 {
        (s, "")
    } else {
        let pos = s.char_indices().skip(pos as usize).next().unwrap().0;
        s.split_at(pos)
    }
}
