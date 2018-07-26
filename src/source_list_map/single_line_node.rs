use super::types::Node;
use super::{utils, MappingFunction, MappingsContext, SourceNode};
use std::str;
use types::string_slice::*;
use vlq;

#[derive(Clone, Debug)]
pub struct SingleLineNode {
    pub generated_code: String,
    pub original_source: Option<i32>,
    pub source: Option<i32>,
    pub line: usize,
    pub number_of_lines: usize,
    pub ends_with_new_line: bool,
}

impl SingleLineNode {
    pub fn new(
        generated_code: StringSlice,
        source: Option<i32>,
        original_source: Option<i32>,
        line: usize,
    ) -> Self {
        SingleLineNode {
            original_source,
            source,
            line,
            number_of_lines: utils::number_of_lines(&generated_code),
            ends_with_new_line: generated_code.ends_with('\n'),
            generated_code: generated_code.into_string(),
        }
    }

    pub fn map_generated_code<T: MappingFunction>(self, mf: &mut T) -> SingleLineNode {
        let generated_code = mf.map(self.generated_code);
        SingleLineNode::new(StringSlice::from(generated_code), self.source, self.original_source, self.line)
    }

    pub fn merge(self, other_node: &Node) -> Result<Node, Node> {
        match other_node {
            Node::NSingleLineNode(n) => self.merge_single_line_node(n),
            _ => Err(Node::NSingleLineNode(self)),
        }
    }

    fn merge_single_line_node(mut self, other_node: &SingleLineNode) -> Result<Node, Node> {
        if self.source == other_node.source && self.original_source == other_node.original_source {
            if self.line == other_node.line {
                self.generated_code += &other_node.generated_code;
                self.number_of_lines += other_node.number_of_lines;
                self.ends_with_new_line = other_node.ends_with_new_line;
                Ok(Node::NSingleLineNode(self))
            } else if self.line + 1 == other_node.line
                && self.ends_with_new_line
                && self.number_of_lines == 1
                && other_node.number_of_lines <= 1
            {
                let new_code = self.generated_code + &other_node.generated_code;
                Ok(Node::NSourceNode(SourceNode::new(
                    StringSlice::from(new_code),
                    self.source,
                    self.original_source,
                    self.line,
                )))
            } else {
                Err(Node::NSingleLineNode(self))
            }
        } else {
            Err(Node::NSingleLineNode(self))
        }
    }

    // fn add_single_line_node(&mut self, other_node: SingleLineNode) {
    //     self.generated_code += &other_node.generated_code;
    //     self.number_of_lines += other_node.number_of_lines;
    //     self.ends_with_new_line = other_node.ends_with_new_line;
    //     self
    // }

    pub fn get_generated_code(&self) -> &str {
        &self.generated_code
    }

    pub fn get_mappings(&self, mappings_context: &mut MappingsContext) -> String {
        let mut buf = Vec::<u8>::new();
        if self.generated_code.is_empty() {
            String::new()
        } else {
            let line_mapping = ";AAAA";
            let lines = self.number_of_lines;
            let source_index = mappings_context.ensure_source(
                self.source.clone(),
                self.original_source.clone().map(|n| Node::NStringIdx(n)),
            );

            let mut mappings = if mappings_context.unfinished_generated_line != 0 {
                vlq::encode(mappings_context.unfinished_generated_line as i64, &mut buf).unwrap();
                String::from(",")
            } else {
                String::from("A")
            };
            vlq::encode(
                source_index as i64 - mappings_context.current_source as i64,
                &mut buf,
            ).unwrap();
            vlq::encode(
                self.line as i64 - mappings_context.current_original_line as i64,
                &mut buf,
            ).unwrap();
            buf.push(b'A');
            mappings += str::from_utf8(&buf).unwrap();
            buf.clear();

            mappings_context.current_source = source_index;
            mappings_context.current_original_line = self.line;

            let unfinished_generated_line = utils::get_unfinished_lines(&self.generated_code);
            mappings_context.unfinished_generated_line = unfinished_generated_line;
            if lines > 0 {
                mappings += &line_mapping.repeat(lines.wrapping_sub(1));
            }

            if mappings_context.unfinished_generated_line == 0 {
                mappings += ";";
            } else if lines != 0 {
                mappings += line_mapping;
            }
            mappings
        }
    }

    pub fn get_normalized_nodes(self) -> Vec<SingleLineNode> {
        vec![self]
    }
}
