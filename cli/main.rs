use clap::App;
use clap::Arg;
use mdview_html::generate_html_from_ast;
use mdview_lexer::generate_ast;
use mdview_lexer::structures::AST;
use serde_json::to_string_pretty;
use std::fs;

pub fn main() {
  let args = get_cli_args();
  let data = fs::read_to_string(args.input).unwrap();
  let start = std::time::Instant::now();
  let ast = generate_ast(data);
  let stop = start.elapsed().as_millis();
  println!("{}", stop);

  let json_data = to_string_pretty(&AST {
    body: ast.body.clone(),
  })
  .unwrap();
  fs::write("out.json", json_data).unwrap();

  let htmlcode = generate_html_from_ast(ast);
  // println!("{:?}", htmlcode);

  fs::write("out.html", htmlcode).unwrap();
}

fn get_cli_args() -> Arguments {
  let cli_args = App::new("Markdown To HTML")
    .version("v0.1.0")
    .arg(
      Arg::with_name("Input File")
        .help("The input path for a file you want to convert.")
        .short("I")
        .long("input")
        .required(true)
        .takes_value(true),
    )
    .arg(
      Arg::with_name("Emit Output")
        .help("Emit the html to stdout or to a specified file.")
        .short("E")
        .long("emit")
        .takes_value(true),
    )
    .get_matches();

  Arguments {
    emit: match cli_args.value_of("Emit Output") {
      Some(e) => Some(e.to_string()),
      _ => None,
    },
    input: cli_args.value_of("Input File").unwrap().to_string(),
  }
}

pub struct Arguments {
  pub emit: Option<String>,
  pub input: String,
}
