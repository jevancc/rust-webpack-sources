use source_map::{SourceNode, Node as SMNode};
use source_list_map::{SourceListMap, MappingFunction};
use source::{Source, SourceTrait};
use std::rc::Rc;

// TODO: `append` to be a counter
fn clone_and_prefix(node: SMNode, prefix: &str, append: &mut Vec<String>) -> Result<SMNode, &'static str> {
    match node {
        SMNode::NRcString(s) => {
            Ok(clone_and_prefix(SMNode::NString((*s).clone()), prefix, append).unwrap())
        }
        SMNode::NString(mut s) => {
            let end_with_new_line = s.ends_with('\n');
            if end_with_new_line {
                s.pop();
                s = s.replace('\n', &(String::from("\n") + prefix));
                s.push('\n');
            } else {
                s = s.replace('\n', &(String::from("\n") + prefix));
            }
            
            if !append.is_empty() {
                s = append.pop().unwrap() + &s;
            }
            if end_with_new_line {
                append.push(String::from(prefix));
            }
            Ok(SMNode::NRcString(Rc::new(s)))
        }
        SMNode::NSourceNode(mut sn) => {
            let mut new_children = Vec::<SMNode>::new();
            for child in sn.children.into_iter() {
                new_children.push(clone_and_prefix(child, prefix, append).unwrap());
            }
            sn.children = new_children;
            Ok(SMNode::NSourceNode(sn))
        }
        _ => {
            Ok(SMNode::NString(String::new()))
        }
    }
}

#[derive(Debug)]
pub struct PrefixSource {
    pub source: Source,
    prefix: String,
}

impl PrefixSource {
    pub fn new(prefix: String, source: Source) -> PrefixSource {
        PrefixSource {
            source,
            prefix,
        }
    }
}

impl SourceTrait for PrefixSource {
    fn source(&mut self) -> String {
        let mut node = match &mut self.source {
            Source::Raw(s) => s.source(),
            Source::Original(s) => s.source(),
            Source::Replace(s) => s.source(),
            Source::Prefix(s) => s.source(),
            Source::Concat(s) => s.source(),
            Source::LineToLineMapped(s) => s.source(),
            Source::SString(s) => String::clone(&s),
        };

        if node.ends_with('\n') {
            node = self.prefix.clone() + &node;
            node.pop();
            node = node.replace('\n', &(String::from("\n") + &self.prefix));
            node.push('\n');
        } else {
            node = self.prefix.clone() + &node;
            node = node.replace('\n', &(String::from("\n") + &self.prefix));
        }
        node
    }

    fn node(&mut self, columns: bool, module: bool) -> SourceNode {
        let node = match &mut self.source {
            Source::Raw(s) => s.node(columns, module),
            Source::Original(s) => s.node(columns, module),
            Source::Replace(s) => s.node(columns, module),
            Source::Prefix(s) => s.node(columns, module),
            Source::Concat(s) => s.node(columns, module),
            Source::LineToLineMapped(s) => s.node(columns, module),
            Source::SString(_) => panic!(),
        };

        let mut append = Vec::<String>::new();
        append.push(self.prefix.clone());
        SourceNode::new(None, None, None, Some(
            clone_and_prefix(SMNode::NSourceNode(node), &self.prefix, &mut append).unwrap()
        ))
    }

    fn list_map(&mut self, columns: bool, module: bool) -> SourceListMap {
        let mut mapping_fn = PrefixMappingFunction { prefix: &self.prefix };
        let map = match &mut self.source {
            Source::Raw(s) => s.list_map(columns, module),
            Source::Original(s) => s.list_map(columns, module),
            Source::Replace(s) => s.list_map(columns, module),
            Source::Prefix(s) => s.list_map(columns, module),
            Source::Concat(s) => s.list_map(columns, module),
            Source::LineToLineMapped(s) => s.list_map(columns, module),
            Source::SString(_) => panic!(),
        };
        map.map_generated_code(&mut mapping_fn)
    }
}

struct PrefixMappingFunction<'a> {
    prefix: &'a str,
}

impl<'a> MappingFunction for PrefixMappingFunction<'a> {
    fn map(&mut self, code: String) -> String {
        let mut mapped = String::from(self.prefix) + &code;
        if code.ends_with('\n') {
            mapped.pop();
            mapped = mapped.replace('\n', &(String::from("\n") + &self.prefix));
            mapped.push('\n');
        } else {
            mapped = mapped.replace('\n', &(String::from("\n") + &self.prefix));
        }
        mapped
    }
}
