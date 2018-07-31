use source::{Source, SourceTrait};
use source_list_map::{MappingFunction, SourceListMap};
use source_map::{types::Node as SmNode, SourceNode};
use std::rc::Rc;
use types::string_slice::*;
use types::string_cat::*;

fn clone_and_prefix(node: SmNode, prefix: &str, append: &mut i32) -> SmNode {
    match node {
        SmNode::NString(s) => {
            let mut s = s.into_string();
            let end_with_new_line = s.ends_with('\n');
            if end_with_new_line {
                s.pop();
                s = s.replace('\n', &(String::from("\n") + prefix));
                s.push('\n');
            } else {
                s = s.replace('\n', &(String::from("\n") + prefix));
            }

            if *append > 0 {
                *append -= 1;
                s = String::from(prefix) + &s;
            }
            if end_with_new_line {
                *append += 1;
            }
            SmNode::NString(StringSlice::from(s))
        }
        SmNode::NSourceNode(mut sn) => {
            let mut new_children = Vec::<SmNode>::new();
            for child in sn.children.into_iter() {
                new_children.push(clone_and_prefix(child, prefix, append));
            }
            sn.children = new_children;
            SmNode::NSourceNode(sn)
        }
        _ => SmNode::NString(StringSlice::new()),
    }
}

#[derive(Debug)]
pub struct PrefixSource {
    pub source: Source,
    prefix: StringSlice,
}

impl PrefixSource {
    pub fn new(prefix: String, source: Source) -> PrefixSource {
        PrefixSource { source, prefix: StringSlice::from(prefix) }
    }
}

impl SourceTrait for PrefixSource {
    fn source(&mut self) -> StringCat {
        let mut s = self.prefix.to_string();
        s.push_string_cat(&self.source.source());
        if s.ends_with('\n') {
            s.pop();
            s = s.replace('\n', &(String::from("\n") + &self.prefix));
            s.push('\n');
        } else {
            s = s.replace('\n', &(String::from("\n") + &self.prefix));
        }
        StringCat::from(Rc::new(s))
    }

    fn node(&mut self, columns: bool, module: bool) -> SourceNode {
        let node = self.source.node(columns, module);
        let mut append = 1;
        SourceNode::new(
            None,
            None,
            None,
            Some(clone_and_prefix(
                SmNode::NSourceNode(node),
                &self.prefix,
                &mut append,
            )),
        )
    }

    fn list_map(&mut self, columns: bool, module: bool) -> SourceListMap {
        let mut mapping_fn = PrefixMappingFunction {
            prefix: &self.prefix,
        };
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
