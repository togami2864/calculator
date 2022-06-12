use std::io::Result;

use calculator::{interpreter::Interpreter, lexer::Lexer, parser::Parser};

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
            let l = Lexer::new(line.as_str());
            let mut p = Parser::new(l);
            let ast = p.parse_expr().unwrap();
            println!("{:?}", &ast);
            let mut i = Interpreter::new();
            println!("{:?}", i.eval(ast).unwrap());
        }
    }
}
