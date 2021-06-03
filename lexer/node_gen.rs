use super::structures::*;
use regex::Regex;

#[inline(always)]
pub fn normal_line(line: &str) -> Node {
  let some_string = Some(String::from(line));
  Node {
    node_type: NodeType::NormalLine,
    node_info: NodeInfo {
      text: some_string.clone(),
      attributes: get_attributes(line),
      header: None,
      sorted_list_number: None,
    },
    original_string: some_string,
    include_next_line: !(line.ends_with("  ") || line.ends_with("\\")),
  }
}

#[inline(always)]
pub fn new_line() -> Node {
  Node {
    node_type: NodeType::NewLine,
    node_info: NodeInfo {
      text: None,
      attributes: None,
      header: None,
      sorted_list_number: None,
    },
    original_string: None,
    include_next_line: false,
  }
}

#[inline(always)]
pub fn list_node(line: &str) -> Node {
  let some_string = Some(String::from(line));
  let is_ulist = Regex::new("^-\\s+").unwrap().is_match(line);
  Node {
    node_type: if is_ulist {
      NodeType::UList
    } else {
      NodeType::SList
    },
    node_info: NodeInfo {
      text: some_string.clone(),
      attributes: get_attributes(line),
      header: None,
      sorted_list_number: if !is_ulist { line.find(".") } else { None },
    },
    original_string: some_string,
    include_next_line: !(line.ends_with("  ") || line.ends_with("\\")),
  }
}

#[inline(always)]
pub fn header_node(line: &str) -> Node {
  let some_copy = Some(String::from(line));
  Node {
    node_type: NodeType::Header,
    node_info: NodeInfo {
      text: some_copy.clone(),
      attributes: get_attributes(line),
      header: line.find(' '),
      sorted_list_number: None,
    },
    original_string: some_copy,
    include_next_line: false,
  }
}

#[inline(always)]
fn get_attributes(line: &str) -> Option<TextAttributes> {
  Some(TextAttributes {
    image_or_link: Regex::new("\\[(.*)\\]\\((.*)\\)").unwrap().is_match(line),
    strike: Regex::new("~~(.*)~~").unwrap().is_match(line),
    bold_or_italics: Regex::new("\\*(.*)\\*|_(.*)_").unwrap().is_match(line),
  })
}
