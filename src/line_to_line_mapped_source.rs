use source::SourceTrait;
use source_list_map::{types::GenCode, types::Node as SlmNode, SourceListMap};
use source_map::{types::Node as SmNode, SourceNode};
use types::string_slice::*;

#[derive(Debug)]
pub struct LineToLineMappedSource {
    value: StringSlice,
    name: i32,
    original_source: i32,
}

impl LineToLineMappedSource {
    pub fn new(value: String, name: i32, original_source: i32) -> LineToLineMappedSource {
        LineToLineMappedSource {
            value: StringSlice::from(value),
            name,
            original_source,
        }
    }
}

impl SourceTrait for LineToLineMappedSource {
    fn source(&mut self) -> StringSlice {
        self.value.clone()
    }

    fn size(&mut self) -> usize {
        self.value.len()
    }

    fn node(&mut self, _columns: bool, _module: bool) -> SourceNode {
        let mut chunks = Vec::<SmNode>::new();

        let mut code = self.value.clone();
        let mut line_start = 0;
        let mut current_line = 1;
        let code_len = code.len();

        while line_start < code_len {
            let line_end = if let Some(pos) = code.find('\n') {
                pos + 1
            } else {
                code_len - line_start
            };
            let (line, rest) = code.split_at(line_end);
            chunks.push(SmNode::NSourceNode(SourceNode::new(
                Some((current_line, 0)),
                Some(self.name.clone()),
                None,
                Some(SmNode::NString(line)),
            )));
            code = rest;
            line_start += line_end;
            current_line += 1;
        }

        let mut node = SourceNode::new(None, None, None, Some(SmNode::NNodeVec(chunks)));
        node.set_source_content(self.name.clone(), self.original_source.clone());
        node
    }

    fn list_map(&mut self, _columns: bool, _module: bool) -> SourceListMap {
        SourceListMap::new(
            Some(GenCode::Code(SlmNode::NString(self.value.clone()))),
            Some(self.name.clone()),
            Some(self.original_source.clone()),
        )
    }
}
