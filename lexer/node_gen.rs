use super::structures::*;
use regex::Regex;

#[inline(always)]
pub fn normal_line(line: &str) -> Node {
  Node {
    node_type: NodeType::NormalLine,
    node_info: NodeInfo {
      string: Some(String::from(line)),
      attributes: get_attributes(line),
      header: None,
      sorted_list_number: None,
    },
    include_next_line: !(line.ends_with("  ") || line.ends_with("\\")),
    allow_merge: true,
  }
}

#[inline(always)]
pub fn new_line() -> Node {
  Node {
    node_type: NodeType::NewLine,
    node_info: NodeInfo {
      string: None,
      attributes: None,
      header: None,
      sorted_list_number: None,
    },
    include_next_line: false,
    allow_merge: false,
  }
}

#[inline(always)]
pub fn list_node(line: &str) -> Node {
  let is_ulist = Regex::new("^-\\s+").unwrap().is_match(line);
  Node {
    node_type: if is_ulist {
      NodeType::UList
    } else {
      NodeType::SList
    },
    node_info: NodeInfo {
      string: Some(String::from(line)),
      attributes: get_attributes(line),
      header: None,
      sorted_list_number: if !is_ulist { line.find(".") } else { None },
    },
    include_next_line: !(line.ends_with("  ") || line.ends_with("\\")),
    allow_merge: false,
  }
}

#[inline(always)]
pub fn header_node(line: &str) -> Node {
  Node {
    node_type: NodeType::Header,
    node_info: NodeInfo {
      string: Some(String::from(line)),
      attributes: get_attributes(line),
      header: line.find(' '),
      sorted_list_number: None,
    },
    include_next_line: false,
    allow_merge: false,
  }
}

pub fn merge_nodes(last_node: Node, current_node: Node) -> Node {
  let string = push_new_str(
    last_node.node_info.string.unwrap(),
    current_node.node_info.string.unwrap(),
  );

  Node {
    node_type: last_node.node_type,
    node_info: NodeInfo {
      string: Some(string.clone()),
      attributes: get_attributes(&string),
      header: last_node.node_info.header,
      sorted_list_number: last_node.node_info.sorted_list_number,
    },
    include_next_line: current_node.include_next_line,
    allow_merge: last_node.allow_merge,
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

#[inline(always)]
fn push_new_str(first_string: String, string: String) -> String {
  let mut copy = String::from(first_string);
  copy.push_str(&string);

  copy
}
