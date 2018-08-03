use super::types::Node;
use super::utils::*;
use super::{utils, MappingFunction, MappingsContext};
use types::string_slice::*;

#[derive(Clone, Debug)]
pub struct CodeNode {
    generated_code: Vec<StringSlice>,
    generated_code_len: usize,
    number_of_lines: usize,
}

impl CodeNode {
    pub fn new(generated_code: StringSlice) -> Self {
        CodeNode {
            number_of_lines: utils::number_of_lines(&generated_code),
            generated_code_len: generated_code.len(),
            generated_code: vec![generated_code],
        }
    }

    pub fn add_generated_code(&mut self, generated_code: StringSlice) {
        self.number_of_lines += utils::number_of_lines(&generated_code);
        self.generated_code_len += generated_code.len();
        self.generated_code.push(generated_code);
    }

    pub fn map_generated_code<T: MappingFunction>(self, mf: &mut T) -> CodeNode {
        let code = self.generated_code.into_string(self.generated_code_len);
        let generated_code = mf.map(code);
        CodeNode::new(StringSlice::from(generated_code))
    }

    pub fn merge(mut self, other_node: &Node) -> Result<Node, Node> {
        match other_node {
            Node::NCodeNode(n) => {
                self.generated_code.extend(n.generated_code.iter().cloned());
                self.generated_code_len += n.generated_code_len;
                self.number_of_lines += n.number_of_lines;
                Ok(Node::NCodeNode(self))
            }
            _ => Err(Node::NCodeNode(self)),
        }
    }

    pub fn get_generated_code(&self) -> (&Vec<StringSlice>, usize) {
        (&self.generated_code, self.generated_code_len)
    }

    pub fn get_mappings(&self, mappings_context: &mut MappingsContext) -> Vec<u8> {
        let lines = self.number_of_lines;
        if lines > 0 {
            let mut mappings = Vec::<u8>::with_capacity(32);
            for _ in 0..lines {
                mappings.push(b';');
            }
            mappings_context.unfinished_generated_line =
                utils::get_unfinished_lines(&self.generated_code);
            if mappings_context.unfinished_generated_line > 0 {
                mappings.push(b'A');
            }
            mappings
        } else {
            let prev_unfinished = mappings_context.unfinished_generated_line;
            mappings_context.unfinished_generated_line +=
                utils::get_unfinished_lines(&self.generated_code);

            if prev_unfinished == 0 && mappings_context.unfinished_generated_line > 0 {
                vec![b'A']
            } else {
                Vec::new()
            }
        }
    }

    pub fn get_normalized_nodes(self) -> Vec<Node> {
        vec![Node::NCodeNode(self)]
    }
}
