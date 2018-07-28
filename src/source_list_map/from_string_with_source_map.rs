use super::types::{GenCode, Node};
use super::{CodeNode, SourceListMap, SourceNode};
use types::string_slice::*;
use vlq;

pub fn from_string_with_source_map(
    code: StringSlice,
    sources: Vec<i32>,
    sources_content: Vec<i32>,
    mappings: StringSlice,
) -> SourceListMap {
    let mut nodes: Vec<Node> = Vec::new();

    let mut current_line: i64 = 1;
    let mut current_source_index: usize = 0;
    let mut current_source_node_line: usize = 0;

    let mut lines = code.split_keep_seperator('\n');
    for mapping in mappings.split(';') {
        if let Some(line) = lines.next() {
            if !mapping.is_empty() {
                let mut line_added: bool = false;
                let mut rest = mapping.as_bytes().iter().cloned().peekable();

                while rest.peek().is_some() {
                    line_added = {
                        if let Some(c) = rest.clone().peek() {
                            if *c != b',' {
                                vlq::decode(&mut rest).unwrap();
                            }
                        }

                        match rest.clone().peek() {
                            None => false,
                            Some(c) => {
                                if *c == b',' {
                                    rest.next();
                                    false
                                } else {
                                    let value = vlq::decode(&mut rest).unwrap();
                                    let source_index = value as usize + current_source_index;
                                    current_source_index = source_index;

                                    let mut line_position: i64;
                                    if let Some(c) = rest.clone().peek() {
                                        if *c != b',' {
                                            let value = vlq::decode(&mut rest).unwrap();
                                            line_position = value + current_line as i64;
                                            current_line = line_position;
                                        } else {
                                            line_position = current_line;
                                        }
                                    } else {
                                        line_position = current_line;
                                    }

                                    while let Some(c) = rest.clone().peek() {
                                        if *c != b',' {
                                            rest.next();
                                        } else {
                                            break;
                                        }
                                    }

                                    if !line_added {
                                        add_source(
                                            &mut nodes,
                                            &mut current_source_node_line,
                                            line.clone(),
                                            sources.get(source_index).map(|idx| *idx),
                                            sources_content.get(source_index).map(|idx| *idx),
                                            line_position as usize,
                                        );
                                        true
                                    } else {
                                        false
                                    }
                                }
                            }
                        }
                    } || line_added;
                }
                if !line_added {
                    add_code(&mut nodes, &mut current_source_node_line, line);
                }
            } else {
                add_code(&mut nodes, &mut current_source_node_line, line);
            }
        }
    }

    while let Some(line) = lines.next() {
        if line.clone().offset(-1).trim().is_empty() {
            add_code(&mut nodes, &mut current_source_node_line, line);
        } else {
            let last = if let Some(last) = lines.rest() {
                last.offset(-(line.len() as isize))
            } else {
                line
            };
            add_code(&mut nodes, &mut current_source_node_line, last);
        }
    }
    SourceListMap::new(Some(GenCode::CodeVec(nodes)), None, None)
}

#[inline]
fn add_code(
    nodes: &mut Vec<Node>,
    current_source_node_line: &mut usize,
    generated_code: StringSlice,
) {
    match nodes.last_mut() {
        Some(Node::NCodeNode(ref mut n)) => {
            n.add_generated_code(&generated_code);
            return;
        }
        Some(Node::NSourceNode(ref mut n)) => {
            if generated_code.trim().is_empty() {
                n.add_generated_code(generated_code);
                *current_source_node_line += 1;
                return;
            }
        }
        _ => {}
    }
    nodes.push(Node::NCodeNode(CodeNode::new(generated_code)));
}

#[inline]
fn add_source(
    nodes: &mut Vec<Node>,
    current_source_node_line: &mut usize,
    generated_code: StringSlice,
    source: Option<i32>,
    original_source: Option<i32>,
    line: usize,
) {
    if let Some(Node::NSourceNode(ref mut n)) = nodes.last_mut() {
        if n.source == source && *current_source_node_line == line {
            n.add_generated_code(generated_code);
            *current_source_node_line += 1;
            return;
        }
    }
    nodes.push(Node::NSourceNode(SourceNode::new(
        generated_code,
        source,
        original_source,
        line,
    )));
    *current_source_node_line = line + 1;
}
