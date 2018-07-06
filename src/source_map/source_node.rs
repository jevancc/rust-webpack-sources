use std::rc::Rc;
use std::collections::HashMap;
use super::{SourceMapConsumer, SourceMapGenerator};
use super::types::{Node, Mapping};
use types::{StringWithSourceMap, StringPtr};
use serde_json;

#[derive(Clone, Debug)]
pub struct SourceNode {
    pub children: Vec<Node>,
    pub source_contents: HashMap<Rc<String>, Rc<String>>,
    // (line, column)
    pub position: Option<(usize, usize)>,
    pub source: Option<Rc<String>>,
    pub name: Option<Rc<String>>,
}

impl SourceNode {
    pub fn new(
        position: Option<(usize, usize)>,
        source: Option<StringPtr>,
        name: Option<StringPtr>,
        chunks: Option<Node>,
    ) -> SourceNode {
        let source = source.map(|sp| sp.to_ptr());
        let name = name.map(|sp| sp.to_ptr());
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
            Node::NNodeVec(mut nv) => {
                self.children.append(&mut nv);
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

    pub fn set_source_content(&mut self, source: StringPtr, source_content: StringPtr) {
        let source = source.to_ptr();
        let source_content = source_content.to_ptr();
        self.source_contents.insert(source, source_content);
    }

    pub fn to_string_with_source_map(
        &self,
        file: Option<StringPtr>,
        source_root: Option<StringPtr>,
    ) -> StringWithSourceMap {
        let file = file.map(|sp| sp.to_ptr());
        let source_root = source_root.map(|sp| sp.to_ptr());
        let skip_validation = true;
        let mut context = ToSourceMapContext::new(file, source_root, skip_validation);
        self.walk(&mut context);

        StringWithSourceMap {
            source: context.generated_code,
            map: context.map.to_source_map(),
        }
    }

    pub fn to_source_map_generator(&self, file: Option<StringPtr>, source_root: Option<StringPtr>)
        -> SourceMapGenerator {
        let file = file.map(|sp| sp.to_ptr());
        let source_root = source_root.map(|sp| sp.to_ptr());
        let skip_validation = true;
        let mut context = ToSourceMapContext::new(file, source_root, skip_validation);
        self.walk(&mut context);

        context.map
    }

    fn walk<T: WalkFunction>(&self, context: &mut T) {
        for child in &self.children {
            match child {
                Node::NSourceNode(sn) => {
                    sn.walk(context);
                }
                Node::NRcString(chunk) => {
                    context.process_chunk(&chunk, &self.source, &self.position, &self.name);
                }
                _ => {}
            }
        }
        for (source, source_content) in &self.source_contents {
            context.process_source_content(source, source_content);
        }
    }

    pub fn from_string_with_source_map(code: &str, source_map_consumer_json: &str) -> SourceNode {
        let source_map: SourceMapConsumer = serde_json::from_str(source_map_consumer_json).unwrap();
        let mut node = SourceNode::new(None, None, None, None);

        let mut lines = code.split('\n').peekable();
        let mut last_generated_position: (usize, usize) = (1, 0);
        let mut last_mapping: Option<Mapping> = None;
        let mut shift_lines = || {
            let next = lines.next();
            next.map_or((String::new(), false), |line| {
                if lines.peek().is_some() {
                    (String::from(line) + "\n", true)
                } else {
                    (String::from(line), true)
                }
            })
        };
        let mut next_line = shift_lines();

        for mapping in source_map.mappings.into_iter() {
            let generated_position = mapping.0;
            let source = mapping.1;
            let name = mapping.2;
            let original_position = mapping.3;

            if last_mapping.is_some() {
                if last_generated_position.0 < generated_position.0 {
                    node.add_mapping_with_code(last_mapping, next_line.0);
                    next_line = shift_lines();
                    // line++, column = 0
                    last_generated_position.0 += 1;
                    last_generated_position.1 = 0;
                } else {
                    let splitted = split_string(
                        next_line.0,
                        generated_position.1 as i32 - last_generated_position.1 as i32,
                        None
                    );
                    let code = splitted.0;
                    next_line.0 = splitted.1;
                    last_generated_position.1 = generated_position.1;

                    node.add_mapping_with_code(last_mapping, code);
                    last_mapping = Some(Mapping {
                        generated: generated_position,
                        source: source.map(|s| Rc::new(s)),
                        name: name.map(|s| Rc::new(s)),
                        original: original_position,
                    });
                    continue;
                }
            }

            while last_generated_position.0 < generated_position.0 {
                node.add(Node::NString(next_line.0));
                next_line = shift_lines();
                last_generated_position.0 += 1;
            }
            if last_generated_position.1 < generated_position.1 {
                let splitted = split_string(
                    next_line.0,
                    generated_position.1 as i32,
                    None
                );
                node.add(Node::NString(splitted.0));
                next_line.0 = splitted.1;
                last_generated_position.1 = generated_position.1;
            }
            last_mapping = Some(Mapping {
                generated: generated_position,
                source: source.map(|s| Rc::new(s)),
                name: name.map(|s| Rc::new(s)),
                original: original_position,
            });
        }

        if next_line.1 {
            if last_mapping.is_some() {
                node.add_mapping_with_code(last_mapping, next_line.0);
                next_line = shift_lines();
            }
            let mut remaining = String::new();
            while next_line.1 {
                remaining += &next_line.0;
                next_line = shift_lines();
            }
            node.add(Node::NString(remaining));
        }

        for source in source_map.sources.into_iter() {
            let file = source.0;
            let content = source.1;
            content.map(|content| {
                node.set_source_content(
                    StringPtr::Str(file),
                    StringPtr::Str(content)
                )
            });
        }
        node
    }

    fn add_mapping_with_code(&mut self, mapping: Option<Mapping>, code: String) {
        let is_original = mapping.as_ref().map_or(false, |mapping| mapping.source.is_some());
        if !is_original {
            self.add(Node::NString(code));
        } else {
            let mapping = mapping.unwrap();
            self.add(Node::NSourceNode(
                SourceNode::new(
                    mapping.original,
                    mapping.source.map(|sp| StringPtr::Ptr(sp)),
                    mapping.name.map(|sp| StringPtr::Ptr(sp)),
                    Some(Node::NString(code))
                )
            ));
        }
    }
}

struct ToSourceMapContext {
    pub map: SourceMapGenerator,
    source_mapping_active: bool,
    last_original_source: Option<Rc<String>>,
    last_original_position: Option<(usize, usize)>,
    last_original_name: Option<Rc<String>>,
    generated_code: String,
    generated_position: (usize, usize),
}

impl ToSourceMapContext {
    pub fn new(
        file: Option<Rc<String>>,
        source_root: Option<Rc<String>>,
        skip_validation: bool,
    ) -> ToSourceMapContext {
        let file = file.map(|sp| StringPtr::Ptr(sp));
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
        original_source: &Option<Rc<String>>,
        original_position: &Option<(usize, usize)>,
        original_name: &Option<Rc<String>>,
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

    fn process_source_content(&mut self, source: &Rc<String>, source_content: &Rc<String>) {
        self.map.set_source_content(
            StringPtr::Ptr(source.clone()),
            Some(StringPtr::Ptr(source_content.clone())),
        );
    }
}

trait WalkFunction {
    fn process_chunk(
        &mut self,
        chunk: &Rc<String>,
        original_source: &Option<Rc<String>>,
        original_position: &Option<(usize, usize)>,
        original_name: &Option<Rc<String>>,
    );
    fn process_source_content(&mut self, source: &Rc<String>, source_content: &Rc<String>);
}

#[inline]
fn split_string(mut s: String, pos: i32, s_len: Option<usize>) -> (String, String) {
    let s_len = s_len.map_or(s.chars().count(), |l| l);

    if pos <= 0 {
        (String::new(), s)
    } else if pos >= s_len as i32 {
        (s, String::new())
    } else {
        let pos = s.char_indices().skip(pos as usize).next().unwrap().0;
        let off = s.split_off(pos);
        (s, off)
    }
}
