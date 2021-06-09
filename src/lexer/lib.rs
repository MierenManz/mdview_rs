mod node_gen;
pub mod structures;

use node_gen::*;
use regex::Regex;
use structures::Ast;

pub fn generate_ast(mut markdown_string: String) -> Ast {
    let mut ast = Ast { body: Vec::new() };
    if !markdown_string.ends_with('\n') {
        markdown_string.push('\n');
    }

    let s_list_regex = Regex::new("^\\d+\\.\\s+.*").unwrap();

    for (iteration, line) in markdown_string.lines().enumerate() {
        let node = if line.starts_with('#') {
            if iteration > 0 && iteration < ast.body.len() {
                ast.body[iteration - 1].include_next_line = false;
            }
            header_node(&line)
        } else if line.starts_with("- ") || s_list_regex.is_match(&line) {
            if iteration > 0 && iteration < ast.body.len() {
                ast.body[iteration - 1].include_next_line = false;
            }

            list_node(&line)
        } else if !line.is_empty() {
            normal_line(&line)
        } else {
            new_line()
        };

        ast.body.push(node);

        let ast_len = ast.body.len();
        if iteration > 2
            && iteration - 1 < ast_len
            && ast.body[ast_len - 2].include_next_line
            && ast.body[ast_len - 1].allow_merge
        {
            let old_index = ast_len - 2;
            let old_node = ast.body[old_index].clone();
            let current_node = ast.body[ast_len - 1].clone();

            let new_node = merge_nodes(old_node, current_node);
            ast.body.remove(old_index);
            ast.body[old_index] = new_node;
        }
    }

    let len = ast.body.len(); // Borrowing error
    ast.body[len - 1].include_next_line = false;

    ast
}
