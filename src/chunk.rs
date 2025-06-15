

use crate::{chunk, value::{self, Value, ValueArray}};

#[derive(Debug)]
#[repr(u8)]
pub enum OpCode{
    Return,
    Op_Constnats,
}

#[derive(Debug)]
pub struct Chunk {
    pub code:Vec<u8>,
    pub lines:Vec<i32>,
    pub constants:ValueArray
}

impl Chunk{


    pub fn new()->Self{
        Self { code: vec![], lines: vec![], constants: ValueArray::new() }
    }

    pub fn disassembleChunk(&mut self,name:&str){
        println!("== {} ==",name);
        let mut offset = 0;
        while offset < self.code.len(){

            offset = self.disassembleInstruction(offset)
        }
        
    }

    pub fn write_chunk(&mut self,opcode:u8,line:i32){
        self.code.push(opcode as u8);
        self.lines.push(line);
    }

    pub fn addConstant(&mut self,value:Value)-> usize{
        self.constants.values.push(value);
        self.constants.values.len() -1
    }

    fn disassembleInstruction(&mut self,offset:usize)->usize{
        print!("{:04} ",offset);
        if offset > 0 && self.lines[offset] == self.lines[offset -1]{
            print!("   | ")
        }else {
            print!("{:04} ",self.lines[offset]);
        }

        let instruction = self.code[offset];
        match unsafe {
            core::mem::transmute::<u8,OpCode>(instruction)
        }{
            OpCode::Return => {
                println!("OP_RETURN");
                offset + 1
            },
            OpCode::Op_Constnats =>{
                let constant = self.code[offset +1];
                print!("{:<16} {:>4}","OP_CONSTANT",constant);
                println!("{}",self.constants.values[constant as usize]);
                println!("");
                offset +2
            }
            _=>todo!()
        }
    }
}
