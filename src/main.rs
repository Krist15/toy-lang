mod run;
mod scanner;
mod token;
mod token_type;

#[macro_use]
extern crate lazy_static;

use run::Lox;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut run = Lox::new();

    if args.len() > 1 {
        println!("Usage: toylang [script]");
        std::process::exit(1);
    }
    if args.len() == 2 {
        run.run_file(&args[0].as_str()).unwrap();
        println!("Hello");
    } else {
        run.run_prompt().unwrap();
    }
}
