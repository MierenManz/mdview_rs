mod node_gen;
pub mod structures;

use node_gen::*;
use regex::Regex;
use structures::*;

pub fn generate_ast(mut markdown_string: String) -> AST {
  let mut ast = AST { body: Vec::new() };

  markdown_string = markdown_string.replace("\r\n", "\n");

  let chararray = markdown_string.chars();
  let array_length = markdown_string.len();
  let mut line = String::new();
  let mut i = 0;
  let mut iter = 0;

  for character in chararray {
    let start = std::time::Instant::now();
    iter += 1;

    if character != '\n' {
      line.push(character);
    } else if character == '\n' || iter == array_length {
      line = String::from(line.trim());
      if line.starts_with("#") {
        ast.body.push(header_node(&line));
      } else if line.starts_with("- ")
        || line.starts_with("* ")
        || Regex::new("/^[0-9]\\.\\s/g").unwrap().is_match(&line)
      {
        let buff = list_node(&line);

        if i > 0 && i < ast.body.len() {
          ast.body[i - 1].include_next_line = false;
          ast.body.push(new_line());
          ast.body.push(buff);
        }
      } else if line != "" {
        ast.body.push(normal_line(&line));
      }

      if !ast.body[ast.body.len() - 1].include_next_line
        || ast.body[ast.body.len() - 1].original_string != Some(String::new())
      {
        ast.body.push(new_line());
      }

      line = String::new();
      i += 1;
      let node = String::from(ast.body[ast.body.len() - 1].node_type);
      if node != "NewLine" {
        println!(
          "time: {}, node: {:?}",
          start.elapsed().as_micros(),
          ast.body[ast.body.len() - 1].node_type
        );
      }
    }
  }

  return ast;
}
