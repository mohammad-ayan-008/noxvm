use std::{collections::VecDeque, mem::offset_of, usize, vec};

use crate::{
    chunk::{Chunk, OpCode},
    compiler::Compiler,
    value::Value,
};

pub struct VM<'a> {
    chunk: Option<&'a mut Chunk>,
    pub ip: usize,
    pub stack: VecDeque<Value>,
}
pub enum InterpretResult {
    INTERPRET_OK,
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR,
}

#[allow(non_camel_case_types, non_snake_case)]
impl<'a> VM<'a> {
    pub fn new() -> Self {
        Self {
            chunk: None,
            ip: 0,
            stack: VecDeque::with_capacity(256),
        }
    }

    pub fn interpret(&mut self, source: String, chunk: &'a mut Chunk) -> InterpretResult {
        if !Compiler::compile(source, chunk) {
            return InterpretResult::INTERPRET_COMPILE_ERROR;
        }
        self.chunk = Some(chunk);
        self.run().unwrap()
    }

    pub fn run(&mut self) -> Option<InterpretResult> {
        loop {
            print!("          ");
            println!(" stack {:?}", self.stack);
            self.chunk
                .as_deref_mut()
                .unwrap()
                .disassembleInstruction(self.ip);
            let instruction = self.read_byte();
            match OpCode::try_from(instruction).unwrap() {
                OpCode::Return => {
                    println!("{}", self.stack.pop_back().unwrap());
                    return Some(InterpretResult::INTERPRET_OK);
                }
                OpCode::Op_Constnats => {
                    let value = self.read_constant();
                    self.stack.push_back(value);
                    println!("{:>4}", value);
                }
                OpCode::OP_NEGATE => {
                    let value = -self.stack.pop_back().unwrap();
                    self.stack.push_back(value);
                }
                OpCode::OP_ADD => {
                    let value_1 = self.stack.pop_back().unwrap();
                    let value_2 = self.stack.pop_back().unwrap();
                    self.stack.push_back(value_1 + value_2);
                }
                OpCode::OP_DIVIDE => {
                    let value_1 = self.stack.pop_back().unwrap();
                    let value_2 = self.stack.pop_back().unwrap();
                    self.stack.push_back(value_1 / value_2);
                }
                OpCode::OP_SUBTRACT => {
                    let value_1 = self.stack.pop_back().unwrap();
                    let value_2 = self.stack.pop_back().unwrap();
                    self.stack.push_back(value_1 - value_2);
                }
                OpCode::OP_MULTIPLY => {
                    let value_1 = self.stack.pop_back().unwrap();
                    let value_2 = self.stack.pop_back().unwrap();
                    self.stack.push_back(value_1 * value_2);
                }
                _ => todo!(),
            }
        }
    }

    pub fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.as_deref_mut().unwrap().code[self.ip];
        self.ip += 1;
        byte
    }

    pub fn read_constant(&mut self) -> Value {
        let byte = self.read_byte();
        self.chunk.as_deref().unwrap().constants.values[byte as usize]
    }
}
