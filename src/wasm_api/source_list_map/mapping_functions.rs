use source_list_map::*;

pub struct IdenticalFunction;
impl MappingFunction for IdenticalFunction {
    fn map(&mut self, code: String) -> String {
        code
    }
}

pub struct TestMappingFunction;
impl MappingFunction for TestMappingFunction {
    fn map(&mut self, code: String) -> String {
        code.replace(";", "\n")
            .replace("\\\n", " ")
            .replace("$\n", "")
    }
}

pub struct PrefixMappingFunction {
    pub prefix: String,
}

impl MappingFunction for PrefixMappingFunction {
    fn map(&mut self, code: String) -> String {
        let replace = String::from("\n") + &self.prefix;
        let mut result = self.prefix.clone();

        let lines = code.split('\n');
        let lines_count = lines.clone().count();
        let mut it = lines.enumerate();
        while let Some((idx, line)) = it.next() {
            match idx {
                n if n == lines_count - 2 => {
                    result += line;
                }
                n if n == lines_count - 1 => {
                    if line.is_empty() {
                        result += "\n";
                    } else {
                        result += &replace;
                    }
                    result += line;
                }
                _ => {
                    result += line;
                    result += &replace;
                }
            }
        }

        result
    }
}
