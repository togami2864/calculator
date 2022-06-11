use std::io::Result;

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
            let v: Vec<_> = line.chars().collect();
            dbg!(v);
        }
    }
}
