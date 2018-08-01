use source::{Source, SourceTrait};
use source_list_map::{MappingFunction, SourceListMap};
use source_map::{types::Node as SmNode, SourceNode};
use std::str;
use types::string_slice::*;

fn make_prefixed_string(s: StringSlice, prefix: &str, prefix_at_front: bool) -> StringSlice {
    let prefix = prefix.as_bytes();
    let mut result = Vec::<u8>::with_capacity(s.len() * 2);
    if prefix_at_front {
        result.extend_from_slice(&prefix);
    }

    let mut lines = s.split_keep_seperator(b'\n');
    while let Some(line) = lines.next() {
        result.extend_from_slice(line.as_bytes());
        if let Some(rest) = &lines.rest {
            if rest.len() > 0 {
                result.extend_from_slice(prefix);
            }
        }
    }
    StringSlice::from(unsafe { str::from_utf8_unchecked(&result).to_string() })
}

fn clone_and_prefix(node: SmNode, prefix: &str, append: &mut i32) -> SmNode {
    match node {
        SmNode::NString(s) => {
            let end_with_new_line = s.ends_with('\n');
            let s = if *append > 0 {
                *append -= 1;
                make_prefixed_string(s, prefix, true)
            } else {
                make_prefixed_string(s, prefix, false)
            };

            if end_with_new_line {
                *append += 1;
            }
            SmNode::NString(s)
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
    prefix: String,
}

impl PrefixSource {
    pub fn new(prefix: String, source: Source) -> PrefixSource {
        PrefixSource { source, prefix }
    }
}

impl SourceTrait for PrefixSource {
    fn source(&mut self) -> StringSlice {
        StringSlice::from(make_prefixed_string(
            self.source.source(),
            &self.prefix,
            true,
        ))
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
        make_prefixed_string(StringSlice::from(code), &self.prefix, true).into_string()
    }
}
