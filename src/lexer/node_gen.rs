use super::structures::*;
use regex::Regex;

#[inline(always)]
pub fn normal_line(line: &str) -> Node {
    Node {
        r#type: NodeType::NormalLine,
        info: NodeInfo {
            string: Some(String::from(line)),
            attributes: get_attributes(line),
        },
        include_next_line: !(line.ends_with("  ") || line.ends_with('\\')),
        allow_merge: true,
    }
}

#[inline(always)]
pub fn new_line() -> Node {
    Node {
        r#type: NodeType::NewLine,
        info: NodeInfo {
            string: None,
            attributes: None,
        },
        include_next_line: false,
        allow_merge: false,
    }
}

#[inline(always)]
pub fn list_node(line: &str) -> Node {
    let is_unsorted_list = Regex::new("^-\\s+").unwrap().is_match(line);
    Node {
        r#type: if is_unsorted_list {
            NodeType::UnsortedList
        } else {
            NodeType::SortedList(
                Regex::new("^(\\d+)").unwrap().captures(line).unwrap()[0]
                    .parse::<usize>()
                    .unwrap(),
            )
        },
        info: NodeInfo {
            string: Some(String::from(line)),
            attributes: get_attributes(line),
        },
        include_next_line: !(line.ends_with("  ") || line.ends_with('\\')),
        allow_merge: false,
    }
}

#[inline(always)]
pub fn header_node(line: &str) -> Node {
    Node {
        r#type: NodeType::Header(line.find(' ').unwrap()),
        info: NodeInfo {
            string: Some(String::from(line)),
            attributes: get_attributes(line),
        },
        include_next_line: false,
        allow_merge: false,
    }
}

#[inline(always)]
pub fn merge_nodes(last_node: Node, current_node: Node) -> Node {
    let string = push_new_str(
        last_node.info.string.unwrap(),
        current_node.info.string.unwrap(),
    );

    Node {
        r#type: last_node.r#type,
        info: NodeInfo {
            string: Some(string.clone()),
            attributes: get_attributes(&string),
        },
        include_next_line: current_node.include_next_line,
        allow_merge: last_node.allow_merge,
    }
}

#[inline(always)]
fn get_attributes(line: &str) -> Option<TextAttributes> {
    Some(TextAttributes {
        image_or_link: Regex::new("\\[(.*)\\]\\((.*)\\)")
            .unwrap()
            .is_match(line),
        strike: Regex::new("~~(.*)~~").unwrap().is_match(line),
        bold_or_italics: Regex::new("\\*(.*)\\*|_(.*)_")
            .unwrap()
            .is_match(line),
    })
}

#[inline(always)]
fn push_new_str(mut first_string: String, string: String) -> String {
    first_string.push_str(&string);

    first_string
}
