use super::types::Node;
use super::utils::*;
use super::{utils, MappingFunction, MappingsContext, SourceNode};
use types::string_slice::*;
use vlq;

#[derive(Clone, Debug)]
pub struct SingleLineNode {
    pub generated_code: Vec<StringSlice>,
    pub generated_code_len: usize,
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
            generated_code_len: generated_code.len(),
            generated_code: vec![generated_code],
        }
    }

    pub fn map_generated_code<T: MappingFunction>(self, mf: &mut T) -> SingleLineNode {
        let code = self.generated_code.into_string(self.generated_code_len);
        let generated_code = mf.map(code);
        SingleLineNode::new(
            StringSlice::from(generated_code),
            self.source,
            self.original_source,
            self.line,
        )
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
                self.generated_code
                    .extend(other_node.generated_code.iter().cloned());
                self.generated_code_len += other_node.generated_code_len;
                self.number_of_lines += other_node.number_of_lines;
                self.ends_with_new_line = other_node.ends_with_new_line;
                Ok(Node::NSingleLineNode(self))
            } else if self.line + 1 == other_node.line
                && self.ends_with_new_line
                && self.number_of_lines == 1
                && other_node.number_of_lines <= 1
            {
                self.generated_code
                    .extend(other_node.generated_code.iter().cloned());
                self.generated_code_len += other_node.generated_code_len;
                self.number_of_lines += other_node.number_of_lines;
                self.ends_with_new_line = other_node.ends_with_new_line;
                Ok(Node::NSourceNode(SourceNode {
                    ends_with_new_line: self.ends_with_new_line,
                    number_of_lines: self.number_of_lines,
                    generated_code_len: self.generated_code_len,
                    generated_code: self.generated_code,
                    original_source: self.original_source,
                    source: self.source,
                    starting_line: self.line,
                }))
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

    pub fn get_generated_code(&self) -> (&Vec<StringSlice>, usize) {
        (&self.generated_code, self.generated_code_len)
    }

    pub fn get_mappings(&self, mappings_context: &mut MappingsContext) -> Vec<u8> {
        if self.generated_code_len == 0 {
            Vec::new()
        } else {
            let mut buf = Vec::<u8>::with_capacity(64);
            let line_mapping = ";AAAA".as_bytes();
            let lines = self.number_of_lines;
            let source_index = mappings_context.ensure_source(
                self.source.clone(),
                self.original_source.clone().map(|n| Node::NStringIdx(n)),
            );

            if mappings_context.unfinished_generated_line != 0 {
                buf.push(b',');
                vlq::encode(mappings_context.unfinished_generated_line as i64, &mut buf).unwrap();
            } else {
                buf.push(b'A');
            }
            vlq::encode(
                source_index as i64 - mappings_context.current_source as i64,
                &mut buf,
            ).unwrap();
            vlq::encode(
                self.line as i64 - mappings_context.current_original_line as i64,
                &mut buf,
            ).unwrap();
            buf.push(b'A');

            mappings_context.current_source = source_index;
            mappings_context.current_original_line = self.line;

            let unfinished_generated_line = utils::get_unfinished_lines(&self.generated_code);
            mappings_context.unfinished_generated_line = unfinished_generated_line;
            for _ in 0..lines.saturating_sub(1) {
                buf.extend_from_slice(&line_mapping);
            }

            if mappings_context.unfinished_generated_line == 0 {
                buf.push(b';');
            } else if lines != 0 {
                buf.extend_from_slice(line_mapping);
            }
            buf
        }
    }

    pub fn get_normalized_nodes(self) -> Vec<Node> {
        vec![Node::NSingleLineNode(self)]
    }
}
