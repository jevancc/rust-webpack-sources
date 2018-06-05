use SourceNode;

fn split_code(mut code: &str) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new();
    while !code.is_empty() {
        let pos = code.find(&['\n', '\r', ';', '{', '}'][..]);
        if let Some(pos) = pos {
            let splitted = code.split_at(pos + 1);
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

    pub fn node(&self, columns: bool) -> SourceNode {
        let sn = SourceNode::new_null_null_null();
        let mut lines = self.value.split('\n').enumerate().peekable();

        while let Some((idx, line)) = lines.next() {
            let content = String::from(line) + if lines.peek() != None { "\n" } else { "" };

            if !columns {
                let mut sn2 = SourceNode::new_number_number_string(idx as u32 + 1, 0, &self.name);
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
                        let mut sn3 = SourceNode::new_number_number_string(
                            idx as u32 + 1,
                            pos as u32,
                            &self.name,
                        );
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
