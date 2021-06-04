use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum NodeType {
  Header,
  NewLine,
  NormalLine,
  UList,
  SList,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TextAttributes {
  pub image_or_link: bool,
  pub strike: bool,
  pub bold_or_italics: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NodeInfo {
  pub string: Option<String>,
  pub attributes: Option<TextAttributes>,
  pub header: Option<usize>,
  pub sorted_list_number: Option<usize>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Node {
  pub node_type: NodeType,
  pub node_info: NodeInfo,
  pub include_next_line: bool,
  pub allow_merge: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AST {
  pub body: Vec<Node>,
}
