mod Run;
mod scanner;
mod token;
mod token_type;

use Run::Lox;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut run = Lox::new();

    if args.len() > 1 {
        println!("Usage: toylang [script]");
        std::process::exit(1);
    } else if args.len() == 1 {
        run.run_file(&args[0].as_str());
    } else {
        run.run_prompt();
    }
}
