use std::{
    cell::RefCell,
    env::args,
    io::{Stdout, Write, stdin, stdout},
    process::Stdio,
    rc::Rc,
};

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
    } else if args.len() == 1 {
        repl();
    } else {
        eprintln!("Usage : nlox [path]");
    }
}

fn run_file(path: &String) {
    let source = std::fs::read_to_string(path).expect("Failed to read the file");
    let mut vm = VM::new(source);
    let chunk = Rc::new(RefCell::new(Chunk::new()));
    vm.interpret(chunk);
}

fn repl() {
    let mut data = String::new();

    loop {
        print!(">>");
        stdout().flush().unwrap();

        data.clear(); // ✅ move before read_line
        if stdin().read_line(&mut data).is_err() {
            println!("Error reading input");
            break;
        }

        if data.trim().is_empty() {
            println!("No input");
            continue;
        }

        let input = data.trim_end().to_string(); // ✅ trim newline
        let chunk = Rc::new(RefCell::new(Chunk::new()));
        let mut vm = VM::new(input);
        vm.interpret(chunk.clone());
    }
}
