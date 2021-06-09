mod serializer;
mod string_gen;
use mdview_lexer::structures::Ast;
use mdview_lexer::structures::NodeType;
use string_gen::*;

// Just food for thought, this crate could take a std::io::Writer trait object.
// That way the generated HTML can be streamed, using less memory.
// Of course this design choice is heavily use-case dependent.

pub fn generate_html_from_ast(tree: Ast) -> String {
    let mut html_string = String::new();

    for node in tree.body {
        let string = match node.r#type {
            NodeType::Header(value) => header(node, value),
            NodeType::NormalLine => normal_line(node),
            NodeType::NewLine => new_line(),
            NodeType::UnsortedList => unsorted_list(node),
            NodeType::SortedList(_) => sorted_list(node),
        };
        html_string.push_str(&string);
    }

    html_string
}
