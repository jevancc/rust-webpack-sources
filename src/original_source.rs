use source::SourceTrait;
use source_list_map::{types::GenCode, types::Node as SlmNode, SourceListMap};
use source_map::{types::Node as SmNode, SourceNode};
use types::string_slice::*;
use types::string_cat::*;

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

    fn source(&mut self) -> StringCat {
        StringCat::from(&self.value)
    }

    fn node(&mut self, columns: bool, _module: bool) -> SourceNode {
        let mut sn = SourceNode::new(None, None, None, None);

        for (i, line) in self.value.split_keep_seperator('\n').enumerate() {
            if !columns {
                sn.add(SmNode::NSourceNode(SourceNode::new(
                    Some((i + 1, 0)),
                    Some(self.name),
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
                        let len = item.len();
                        sn2.add(SmNode::NSourceNode(SourceNode::new(
                            Some((i + 1, pos)),
                            Some(self.name.clone()),
                            None,
                            Some(SmNode::NString(item)),
                        )));
                        pos += len;
                    }
                }
                sn.add(SmNode::NSourceNode(sn2))
            }
        }
        sn.set_source_content(self.name, self.value_idx);
        sn
    }

    fn list_map(&mut self, _columns: bool, _module: bool) -> SourceListMap {
        SourceListMap::new(
            Some(GenCode::Code(SlmNode::NString(self.value.clone()))),
            Some(self.name),
            Some(self.value_idx),
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
fn find_split_pos(code: &StringSlice) -> Option<usize> {
    let chars = code.char_indices();
    let mut chars = chars
        .skip_while(|c| !is_splitter(c.1))
        .skip_while(|c| is_splitter(c.1));
    chars.next().map(|(pos, _)| pos)
}

fn split_code(mut code: StringSlice) -> Vec<StringSlice> {
    let mut result: Vec<StringSlice> = Vec::new();
    while code.len() != 0 {
        let split_pos = find_split_pos(&code);
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
