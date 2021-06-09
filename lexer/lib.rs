mod node_gen;
pub mod structures;

use node_gen::*;
use regex::Regex;
use structures::*;

#[inline(always)]
pub fn generate_ast(mut markdown_string: String) -> AST {
  let mut ast = AST { body: Vec::new() };
  if !markdown_string.ends_with('\n') {
    markdown_string.push('\n');
  }
  let chararray = markdown_string.chars();
  let array_length = markdown_string.len();
  let s_list_regex = Regex::new("/^[0-9]\\.\\s/g").unwrap();
  let mut line = String::new();
  let mut i = 0;

  for character in chararray {
    if character != '\n' {
      line.push(character);
    } else if character == '\n' || i + 1 == array_length {
      line = String::from(line.trim());
      if line.starts_with("#") {
        if i > 0 && i < ast.body.len() {
          ast.body[i - 1].include_next_line = false;
        }
        ast.body.push(header_node(&line));
      } else if line.starts_with("- ") || s_list_regex.is_match(&line) {
        if i > 0 && i < ast.body.len() {
          ast.body[i - 1].include_next_line = false;
        }

        ast.body.push(list_node(&line));
      } else if line != "" {
        ast.body.push(normal_line(&line));
      } else {
        ast.body.push(new_line());
      }
      {
        if i > 2
          && i - 1 < ast.body.len()
          && ast.body[ast.body.len() - 2].include_next_line
          && ast.body[ast.body.len() - 1].allow_merge
        {
          let len = ast.body.len();
          let old_index = len - 2;
          let old_node = ast.body[old_index].clone();
          let current_node = ast.body[len - 1].clone();

          let new_node = merge_nodes(old_node, current_node);
          ast.body.remove(old_index);
          ast.body[old_index] = new_node;
        }
      }

      line = String::new();
      i += 1;
    }
  }

  let len = ast.body.len();
  ast.body[len - 1].include_next_line = false;

  return ast;
}
