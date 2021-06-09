use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum NodeType {
    Header(usize),
    NewLine,
    NormalLine,
    UnsortedList,
    SortedList(usize),
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Node {
    pub r#type: NodeType,
    pub info: NodeInfo,
    pub include_next_line: bool,
    pub allow_merge: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ast {
    pub body: Vec<Node>,
}
