use super::structures::*;
use lazy_static::lazy_static;
use regex::Regex;

#[inline(always)]
pub(crate) fn normal_line(line: &str) -> Node {
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
pub(crate) fn new_line() -> Node {
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
pub(crate) fn list_node(line: &str) -> Node {
    lazy_static! {
        static ref IS_UNSORTED_LIST: Regex = Regex::new(r"^-\s+").unwrap();
        static ref SORTED_LIST: Regex = Regex::new(r"^(\d+)").unwrap();
    };
    Node {
        r#type: if IS_UNSORTED_LIST.is_match(line) {
            NodeType::UnsortedList
        } else {
            NodeType::SortedList(
                SORTED_LIST.captures(line).unwrap()[0]
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
pub(crate) fn header_node(line: &str) -> Node {
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
pub(crate) fn merge_nodes(old_node: Node, current_node: Node) -> Node {
    let string = push_new_str(
        old_node.info.string.unwrap(),
        current_node.info.string.unwrap(),
    );

    Node {
        r#type: old_node.r#type,
        info: NodeInfo {
            string: Some(string.clone()),
            attributes: get_attributes(&string),
        },
        include_next_line: current_node.include_next_line,
        allow_merge: old_node.allow_merge,
    }
}

#[inline(always)]
fn get_attributes(line: &str) -> Option<TextAttributes> {
    lazy_static! {
        static ref INLINE_CODE: Regex = Regex::new("`(.*)`").unwrap();
        static ref IMAGE_OR_LINK: Regex =
            Regex::new(r"\[(.*)\]\((.*)\)").unwrap();
        static ref STRIKE: Regex = Regex::new(r"~~(.*)~~").unwrap();
        static ref BOLD_OR_ITALICS: Regex = Regex::new(r"\*(.*)\*").unwrap();
    }
    Some(TextAttributes {
        inline_code: INLINE_CODE.is_match(line),
        image_or_link: IMAGE_OR_LINK.is_match(line),
        strike: STRIKE.is_match(line),
        bold_or_italics: BOLD_OR_ITALICS.is_match(line),
    })
}

#[inline(always)]
fn push_new_str(mut first_string: String, string: String) -> String {
    first_string.push_str(&string);

    first_string
}
