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
        lines.join(&replace)
    }
}
