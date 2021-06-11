use clap::App;
use clap::Arg;
use mdview_html::generate_html_from_ast;
use mdview_lexer::generate_ast;
use mdview_window::create_window;
use std::fs;

pub fn main() {
    let args = get_cli_args();
    let data = fs::read_to_string(&args.input).unwrap();
    let start = std::time::Instant::now();
    let ast = generate_ast(data);
    let stop = start.elapsed().as_millis();
    println!("{}", stop);

    let htmlcode = generate_html_from_ast(ast);
    create_window(&htmlcode, &args.input, true);
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
        .get_matches();

    Arguments {
        input: cli_args.value_of("Input File").unwrap().to_string(),
    }
}

pub struct Arguments {
    pub input: String,
}
