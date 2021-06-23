mod node_gen;
pub mod structures;

use lazy_static::lazy_static;
use node_gen::*;
use regex::Regex;
use structures::Ast;

pub fn generate_ast(mut markdown_string: String) -> Ast {
    let mut ast = Ast { body: Vec::new() };

    if !markdown_string.ends_with('\n') {
        markdown_string.push('\n');
    }

    lazy_static! {
        static ref SORTED_LIST_REG: Regex = Regex::new(r"^\d+\.\s+.*").unwrap();
    }

    for (iteration, line) in markdown_string.lines().enumerate() {
        let mut body_len = ast.body.len();

        let node = if line.starts_with('#') {
            if iteration > 0 && body_len > 0 {
                ast.body[body_len - 1].include_next_line = false;
            }
            header_node(&line)
        } else if line.starts_with("- ") || SORTED_LIST_REG.is_match(&line) {
            if iteration > 0 && iteration < body_len {
                ast.body[iteration - 1].include_next_line = false;
            }

            list_node(&line)
        } else if !line.is_empty() {
            normal_line(&line)
        } else {
            new_line()
        };

        ast.body.push(node);

        body_len += 1;

        let old_index = body_len - 2;
        let new_index = body_len - 1;

        if iteration > 2
            && iteration - 1 < body_len
            && ast.body[old_index].include_next_line
            && ast.body[new_index].allow_merge
        {
            let old_node = ast.body[old_index].clone();
            let current_node = ast.body[new_index].clone();

            let new_node = merge_nodes(old_node, current_node);
            ast.body.remove(old_index);
            ast.body[old_index] = new_node;
        }
    }

    let len = ast.body.len(); // Borrowing error
    ast.body[len - 1].include_next_line = false;

    ast
}
