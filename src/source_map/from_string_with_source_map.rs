use super::types::{Mapping, Node};
use super::{SourceMapGenerator, SourceNode};
use types::string_slice::StringSlice;
use utils;

pub fn from_string_with_source_map_generator(
    code: StringSlice,
    generator: &mut SourceMapGenerator,
) -> SourceNode {
    let mut node = SourceNode::new(None, None, None, None);

    let code_len = code.len();
    // line, is_existing, single_byte_char_only
    let mut lines = Vec::<(StringSlice, bool, bool)>::with_capacity(128);
    {
        let mut line_start = 0;
        let mut single_byte_char_only = true;
        for (p, c) in code.char_indices() {
            if c.len_utf8() > 1 {
                single_byte_char_only = false;
            }

            if c == '\n' {
                let line_end = p + 1;
                lines.push((
                    code.substr(line_start, line_end),
                    true,
                    single_byte_char_only,
                ));
                line_start = line_end;
                single_byte_char_only = true;
            }
        }
        if line_start != code_len {
            lines.push((
                code.substr(line_start, code_len),
                true,
                single_byte_char_only,
            ));
        }
    }

    let mut last_generated_position: (usize, usize) = (1, 0);
    let mut last_mapping: Option<Mapping> = None;
    let mut line_iter = lines.into_iter();
    let mut next_line: (StringSlice, bool, bool) =
        line_iter
            .next()
            .unwrap_or((StringSlice::new(), false, true));

    for mapping in &generator.mappings.list {
        let generated_position = mapping.generated;
        if last_mapping.is_some() {
            if last_generated_position.0 < generated_position.0 {
                node.add_mapping_with_code(last_mapping, next_line.0);
                next_line = line_iter
                    .next()
                    .unwrap_or((StringSlice::new(), false, true));
                // line++, column = 0
                last_generated_position.0 += 1;
                last_generated_position.1 = 0;
            } else {
                let splitted = utils::split_string_slice(
                    next_line.0,
                    generated_position.1 as i32 - last_generated_position.1 as i32,
                    next_line.2,
                ).unwrap();
                let code = splitted.0;
                next_line.0 = splitted.1;
                next_line.2 = splitted.3;
                last_generated_position.1 = generated_position.1;
                node.add_mapping_with_code(last_mapping, code);
                last_mapping = Some(mapping.clone());
                continue;
            }
        }

        while last_generated_position.0 < generated_position.0 {
            node.add(Node::NString(next_line.0));
            next_line = line_iter
                .next()
                .unwrap_or((StringSlice::new(), false, true));
            last_generated_position.0 += 1;
        }
        if last_generated_position.1 < generated_position.1 {
            let splitted =
                utils::split_string_slice(next_line.0, generated_position.1 as i32, next_line.2)
                    .unwrap();
            node.add(Node::NString(splitted.0));
            next_line.0 = splitted.1;
            next_line.2 = splitted.3; // new len
            last_generated_position.1 = generated_position.1;
        }
        last_mapping = Some(mapping.clone());
    }

    if next_line.1 {
        if last_mapping.is_some() {
            node.add_mapping_with_code(last_mapping, next_line.0);
            next_line = line_iter
                .next()
                .unwrap_or((StringSlice::new(), false, true));
        }
        let mut remaining = String::with_capacity(80);
        while next_line.1 {
            remaining.push_str(&next_line.0);
            next_line = line_iter
                .next()
                .unwrap_or((StringSlice::new(), false, true));
        }
        node.add(Node::NString(StringSlice::from(remaining)));
    }
    node.source_contents = generator.sources_contents.clone();
    node
}
