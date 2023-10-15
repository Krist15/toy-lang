use std::io::BufRead;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn run_file(&self, path: &str) -> Result<(), std::io::Error> {
        let bytes = std::fs::read(path)?;
        let text = std::string::String::from_utf8(bytes).unwrap();
        self.run(text.as_str());

        if self.had_error {
            std::process::exit(1);
        }
        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<(), std::io::Error> {
        let mut reader = std::io::BufReader::new(std::io::stdin());

        loop {
            println!("> ");
            let mut line = String::new();
            reader.read_line(&mut line);
            if line.is_empty() {
                break;
            }

            self.run(&line);
            self.had_error = false;
        }

        Ok(())
    }

    fn run(&self, source: &str) {
        let scanner = Scanner::new();
        let tokens: Vec<Tokens> = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", tokens);
        }
    }

    fn error(&self, line: u32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: u32, wh: &str, message: &str) {
        eprintln!("[line {line}] Error {wh}: {message}");
        self.had_error = true;
    }
}
