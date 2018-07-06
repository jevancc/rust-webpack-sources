use super::types::{Mapping, Node};
use super::utils::split_string;
use super::{SourceMapGenerator, SourceNode};
use types::StringPtr;

pub fn from_string_with_source_map(
    code: StringPtr,
    sources: Vec<StringPtr>,
    sources_content: Vec<StringPtr>,
    mappings: StringPtr,
    names: Vec<StringPtr>,
    file: Option<StringPtr>,
    source_root: Option<StringPtr>,
) -> SourceNode {
    let generator = SourceMapGenerator::from_source_map(
        sources,
        sources_content,
        mappings,
        names,
        file,
        source_root,
        true,
    );
    let mut node = SourceNode::new(None, None, None, None);

    let mut lines = code.get().split('\n').peekable();
    let mut last_generated_position: (usize, usize) = (1, 0);
    let mut last_mapping: Option<Mapping> = None;
    let mut shift_lines = || {
        let next = lines.next();
        next.map_or((String::new(), false), |line| {
            if lines.peek().is_some() {
                (String::from(line) + "\n", true)
            } else {
                (String::from(line), true)
            }
        })
    };
    let mut next_line = shift_lines();

    for mapping in generator.mappings.list.into_iter() {
        let generated_position = mapping.generated;
        if last_mapping.is_some() {
            if last_generated_position.0 < generated_position.0 {
                node.add_mapping_with_code(last_mapping, next_line.0);
                next_line = shift_lines();
                // line++, column = 0
                last_generated_position.0 += 1;
                last_generated_position.1 = 0;
            } else {
                let splitted = split_string(
                    next_line.0,
                    generated_position.1 as i32 - last_generated_position.1 as i32,
                    None,
                );
                let code = splitted.0;
                next_line.0 = splitted.1;
                last_generated_position.1 = generated_position.1;

                node.add_mapping_with_code(last_mapping, code);
                last_mapping = Some(mapping);
                continue;
            }
        }

        while last_generated_position.0 < generated_position.0 {
            node.add(Node::NString(next_line.0));
            next_line = shift_lines();
            last_generated_position.0 += 1;
        }
        if last_generated_position.1 < generated_position.1 {
            let splitted = split_string(next_line.0, generated_position.1 as i32, None);
            node.add(Node::NString(splitted.0));
            next_line.0 = splitted.1;
            last_generated_position.1 = generated_position.1;
        }
        last_mapping = Some(mapping);
    }

    if next_line.1 {
        if last_mapping.is_some() {
            node.add_mapping_with_code(last_mapping, next_line.0);
            next_line = shift_lines();
        }
        let mut remaining = String::new();
        while next_line.1 {
            remaining += &next_line.0;
            next_line = shift_lines();
        }
        node.add(Node::NString(remaining));
    }
    node.source_contents = generator.sources_contents;
    node
}
