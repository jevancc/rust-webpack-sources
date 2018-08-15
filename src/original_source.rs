use regex::bytes::Regex;
use source::SourceTrait;
use source_list_map::{types::GenCode, types::Node as SlmNode, SourceListMap};
use source_map::{types::Node as SmNode, SourceNode};
use types::string_slice::*;

lazy_static! {
    static ref SPLITTER_REGEX: Regex = Regex::new(r"[^\n\r;{}]*[\n\r;{}]*").unwrap();
}

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

        for (i, mut line) in self.value.split_keep_seperator(b'\n').enumerate() {
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

                while !line.is_empty() {
                    let p = SPLITTER_REGEX
                        .find(&line.as_bytes())
                        .map_or(line.len(), |m| m.end());
                    let splitted = line.split_at(p);
                    let item = splitted.0;
                    line = splitted.1;

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
