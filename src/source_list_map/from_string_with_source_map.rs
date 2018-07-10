use super::types::{GenCode, Node};
use super::{CodeNode, SourceListMap, SourceNode};
use types::StringPtr;
use vlq;

pub fn from_string_with_source_map(
    code: StringPtr,
    sources: Vec<i32>,
    sources_content: Vec<i32>,
    mappings: StringPtr,
) -> SourceListMap {
    let mappings = mappings.get().split(';').enumerate();
    let mut lines = code.get().split('\n').enumerate();
    let lines_count = lines.clone().count();
    let mut nodes: Vec<Node> = vec![];

    let mut current_line: i64 = 1;
    let mut current_source_index: usize = 0;
    let mut current_source_node_line: usize = 0;

    for (i, mapping) in mappings {
        if let Some((_, line)) = lines.next() {
            let line = if i != lines_count - 1 {
                String::from(line) + "\n"
            } else {
                String::from(line)
            };
            if !mapping.is_empty() {
                let mut line_added: bool = false;
                let mut rest = mapping.as_bytes().iter().cloned().peekable();

                while let Some(_) = rest.peek() {
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

    let mut last = String::new();
    while let Some((i, line)) = lines.next() {
        if i < lines_count - 1 && line.trim().is_empty() {
            let line = String::from(line) + "\n";
            add_code(&mut nodes, &mut current_source_node_line, line);
        } else {
            last += line;
            while let Some((_, line)) = lines.next() {
                last += "\n";
                last += line;
            }
            add_code(&mut nodes, &mut current_source_node_line, last);
            break;
        }
    }
    SourceListMap::new(Some(GenCode::CodeVec(nodes)), None, None)
}

#[inline]
fn add_code(nodes: &mut Vec<Node>, current_source_node_line: &mut usize, generated_code: String) {
    match nodes.last_mut() {
        Some(Node::NCodeNode(ref mut n)) => {
            n.add_generated_code(&generated_code);
            return;
        }
        Some(Node::NSourceNode(ref mut n)) => {
            if generated_code.trim().is_empty() {
                n.add_generated_code(&generated_code);
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
    generated_code: String,
    source: Option<i32>,
    original_source: Option<i32>,
    line: usize,
) {
    if let Some(Node::NSourceNode(ref mut n)) = nodes.last_mut() {
        if n.source == source && *current_source_node_line == line {
            n.add_generated_code(&generated_code);
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
