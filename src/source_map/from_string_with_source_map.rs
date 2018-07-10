use super::types::{Mapping, Node};
use super::{SourceMapGenerator, SourceNode};
use types::StringPtr;
use utils;

pub fn from_string_with_source_map(
    code: StringPtr,
    sources: Vec<i32>,
    sources_content: Vec<i32>,
    mappings: StringPtr,
    names: Vec<i32>,
    file: Option<i32>,
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

    let code = code.get();
    let code_len = code.len();
    let mut lines = Vec::<(&str, bool, usize)>::new();
    {
        let mut line_start = 0;
        let mut line_len = 0;
        for (pos, c) in code.char_indices() {
            line_len += 1;
            if c == '\n' {
                let line_end = pos + 1;
                lines.push((&code[line_start..line_end], true, line_len));
                line_start = line_end;
                line_len = 0;
            }
        }
        if line_start != code_len {
            lines.push((&code[line_start..code_len], true, line_len));
        }
    }

    let mut last_generated_position: (usize, usize) = (1, 0);
    let mut last_mapping: Option<Mapping> = None;
    let mut line_iter = lines.into_iter();
    let mut next_line: (&str, bool, usize) = line_iter.next().unwrap_or(("", false, 0));

    for mapping in generator.mappings.list.into_iter() {
        let generated_position = mapping.generated;
        if last_mapping.is_some() {
            if last_generated_position.0 < generated_position.0 {
                node.add_mapping_with_code(last_mapping, next_line.0);
                next_line = line_iter.next().unwrap_or(("", false, 0));
                // line++, column = 0
                last_generated_position.0 += 1;
                last_generated_position.1 = 0;
            } else {
                let splitted = utils::split_str(
                    next_line.0,
                    generated_position.1 as i32 - last_generated_position.1 as i32,
                    Some(next_line.2),
                );
                let code = splitted.0;
                next_line.0 = splitted.1;
                next_line.2 = splitted.3;
                last_generated_position.1 = generated_position.1;
                node.add_mapping_with_code(last_mapping, code);
                last_mapping = Some(mapping);
                continue;
            }
        }

        while last_generated_position.0 < generated_position.0 {
            node.add(Node::NString(String::from(next_line.0)));
            next_line = line_iter.next().unwrap_or(("", false, 0));
            last_generated_position.0 += 1;
        }
        if last_generated_position.1 < generated_position.1 {
            let splitted =
                utils::split_str(next_line.0, generated_position.1 as i32, Some(next_line.2));
            node.add(Node::NString(String::from(splitted.0)));
            next_line.0 = splitted.1;
            next_line.2 = splitted.3; // new len
            last_generated_position.1 = generated_position.1;
        }
        last_mapping = Some(mapping);
    }

    if next_line.1 {
        if last_mapping.is_some() {
            node.add_mapping_with_code(last_mapping, next_line.0);
            next_line = line_iter.next().unwrap_or(("", false, 0));
        }
        let mut remaining = String::new();
        while next_line.1 {
            remaining += &next_line.0;
            next_line = line_iter.next().unwrap_or(("", false, 0));
        }
        node.add(Node::NString(remaining));
    }
    node.source_contents = generator.sources_contents;
    node
}
