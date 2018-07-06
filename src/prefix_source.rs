use std::rc::Rc;
use source_map::{SourceNode, types::Node as SmNode};
use source_list_map::{SourceListMap, MappingFunction};
use source::{Source, SourceTrait};

// TODO: `append` to be a counter
fn clone_and_prefix(node: SmNode, prefix: &str, append: &mut Vec<String>) -> Result<SmNode, &'static str> {
    match node {
        SmNode::NRcString(s) => {
            Ok(clone_and_prefix(SmNode::NString((*s).clone()), prefix, append).unwrap())
        }
        SmNode::NString(mut s) => {
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
            Ok(SmNode::NRcString(Rc::new(s)))
        }
        SmNode::NSourceNode(mut sn) => {
            let mut new_children = Vec::<SmNode>::new();
            for child in sn.children.into_iter() {
                new_children.push(clone_and_prefix(child, prefix, append).unwrap());
            }
            sn.children = new_children;
            Ok(SmNode::NSourceNode(sn))
        }
        _ => {
            Ok(SmNode::NString(String::new()))
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
            clone_and_prefix(SmNode::NSourceNode(node), &self.prefix, &mut append).unwrap()
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
