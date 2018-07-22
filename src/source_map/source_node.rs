use super::types::{Mapping, Node};
use super::SourceMapGenerator;
use std::collections::HashMap;
use std::rc::Rc;
use types::{StringPtr, StringWithSourceMap};

#[derive(Clone, Debug)]
pub struct SourceNode {
    pub children: Vec<Node>,
    pub source_contents: HashMap<i32, i32>,
    // (line, column)
    pub position: Option<(usize, usize)>,
    pub source: Option<i32>,
    pub name: Option<i32>,
}

impl SourceNode {
    pub fn new(
        position: Option<(usize, usize)>,
        source: Option<i32>,
        name: Option<i32>,
        chunks: Option<Node>,
    ) -> SourceNode {
        let mut sn = SourceNode {
            position,
            source,
            name,
            children: Vec::new(),
            source_contents: HashMap::new(),
        };
        if let Some(chunks) = chunks {
            sn.add(chunks);
        }
        sn
    }

    pub fn add(&mut self, chunk: Node) {
        match chunk {
            Node::NNodeVec(nv) => {
                for node in nv {
                    self.children.push(node);
                }
            }
            Node::NSourceNode(sn) => {
                self.children.push(Node::NSourceNode(sn));
            }
            Node::NString(s) => {
                self.children.push(Node::NRcString(Rc::new(s)));
            }
            Node::NRcString(sp) => {
                self.children.push(Node::NRcString(sp));
            }
        }
    }

    pub fn set_source_content(&mut self, source: i32, source_content: i32) {
        self.source_contents.insert(source, source_content);
    }

    pub fn to_string_with_source_map(
        self,
        file: Option<i32>,
        source_root: Option<StringPtr>,
    ) -> StringWithSourceMap {
        let source_root = source_root.map(|sp| sp.to_ptr());
        let skip_validation = true;
        let mut context = ToSourceMapContext::new(file, source_root, skip_validation);
        self.map_generated_code(&mut context);

        StringWithSourceMap {
            source: context.generated_code,
            map: context.map.to_source_map(),
        }
    }

    pub fn to_source_map_generator(
        self,
        file: Option<i32>,
        source_root: Option<StringPtr>,
    ) -> SourceMapGenerator {
        let source_root = source_root.map(|sp| sp.to_ptr());
        let skip_validation = true;
        let mut context = ToSourceMapContext::new(file, source_root, skip_validation);
        self.map_generated_code(&mut context);

        context.map
    }

    fn map_generated_code<T: WalkFunction>(self, context: &mut T) {
        // position: source: Option<i32>, Option<(usize, usize)>, name: Option<i32>
        let mut parents: Vec<(Option<i32>, Option<(usize, usize)>, Option<i32>)> = Vec::new();
        parents.push((self.source, self.position, self.name));

        let mut stack: Vec<(usize, Node)> = self.children.into_iter().rev().map(|child| (0, child)).collect();
        while let Some((pidx, child)) = stack.pop() {
            match child {
                Node::NSourceNode(sn) => {
                    let parent_idx = parents.len();
                    let mut children: Vec<(usize, Node)>
                        = sn.children.into_iter().rev().map(|child| (parent_idx, child)).collect();
                    stack.append(&mut children);
                    parents.push((sn.source, sn.position, sn.name));

                    for (source, source_content) in sn.source_contents {
                        context.process_source_content(source, source_content);
                    }
                }
                Node::NRcString(chunk) => {
                    context.process_chunk(&chunk, &parents[pidx].0, &parents[pidx].1, &parents[pidx].2);
                }
                _ => {}
            }
        }
    }

    pub fn add_mapping_with_code(&mut self, mapping: Option<Mapping>, code: &str) {
        let is_original = mapping
            .as_ref()
            .map_or(false, |mapping| mapping.source.is_some());
        if !is_original {
            self.add(Node::NString(String::from(code)));
        } else {
            let mapping = mapping.unwrap();
            self.add(Node::NSourceNode(SourceNode::new(
                mapping.original,
                mapping.source,
                mapping.name,
                Some(Node::NString(String::from(code))),
            )));
        }
    }
}

struct ToSourceMapContext {
    pub map: SourceMapGenerator,
    source_mapping_active: bool,
    last_original_source: Option<i32>,
    last_original_position: Option<(usize, usize)>,
    last_original_name: Option<i32>,
    generated_code: String,
    generated_position: (usize, usize),
}

impl ToSourceMapContext {
    pub fn new(
        file: Option<i32>,
        source_root: Option<Rc<String>>,
        skip_validation: bool,
    ) -> ToSourceMapContext {
        let source_root = source_root.map(|sp| StringPtr::Ptr(sp));
        ToSourceMapContext {
            map: SourceMapGenerator::new(file, source_root, skip_validation),
            source_mapping_active: false,
            last_original_source: None,
            last_original_position: None,
            last_original_name: None,
            generated_code: String::new(),
            generated_position: (1, 0),
        }
    }
}

impl WalkFunction for ToSourceMapContext {
    fn process_chunk(
        &mut self,
        chunk: &Rc<String>,
        original_source: &Option<i32>,
        original_position: &Option<(usize, usize)>,
        original_name: &Option<i32>,
    ) {
        self.generated_code += chunk;
        if original_source.is_some() && original_position.is_some() {
            if self.last_original_source != *original_source
                || self.last_original_position != *original_position
                || self.last_original_name != *original_name
            {
                self.map.add_mapping(Mapping {
                    source: original_source.clone(),
                    original: original_position.clone(),
                    generated: self.generated_position.clone(),
                    name: original_name.clone(),
                });
            }
            self.last_original_source = original_source.clone();
            self.last_original_position = original_position.clone();
            self.last_original_name = original_name.clone();
            self.source_mapping_active = true;
        } else if self.source_mapping_active {
            self.map.add_mapping(Mapping {
                source: None,
                original: None,
                generated: self.generated_position.clone(),
                name: None,
            });
            self.last_original_source = None;
            self.source_mapping_active = false;
        }
        let mut chars = chunk.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\n' {
                self.generated_position.0 += 1; // line++
                self.generated_position.1 = 0; // column = 0

                if chars.peek().is_none() {
                    self.last_original_source = None;
                    self.source_mapping_active = false;
                } else if self.source_mapping_active {
                    self.map.add_mapping(Mapping {
                        source: original_source.clone(),
                        original: original_position.clone(),
                        generated: self.generated_position.clone(),
                        name: original_name.clone(),
                    })
                }
            } else {
                self.generated_position.1 += 1; // column++
            }
        }
    }

    fn process_source_content(&mut self, source: i32, source_content: i32) {
        self.map.set_source_content(source, Some(source_content));
    }
}

trait WalkFunction {
    fn process_chunk(
        &mut self,
        chunk: &Rc<String>,
        original_source: &Option<i32>,
        original_position: &Option<(usize, usize)>,
        original_name: &Option<i32>,
    );
    fn process_source_content(&mut self, source: i32, source_content: i32);
}
