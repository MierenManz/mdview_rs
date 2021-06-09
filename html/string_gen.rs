use crate::serializer::serialize;
use mdview_lexer::structures::Node;
use regex::Regex;

#[inline(always)]
pub fn header(node: Node) -> String {
  let node_string = node.node_info.string.clone().unwrap();
  let header = node.node_info.header.unwrap();
  let string = node_string.split_at(header + 1).1;

  return format!("<h{0}>{1}</h{0}>", header, serialize(node, string));
}

#[inline(always)]
pub fn normal_line(node: Node) -> String {
  let node_string = node.node_info.string.clone().unwrap();
  return format!("<p>{}</p>", serialize(node, &node_string));
}

#[inline(always)]
pub fn new_line() -> String {
  return String::from("<br>");
}

#[inline(always)]
pub fn unsorted_list(node: Node) -> String {
  let node_string = node.node_info.string.clone().unwrap();
  let replace_reg = Regex::new("^-\\s+(.*)").unwrap();

  return format!(
    "<ul><li>{}</li></ul>",
    serialize(node, &replace_reg.replace_all(&node_string, "$1"))
  );
}

// #[inline(always)]
// pub fn sorted_list(node: Node) -> String {}
