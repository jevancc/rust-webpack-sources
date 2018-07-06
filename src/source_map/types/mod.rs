mod mapping;
mod mapping_list;

pub use super::types::mapping::*;
pub use super::types::mapping_list::*;

use source_map::SourceNode;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Node {
    NSourceNode(SourceNode),
    NString(String),
    NRcString(Rc<String>),
    NNodeVec(Vec<Node>),
}
