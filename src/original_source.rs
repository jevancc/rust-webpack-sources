use SourceNode;

fn split_code(code: &str) -> Vec<&str> {
    code.split(&['\n', '\r', ';', '{', '}'][..]).collect()
}

pub struct OriginalSource {
    value: String,
    name: String,
}

impl OriginalSource {
    pub fn new(value: String, name: String) -> OriginalSource {
        OriginalSource {
            value,
            name
        }
    }

    pub fn node(&self, columns: bool) -> SourceNode {
        let mut sn = SourceNode::new_null_null_null();
        let mut lines = self.value.split('\n').enumerate().peekable();

        while let Some((idx, line)) = lines.next() {
            let content = String::from(line) + if lines.peek() == None {
                "\n"
            } else {
                ""
            };

            if !columns {
                let mut sn2 =
                    SourceNode::new_number_number_string(idx as u32 + 1, 0, &self.name);
                sn2.add_string(&content);
                sn.add_sourcenode(&sn2);
            } else {
                let mut sn2 = SourceNode::new_null_null_null();
                let mut pos: usize = 0;
                let splitted_codes = split_code(&content);
                for item in splitted_codes.iter() {
                    if item.trim().len() == 0 {
                        sn2.add_string(item);
                    } else {
                        let mut sn3 = SourceNode::new_number_number_string(idx as u32 + 1, pos as u32, &self.name);
                        sn3.add_string(item);
                        pos += item.len();
                        sn2.add_sourcenode(&sn3);
                    }
                }
                sn.add_sourcenode(&sn2)
            }
        }
        sn.setSourceContent(&self.name, &self.value);
        sn
    }

    pub fn size(&self) -> usize {
        self.value.len()
    }

    pub fn source(&self) -> String {
        self.value.clone()
    }
}
