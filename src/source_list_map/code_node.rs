use super::types::Node;
use super::{utils, MappingFunction, MappingsContext};
use std::str;
use types::string_slice::*;

#[derive(Clone, Debug)]
pub struct CodeNode {
    generated_code: String,
}

impl CodeNode {
    pub fn new(generated_code: StringSlice) -> Self {
        CodeNode {
            generated_code: generated_code.into_string(),
        }
    }

    pub fn add_generated_code(&mut self, generated_code: &str) {
        self.generated_code += generated_code;
    }

    pub fn map_generated_code<T: MappingFunction>(self, mf: &mut T) -> CodeNode {
        let generated_code = mf.map(self.generated_code);
        CodeNode::new(StringSlice::from(generated_code))
    }

    pub fn merge(mut self, other_node: &Node) -> Result<Node, Node> {
        match other_node {
            Node::NCodeNode(n) => {
                self.generated_code += &n.generated_code;
                Ok(Node::NCodeNode(self))
            }
            _ => Err(Node::NCodeNode(self)),
        }
    }

    pub fn get_generated_code(&self) -> &str {
        &self.generated_code
    }

    pub fn get_mappings(&self, mappings_context: &mut MappingsContext) -> String {
        let lines = utils::number_of_lines(&self.generated_code);
        let mut mappings: String = ";".repeat(lines);

        if lines > 0 {
            mappings_context.unfinished_generated_line =
                utils::get_unfinished_lines(&self.generated_code);
            if mappings_context.unfinished_generated_line > 0 {
                mappings += "A";
            }
        } else {
            let prev_unfinished = mappings_context.unfinished_generated_line;
            mappings_context.unfinished_generated_line +=
                utils::get_unfinished_lines(&self.generated_code);
            if prev_unfinished == 0 && mappings_context.unfinished_generated_line > 0 {
                mappings = String::from("A");
            } else {
                mappings = String::new();
            }
        }
        mappings
    }

    pub fn get_normalized_nodes(self) -> Vec<CodeNode> {
        vec![self]
    }
}
