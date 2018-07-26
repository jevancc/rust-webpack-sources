use source::SourceTrait;
use source_list_map::{types::GenCode, types::Node as SlmNode, SourceListMap};
use source_map::{types::Node as SmNode, SourceNode};
use types::string_slice::*;

#[derive(Debug)]
pub struct OriginalSource {
    pub value: StringSlice,
    pub name: i32,
    pub value_idx: i32,
}

impl OriginalSource {
    pub fn new(value: String, value_idx: i32, name: i32) -> OriginalSource {
        OriginalSource {
            value: StringSlice::from(value),
            value_idx,
            name,
        }
    }
}

impl SourceTrait for OriginalSource {
    fn size(&mut self) -> usize {
        self.value.len()
    }

    fn source(&mut self) -> StringSlice {
        self.value.clone()
    }

    fn node(&mut self, columns: bool, _module: bool) -> SourceNode {
        let mut sn = SourceNode::new(None, None, None, None);

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
            if !columns {
                sn.add(SmNode::NSourceNode(SourceNode::new(
                    Some((current_line, 0)),
                    Some(self.name.clone()),
                    None,
                    Some(SmNode::NString(line)),
                )));
            } else {
                let mut sn2 = SourceNode::new(None, None, None, None);
                let mut pos: usize = 0;
                let splitted_codes = split_code(line);
                for item in splitted_codes.into_iter() {
                    if item.trim().is_empty() {
                        sn2.add(SmNode::NString(item));
                    } else {
                        pos += item.len();
                        sn2.add(SmNode::NSourceNode(SourceNode::new(
                            Some((current_line, pos)),
                            Some(self.name.clone()),
                            None,
                            Some(SmNode::NString(item)),
                        )));
                    }
                }
                sn.add(SmNode::NSourceNode(sn2))
            }
            code = rest;
            line_start += line_end;
            current_line += 1;
        }

        sn.set_source_content(self.name.clone(), self.value_idx.clone());
        sn
    }

    fn list_map(&mut self, _columns: bool, _module: bool) -> SourceListMap {
        SourceListMap::new(
            Some(GenCode::Code(SlmNode::NString(self.value.clone()))),
            Some(self.name.clone()),
            Some(self.value_idx.clone()),
        )
    }
}

#[inline]
fn is_splitter(c: char) -> bool {
    match c {
        '\n' | '\r' | ';' | '{' | '}' => true,
        _ => false,
    }
}

#[inline]
fn get_split_pos(code: &StringSlice) -> Option<usize> {
    let chars = code.char_indices();
    let mut chars = chars
        .skip_while(|c| !is_splitter(c.1))
        .skip_while(|c| is_splitter(c.1));
    chars.next().map(|(pos, _)| pos)
}

fn split_code(mut code: StringSlice) -> Vec<StringSlice> {
    let mut result: Vec<StringSlice> = Vec::new();
    while code.len() != 0 {
        let split_pos = get_split_pos(&code);
        if let Some(pos) = split_pos {
            let splitted = code.split_at(pos);
            result.push(splitted.0);
            code = splitted.1;
        } else {
            result.push(code);
            break;
        }
    }
    result
}
