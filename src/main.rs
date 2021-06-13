use clap::App;
use clap::Arg;
use mdview_html::generate_html_from_ast;
use mdview_lexer::generate_ast;
use mdview_window::create_window;
use std::fs;

pub fn main() {
    let input = get_input_file();
    let data = fs::read_to_string(&input).unwrap();

    let ast = generate_ast(data);
    let htmlcode = generate_html_from_ast(ast);

    create_window(&htmlcode, &input, false);
}

fn get_input_file() -> String {
    let cli_args = App::new("MDview")
        .version("v0.1.0")
        .arg(
            Arg::with_name("Input File")
                .help("The input path for a file you want to convert.")
                .short("I")
                .long("input")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    String::from(cli_args.value_of("Input File").unwrap())
}
