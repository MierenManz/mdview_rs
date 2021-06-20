use super::serializer::serialize;
use lazy_static::lazy_static;
use mdview_lexer::structures::Node;
use regex::Regex;

#[inline(always)]
pub(crate) fn header(node: Node, header: usize) -> String {
    let node_string = node.info.string.clone().unwrap();
    let string = &node_string[header + 1..];

    return format!("<h{0}>{1}</h{0}>", header, serialize(node, string));
}

#[inline(always)]
pub(crate) fn normal_line(node: Node) -> String {
    let node_string = node.info.string.clone().unwrap();
    return format!("<p>{}</p>", serialize(node, &node_string));
}

#[inline(always)]
pub(crate) fn new_line() -> String {
    String::from("<br>")
}

#[inline(always)]
pub(crate) fn unsorted_list(node: Node) -> String {
    let node_string = node.info.string.clone().unwrap();
    lazy_static! {
        static ref REPLACE_REG: Regex = Regex::new("^-\\s+(.*)").unwrap();
    };

    return format!(
        "<ul><li>{}</li></ul>",
        serialize(node, &REPLACE_REG.replace_all(&node_string, "$1"))
    );
}

#[inline(always)]
pub(crate) fn sorted_list(node: Node, list: usize) -> String {
    let node_string = node.info.string.clone().unwrap();
    lazy_static! {
        static ref REPLACE_REG: Regex = Regex::new("^-\\s+(.*)").unwrap();
    }

    return format!(
        "<ol start=\"{}\"><li>{}</li></ol>",
        list,
        serialize(node, &REPLACE_REG.replace_all(&node_string, "$1"))
    );
}
