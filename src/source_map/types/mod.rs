mod mapping;
mod mapping_list;

pub use super::types::mapping::*;
pub use super::types::mapping_list::*;

use source_map::SourceNode;
use types::string_slice::StringSlice;

#[derive(Clone, Debug)]
pub enum Node {
    NSourceNode(SourceNode),
    NString(StringSlice),
    NNodeVec(Vec<Node>),
}
