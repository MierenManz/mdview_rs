mod node_gen;
pub mod structures;

use node_gen::*;
use regex::Regex;
use structures::*;

#[inline(always)]
pub fn generate_ast(markdown_string: String) -> AST {
  let mut ast = AST { body: Vec::new() };
  let chararray = markdown_string.chars();
  let array_length = markdown_string.len();
  let s_list_regex = Regex::new("/^[0-9]\\.\\s/g").unwrap();
  let mut line = String::new();
  let mut i = 0;
  let mut iter = 0;

  for character in chararray {
    iter += 1;

    if character != '\n' {
      line.push(character);
    } else if character == '\n' || iter == array_length {
      line = String::from(line.trim());
      if line.starts_with("#") {
        if i - 1 < ast.body.len() {
          ast.body[i - 1].include_next_line = false;
        }
        ast.body.push(header_node(&line));
      } else if line.starts_with("- ") || s_list_regex.is_match(&line) {
        if i - 1 < ast.body.len() {
          ast.body[i - 1].include_next_line = false;
        }

        ast.body.push(list_node(&line));
      } else if line != "" {
        ast.body.push(normal_line(&line));
      } else {
        ast.body.push(new_line());
      }

      line = String::new();
      i += 1;
    }
  }

  {
    let len = ast.body.len();
    ast.body[len - 1].include_next_line = false;
  }

  return ast;
}
