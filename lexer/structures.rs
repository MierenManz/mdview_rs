use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum NodeType {
  Header,
  NewLine,
  NormalLine,
  UList,
  SList,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextAttributes {
  pub image_or_link: bool,
  pub strike: bool,
  pub bold_or_italics: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeInfo {
  pub text: Option<String>,
  pub attributes: Option<TextAttributes>,
  pub header: Option<usize>,
  pub sorted_list_number: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
  pub node_type: NodeType,
  pub node_info: NodeInfo,
  pub original_string: Option<String>,
  pub include_next_line: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AST {
  pub body: Vec<Node>,
}
