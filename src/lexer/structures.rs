#[derive(Clone)]
pub enum NodeType {
    Header(usize),
    NewLine,
    NormalLine,
    UnsortedList,
    SortedList(usize),
}

#[derive(Clone)]
pub struct TextAttributes {
    pub inline_code: bool,
    pub image_or_link: bool,
    pub strike: bool,
    pub bold_or_italics: bool,
}

#[derive(Clone)]
pub struct NodeInfo {
    pub string: Option<String>,
    pub attributes: Option<TextAttributes>,
}

#[derive(Clone)]
pub struct Node {
    pub r#type: NodeType,
    pub info: NodeInfo,
    pub include_next_line: bool,
    pub allow_merge: bool,
}

pub struct Ast {
    pub body: Vec<Node>,
}
