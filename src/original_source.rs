use source::SourceTrait;
use source_list_map::{types::GenCode, types::Node as SlmNode, SourceListMap};
use source_map::{types::Node as SmNode, SourceNode};
use std::rc::Rc;
use types::StringPtr;

#[derive(Debug)]
pub struct OriginalSource {
    pub value: Rc<String>,
    pub name: i32,
    pub value_idx: i32,
}

impl OriginalSource {
    pub fn new(value: String, value_idx: i32, name: i32) -> OriginalSource {
        OriginalSource {
            value: Rc::new(value),
            value_idx,
            name,
        }
    }
}

impl SourceTrait for OriginalSource {
    fn size(&mut self) -> usize {
        self.value.len()
    }

    fn source(&mut self) -> Rc<String> {
        self.value.clone()
    }

    fn node(&mut self, columns: bool, _module: bool) -> SourceNode {
        let mut sn = SourceNode::new(None, None, None, None);
        let mut lines = self.value.split('\n').enumerate().peekable();

        while let Some((idx, line)) = lines.next() {
            let content = String::from(line) + if lines.peek().is_some() { "\n" } else { "" };
            if !columns {
                sn.add(SmNode::NSourceNode(SourceNode::new(
                    Some((idx + 1, 0)),
                    Some(self.name.clone()),
                    None,
                    Some(SmNode::NString(content)),
                )));
            } else {
                let mut sn2 = SourceNode::new(None, None, None, None);
                let mut pos: usize = 0;
                let splitted_codes = split_code(&content);
                for item in &splitted_codes {
                    if item.trim().is_empty() {
                        sn2.add(SmNode::NString(String::from(*item)));
                    } else {
                        sn2.add(SmNode::NSourceNode(SourceNode::new(
                            Some((idx + 1, pos)),
                            Some(self.name.clone()),
                            None,
                            Some(SmNode::NString(String::from(*item))),
                        )));
                        pos += item.len();
                    }
                }
                sn.add(SmNode::NSourceNode(sn2))
            }
        }
        sn.set_source_content(self.name.clone(), self.value_idx.clone());
        sn
    }

    fn list_map(&mut self, _columns: bool, _module: bool) -> SourceListMap {
        SourceListMap::new(
            Some(GenCode::Code(SlmNode::NRcString(self.value.clone()))),
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
fn split_code(mut code: &str) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new();
    while !code.is_empty() {
        let chars = code.char_indices();
        let mut chars = chars
            .skip_while(|c| !is_splitter(c.1))
            .skip_while(|c| is_splitter(c.1));
        if let Some((pos, _)) = chars.next() {
            let splitted = code.split_at(pos);
            result.push(splitted.0);
            code = splitted.1;
        } else {
            result.push(code);
            code = "";
        }
    }
    result
}
