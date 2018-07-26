mod code_node;
mod from_string_with_source_map;
mod mapping_function;
mod mappings_context;
mod single_line_node;
mod source_list_map;
mod source_node;
mod utils;

pub use source_list_map::code_node::CodeNode;
pub use source_list_map::from_string_with_source_map::from_string_with_source_map;
pub use source_list_map::mapping_function::MappingFunction;
use source_list_map::mappings_context::MappingsContext;
pub use source_list_map::single_line_node::SingleLineNode;
pub use source_list_map::source_list_map::SourceListMap;
pub use source_list_map::source_node::SourceNode;

pub mod types {
    use super::{CodeNode, SingleLineNode, SourceListMap, SourceNode};
    use types::string_slice::StringSlice;

    #[derive(Debug, Clone)]
    pub enum Node {
        NString(StringSlice),
        NCodeNode(CodeNode),
        NSourceNode(SourceNode),
        NSingleLineNode(SingleLineNode),
        NSourceListMap(SourceListMap),
        NStringIdx(i32),
    }

    #[derive(Debug)]
    pub enum GenCode {
        Code(Node),
        CodeVec(Vec<Node>),
    }
}
