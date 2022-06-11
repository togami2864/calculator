use std::io::Result;

use calculator::{lexer::Lexer, token::Token};

fn prompt(s: &str) -> Result<()> {
    use std::io::{stdout, Write};
    let stdout = stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(s.as_bytes()).unwrap();
    stdout.flush()
}

fn main() {
    use std::io::{stdin, BufRead, BufReader};
    let stdin = stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);
    let mut lines = stdin.lines();
    loop {
        prompt("> ").unwrap();
        if let Some(Ok(line)) = lines.next() {
            let mut l = Lexer::new(line.as_str());
            while let Ok(token) = l.nextToken() {
                println!("{:?}", token);
                if token == Token::Eof {
                    break;
                }
            }
        }
    }
}
