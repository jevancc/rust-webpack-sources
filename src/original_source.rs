use source_map::{SourceNode, StringPtr as SMStringPtr, Node as SMNode};
use source_list_map::{SourceListMap, GenCode, StringPtr as SLMStringPtr, Node as SLMNode};
use source::{SourceTrait};

#[inline]
fn is_splitter(c: char) -> bool {
    match c {
        '\n' | '\r' | ';' | '{' | '}' => true,
        _ => false,
    }
}

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

pub struct OriginalSource {
    pub value: String,
    pub name: String,
}

impl OriginalSource {
    pub fn new(value: String, name: String) -> OriginalSource {
        OriginalSource { value, name }
    }
}

impl SourceTrait for OriginalSource {
    fn size(&mut self) -> usize {
        self.value.len()
    }

    fn source(&mut self) -> String {
        self.value.clone()
    }

    fn node(&mut self, columns: bool, _module: bool) -> SourceNode {
        let mut sn = SourceNode::new(None, None, None, None);
        let mut lines = self.value.split('\n').enumerate().peekable();

        while let Some((idx, line)) = lines.next() {
            let content = String::from(line) + if lines.peek().is_some() { "\n" } else { "" };

            if !columns {
                sn.add(
                    SMNode::NSourceNode(
                        SourceNode::new(
                            Some((idx + 1, 0)),
                            Some(SMStringPtr::Str(self.name.clone())),
                            None,
                            Some(SMNode::NString(content))
                        )
                    )
                );
            } else {
                let mut sn2 = SourceNode::new(None, None, None, None);
                let mut pos: usize = 0;
                let splitted_codes = split_code(&content);
                for item in &splitted_codes {
                    if item.trim().is_empty() {
                        sn2.add(SMNode::NString(String::from(*item)));
                    } else {
                        pos += item.len();
                        sn2.add(SMNode::NSourceNode(SourceNode::new(
                            Some((idx + 1, pos)),
                            Some(SMStringPtr::Str(self.name.clone())),
                            None,
                            Some(SMNode::NString(String::from(*item)))
                        )));
                    }
                }
                sn.add(SMNode::NSourceNode(sn2))
            }
        }
        sn.set_source_content(SMStringPtr::Str(self.name.clone()), SMStringPtr::Str(self.value.clone()));
        sn
    }

    fn list_map(&mut self, _columns: bool, _module: bool) -> SourceListMap {
        SourceListMap::new(
            Some(GenCode::Code(SLMNode::NString(self.value.clone()))),
            Some(SLMStringPtr::Str(self.name.clone())),
            Some(SLMStringPtr::Str(self.value.clone()))
        )
    }
}
