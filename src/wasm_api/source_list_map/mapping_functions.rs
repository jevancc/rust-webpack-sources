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
        let lines: Vec<&str> = code.split('\n').collect();

        let mut result = self.prefix.clone();
        let mut it = lines.iter().enumerate();
        while let Some((idx, line)) = it.next() {
            result += line;
            if idx < lines.len() - 2 {
                result += &replace;
            } else if idx == lines.len() - 1 {
                if line.is_empty() {
                    result += "\n";
                } else {
                    result += &replace;
                }
            }
        }

        // while let Some(line) = lines.next() {
        //     result += line;
        //     if lines.peek() != None || !line.is_empty() {
        //         result += &replace;
        //     }
        // }
        result
    }
}
