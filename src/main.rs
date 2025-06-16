use chunk::{Chunk, OpCode};
use vm::VM;

mod chunk;
mod compiler;
mod scanner;
mod token;
mod value;
mod vm;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        run_file(&args[1]);
    } else {
        eprintln!("Usage : nlox [path]");
    }
}

fn run_file(path: &String) {
    let source = std::fs::read_to_string(path).expect("Failed to read the file");
    let mut scanner = scanner::Scanner::new(source);
    loop {
        let next = scanner.next();
        if next.kind == token::Kind::Eof {
            break;
        } else {
            println!("{:?}", next);
        }
    }
}
