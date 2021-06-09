mod serializer;
mod string_gen;
use mdview_lexer::structures::{NodeType, AST};
use string_gen::*;

pub fn generate_html_from_ast(tree: AST) -> String {
  let mut html_string = String::new();

  for node in tree.body {
    let node_type = node.node_type.clone();
    let string = match node.node_type {
      NodeType::Header => header(node),
      NodeType::NormalLine => normal_line(node),
      NodeType::NewLine => new_line(),
      NodeType::UList => unsorted_list(node),
      // NodeType::SList => sorted_list(node),
      _ => String::from("\n"),
    };
    println!("{:?}: {}", node_type, string);
    html_string.push_str(&string);
  }

  return html_string;
}
