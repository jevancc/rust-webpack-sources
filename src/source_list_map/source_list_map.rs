use super::types::{GenCode, Node};
use super::{CodeNode, MappingFunction, MappingsContext, SourceNode};
use std::str;
use types::{SourceMap, StringWithSourceMap};

#[derive(Debug, Clone)]
pub struct SourceListMap {
    pub children: Vec<Node>,
}

impl SourceListMap {
    pub fn new(
        generated_code: Option<GenCode>,
        source: Option<i32>,
        original_source: Option<i32>,
    ) -> Self {
        match generated_code {
            Some(GenCode::Code(c)) => {
                let mut slm = SourceListMap {
                    children: Vec::new(),
                };
                slm.add(c, source, original_source);
                slm
            }
            Some(GenCode::CodeVec(cv)) => SourceListMap { children: cv },
            None => SourceListMap {
                children: Vec::new(),
            },
        }
    }

    pub fn add(
        &mut self,
        generated_code: Node,
        source: Option<i32>,
        original_source: Option<i32>,
    ) -> &mut SourceListMap {
        match generated_code {
            Node::NString(s) => {
                if source.is_some() {
                    self.children.push(Node::NSourceNode(SourceNode::new(
                        s,
                        source,
                        original_source,
                        1,
                    )));
                } else {
                    let last_is_code_node = match self.children.last() {
                        Some(Node::NCodeNode(_)) => true,
                        _ => false,
                    };
                    if last_is_code_node {
                        let len = self.children.len();
                        let mut ln = self.children.get_mut(len - 1).unwrap();
                        if let Node::NCodeNode(ref mut ln) = ln {
                            ln.add_generated_code(&s);
                        }
                    } else {
                        self.children.push(Node::NCodeNode(CodeNode::new(s)));
                    }
                }
            }
            Node::NCodeNode(cn) => {
                self.children.push(Node::NCodeNode(cn));
            }
            Node::NSourceNode(sn) => {
                self.children.push(Node::NSourceNode(sn));
            }
            Node::NSingleLineNode(sln) => {
                self.children.push(Node::NSingleLineNode(sln));
            }
            Node::NSourceListMap(mut slm) => {
                self.children.append(&mut slm.children);
            }
            Node::NStringIdx(_) => {
                panic!("Generated code can not be an index");
            }
        }
        self
    }

    pub fn prepend(
        &mut self,
        generated_code: Node,
        source: Option<i32>,
        original_source: Option<i32>,
    ) -> &mut SourceListMap {
        match generated_code {
            Node::NString(s) => {
                if source.is_none() {
                    self.children.insert(
                        0,
                        Node::NSourceNode(SourceNode::new(s, source, original_source, 1)),
                    );
                }
                // TODO: branch for last child node with preprendGeneratedCode
                // else if !self.children.is_empty() {}
                else {
                    self.children.insert(0, Node::NCodeNode(CodeNode::new(s)));
                }
            }
            Node::NCodeNode(cn) => self.children.insert(0, Node::NCodeNode(cn)),
            Node::NSourceNode(sn) => self.children.insert(0, Node::NSourceNode(sn)),
            Node::NSingleLineNode(sln) => self.children.insert(0, Node::NSingleLineNode(sln)),
            Node::NSourceListMap(mut slm) => {
                let mut new_childern = Vec::<Node>::new();
                new_childern.append(&mut slm.children);
                new_childern.append(&mut self.children);
                self.children = new_childern;
            }
            Node::NStringIdx(_) => {
                panic!("Generated code can not be an index");
            }
        }
        self
    }

    pub fn map_generated_code<T: MappingFunction>(self, mf: &mut T) -> SourceListMap {
        let mut normalized_nodes: Vec<Node> = Vec::with_capacity(self.children.len());
        let children = self.children;

        for child in children {
            match child {
                Node::NCodeNode(cn) => {
                    normalized_nodes.append(&mut cn.get_normalized_nodes());
                }
                Node::NSourceNode(sn) => {
                    normalized_nodes.append(&mut sn.get_normalized_nodes());
                }
                Node::NSingleLineNode(sln) => {
                    normalized_nodes.append(&mut sln.get_normalized_nodes());
                }
                _ => {}
            }
        }

        let mut optimized_nodes: Vec<Node> = Vec::with_capacity(normalized_nodes.len());
        for nodes in normalized_nodes {
            let sln = match nodes {
                // Node::NSourceNode(n) => Some(Node::NSourceNode(n.map_generated_code(fn_name)),
                Node::NCodeNode(n) => Some(Node::NCodeNode(n.map_generated_code(mf))),
                Node::NSingleLineNode(n) => Some(Node::NSingleLineNode(n.map_generated_code(mf))),
                _ => None,
            };

            if optimized_nodes.is_empty() {
                if let Some(n) = sln {
                    optimized_nodes.push(n);
                }
            } else {
                let last = optimized_nodes.pop().unwrap();
                let merged_node: Result<Node, Node> = match last {
                    Node::NCodeNode(ln) => match sln {
                        Some(ref n) => ln.merge(n),
                        _ => Err(Node::NCodeNode(ln)),
                    },
                    Node::NSourceNode(ln) => match sln {
                        Some(ref n) => ln.merge(n),
                        _ => Err(Node::NSourceNode(ln)),
                    },
                    Node::NSingleLineNode(ln) => match sln {
                        Some(ref n) => ln.merge(n),
                        _ => Err(Node::NSingleLineNode(ln)),
                    },
                    _ => Err(last),
                };

                match merged_node {
                    Ok(n) => {
                        optimized_nodes.push(n);
                    }
                    Err(n) => {
                        optimized_nodes.push(n);
                        if let Some(n) = sln {
                            optimized_nodes.push(n);
                        }
                    }
                }
            }
        }
        SourceListMap::new(Some(GenCode::CodeVec(optimized_nodes)), None, None)
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        for child in &self.children {
            if let Node::NSingleLineNode(sln) = child {
                output.push_str(&sln.get_generated_code());
            }
        }
        output
    }

    pub fn to_string_with_source_map(&self, options_file: Option<i32>) -> StringWithSourceMap {
        let mut mc: MappingsContext = MappingsContext::new();

        let mut src: String = String::with_capacity(80);
        for child in &self.children {
            match child {
                Node::NCodeNode(ref sln) => src.push_str(sln.get_generated_code()),
                Node::NSourceNode(ref sln) => src.push_str(sln.get_generated_code()),
                Node::NSingleLineNode(ref sln) => src.push_str(sln.get_generated_code()),
                Node::NString(ref sln) => src.push_str(&sln),
                _ => {}
            }
        }

        let mut mappings = Vec::<u8>::with_capacity(256);
        for child in &self.children {
            match child {
                Node::NSourceNode(ref sln) => {
                    mappings.extend_from_slice(&sln.get_mappings(&mut mc))
                }
                Node::NCodeNode(ref sln) => mappings.extend_from_slice(&sln.get_mappings(&mut mc)),
                Node::NSingleLineNode(ref sln) => {
                    mappings.extend_from_slice(&sln.get_mappings(&mut mc))
                }
                _ => {}
            };
        }

        let file = options_file.map_or(-1, |s| s);
        let arrays = mc.get_arrays();
        StringWithSourceMap {
            source: src,
            map: SourceMap {
                version: 3,
                file: Some(file),
                sources: arrays.0.iter().map(|idx| idx.unwrap_or(-1)).collect(),
                sources_content: arrays.1,
                mappings: unsafe { str::from_utf8_unchecked(&mappings).to_string() },
                names: vec![],
                source_root: None,
            },
        }
    }
}
