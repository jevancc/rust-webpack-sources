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
        let mut s = self.source.source();

        if s.ends_with('\n') {
            s = self.prefix.clone() + &s;
            s.pop();
            s = s.replace('\n', &(String::from("\n") + &self.prefix));
            s.push('\n');
        } else {
            s = self.prefix.clone() + &s;
            s = s.replace('\n', &(String::from("\n") + &self.prefix));
        }
        s
    }

    fn node(&mut self, columns: bool, module: bool) -> SourceNode {
        let node = self.source.node(columns, module);

        let mut append = Vec::<String>::new();
        append.push(self.prefix.clone());
        SourceNode::new(None, None, None, Some(
            clone_and_prefix(SMNode::NSourceNode(node), &self.prefix, &mut append).unwrap()
        ))
    }

    fn list_map(&mut self, columns: bool, module: bool) -> SourceListMap {
        let mut mapping_fn = PrefixMappingFunction { prefix: &self.prefix };
        let map = self.source.list_map(columns, module);
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
