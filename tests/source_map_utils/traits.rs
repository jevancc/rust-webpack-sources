use source_map_utils::string_cache::*;
use webpack_sources::source_map::*;
use webpack_sources::types::string_slice::*;
use webpack_sources::types::*;

pub trait SourceNodeTest {
    fn add_sources(
        &mut self,
        params: &[(i32, i32, &str, &str, Option<&str>)],
        str_cache: &mut StringCache,
    );
}

impl SourceNodeTest for SourceNode {
    fn add_sources(
        &mut self,
        params: &[(i32, i32, &str, &str, Option<&str>)],
        str_cache: &mut StringCache,
    ) {
        for param in params {
            let (line, column, source, chunk, name) = *param;
            let name = name.map(|s| str_cache.add(s));
            if line >= 0 {
                self.add(types::Node::NSourceNode(SourceNode::new(
                    Some((line as usize, column as usize)),
                    Some(str_cache.add(source)),
                    name,
                    Some(types::Node::NString(StringSlice::from(chunk))),
                )));
            } else {
                self.add(types::Node::NString(StringSlice::from(chunk)));
            }
        }
    }
}

pub trait SourceMapGeneratorTest {
    fn add_mappings(
        &mut self,
        params: &[(i32, i32, Option<&str>, i32, i32, Option<&str>)],
        str_cache: &mut StringCache,
    );
}

impl SourceMapGeneratorTest for SourceMapGenerator {
    fn add_mappings(
        &mut self,
        params: &[(i32, i32, Option<&str>, i32, i32, Option<&str>)],
        str_cache: &mut StringCache,
    ) {
        for param in params {
            let generated = (param.0 as usize, param.1 as usize);
            let source = param.2.map(|s| str_cache.add(s));
            let original = if param.3 >= 0 {
                Some((param.3 as usize, param.4 as usize))
            } else {
                None
            };
            let name = param.5.map(|s| str_cache.add(s));
            self.add_mapping(types::Mapping {
                generated,
                source,
                original,
                name,
            });
        }
    }
}

pub trait SourceMapTest {
    fn to_generator(&self) -> SourceMapGenerator;
}

impl SourceMapTest for SourceMap {
    fn to_generator(&self) -> SourceMapGenerator {
        let map = self.clone();
        SourceMapGenerator::from_source_map(
            map.sources,
            map.sources_content,
            StringSlice::from(map.mappings),
            map.names,
            map.file,
            map.source_root.map(|s| StringSlice::from(s)),
            true,
        )
    }
}
