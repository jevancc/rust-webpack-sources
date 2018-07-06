use super::types::{GenCode, Node};
use super::{CodeNode, MappingFunction, MappingsContext, SourceNode};
use types::{SourceMap, StringPtr, StringWithSourceMap};

#[derive(Clone, Debug)]
pub struct SourceListMap {
    pub children: Vec<Node>,
}

impl SourceListMap {
    pub fn new(
        generated_code: Option<GenCode>,
        source: Option<StringPtr>,
        original_source: Option<StringPtr>,
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
        source: Option<StringPtr>,
        original_source: Option<StringPtr>,
    ) -> &mut SourceListMap {
        match generated_code {
            Node::NRcString(sp) => {
                let s: String = (*sp).clone();
                self.add(Node::NString(s), source, original_source);
            }
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
            Node::NSourceListMap(slm) => {
                for child in slm.children {
                    self.children.push(child);
                }
            }
        }
        self
    }

    pub fn prepend(
        &mut self,
        generated_code: Node,
        source: Option<StringPtr>,
        original_source: Option<StringPtr>,
    ) -> &mut SourceListMap {
        match generated_code {
            Node::NRcString(sp) => {
                let s: String = (*sp).clone();
                self.prepend(Node::NString(s), source, original_source);
            }
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
        }
        self
    }

    pub fn map_generated_code<T: MappingFunction>(self, mf: &mut T) -> SourceListMap {
        let mut normalized_nodes: Vec<Node> = Vec::new();
        let children = self.children;

        for child in children {
            match child {
                Node::NCodeNode(cn) => {
                    for n in cn.get_normalized_nodes() {
                        normalized_nodes.push(Node::NCodeNode(n));
                    }
                }
                Node::NSourceNode(sn) => {
                    for n in sn.get_normalized_nodes() {
                        normalized_nodes.push(Node::NSingleLineNode(n));
                    }
                }
                Node::NSingleLineNode(sln) => {
                    for n in sln.get_normalized_nodes() {
                        normalized_nodes.push(Node::NSingleLineNode(n));
                    }
                }
                _ => {}
            }
        }

        let mut optimized_nodes: Vec<Node> = Vec::new();
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
                output += sln.get_generated_code();
            }
        }
        output
    }

    pub fn to_string_with_source_map(&self, options_file: Option<String>) -> StringWithSourceMap {
        let mut mc: MappingsContext = MappingsContext::new();

        let mut src: String = String::new();
        for child in &self.children {
            match child {
                Node::NCodeNode(ref sln) => src += sln.get_generated_code(),
                Node::NSourceNode(ref sln) => src += sln.get_generated_code(),
                Node::NSingleLineNode(ref sln) => src += sln.get_generated_code(),
                Node::NString(ref sln) => src += &sln,
                _ => {}
            }
        }

        let mut mappings: String = String::new();
        for child in &self.children {
            match child {
                Node::NSourceNode(ref sln) => mappings += &sln.get_mappings(&mut mc),
                Node::NCodeNode(ref sln) => mappings += &sln.get_mappings(&mut mc),
                Node::NSingleLineNode(ref sln) => mappings += &sln.get_mappings(&mut mc),
                _ => {}
            };
        }

        let file = options_file.map_or(String::new(), |s| s);
        let arrays = mc.get_arrays();
        StringWithSourceMap {
            source: src,
            map: SourceMap {
                version: 3,
                file: Some(file),
                sources: arrays
                    .sources
                    .iter()
                    .map(|sp| {
                        if let Some(ref sp) = sp {
                            (**sp).clone()
                        } else {
                            String::new()
                        }
                    })
                    .collect(),
                sources_content: if mc.has_source_content {
                    let mut vec = Vec::<String>::new();
                    for sc in arrays.sources_content {
                        match sc {
                            Node::NString(s) => vec.push(s),
                            Node::NRcString(sp) => vec.push((*sp).clone()),
                            _ => {}
                        }
                    }
                    vec
                } else {
                    vec![]
                },
                mappings,
                names: vec![],
                source_root: None,
            },
        }
    }
}

//
// #[derive(Debug, PartialEq, Serialize, Deserialize)]
// pub struct StringWithSrcMap {
//     pub source: String,
//     pub map: SrcMap,
// }
//
// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct SrcMap {
//     pub version: i32,
//     pub file: String,
//     pub sources: Vec<String>,
//     pub sources_content: Vec<String>,
//     pub mappings: String,
// }
