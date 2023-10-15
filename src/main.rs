use std::io::BufRead;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        println!("Usage: toylang [script]");
        std::process::exit(1);
    } else if args.len() == 1 {
        run_file(args[0].as_str());
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) -> Result<(), std::io::Error> {
    let bytes = std::fs::read(path)?;
    let text = std::string::String::from_utf8(bytes).unwrap();
    run(text.as_str());
    Ok(())
}

fn run_prompt() -> Result<(), std::io::Error> {
    let mut reader = std::io::BufReader::new(std::io::stdin());

    loop {
        println!("> ");
        let mut line = String::new();
        reader.read_line(&mut line);
        if line.is_empty() {
            break;
        }

        run(&line);
    }

    Ok(())
}

fn run(source: &str) {
    let scanner = Scanner::new();
    let tokens: Vec<Tokens> = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", tokens);
    }
}
