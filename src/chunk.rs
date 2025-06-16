use crate::{chunk, value::{self, Value, ValueArray}};

#[derive(Debug)]
#[repr(u8)]
pub enum OpCode{
    Return,
    OP_NEGATE,
    OP_ADD,
    OP_SUBTRACT,
    OP_MULTIPLY,
    OP_DIVIDE,
    Op_Constnats,
}

impl TryFrom<u8> for OpCode{
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let res = unsafe {
            std::mem::transmute::<u8,OpCode>(value)
        };
        Ok(res)
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub code:Vec<u8>,
    pub lines:Vec<usize>,
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

    pub fn write_chunk(&mut self,opcode:u8,line:usize){
        //println!("OP_code as u8{:?}" , opcode );
        self.code.push(opcode);
        self.lines.push(line);
    }

    pub fn addConstant(&mut self,value:Value)-> usize{
        self.constants.values.push(value);
        self.constants.values.len() -1
    }

    pub fn disassembleInstruction(&mut self,offset:usize)->usize{
       // println!("Code: {:?} Constants : {:?}",self.code,self.constants.values);
        print!("{:04} ",offset);
        if offset > 0 && self.lines[offset] == self.lines[offset -1]{
            print!("   | ")
        }else {
            print!("{:04} ",self.lines[offset]);
        }

        let instruction = self.code[offset];
        match OpCode::try_from(instruction).unwrap(){
            OpCode::Return => {
                println!("OP_RETURN");
                offset + 1
            },
            OpCode::Op_Constnats =>{
                let constant = self.code[offset +1];
                print!("{:<16} {:>4}","OP_CONSTANT",constant);
                println!("-> {}",self.constants.values[constant as usize]);
                println!("");
                offset +2
            },
            OpCode::OP_NEGATE =>{
                println!("OP_NEGATE");
                offset + 1 
            },
            OpCode::OP_MULTIPLY=>{
                println!("OP_MULTIPLY");
                offset + 1 
            },
            OpCode::OP_SUBTRACT=>{
                println!("OP_SUBTRACT");
                offset + 1 
            },
            OpCode::OP_DIVIDE=>{
                println!("OP_DIVIDE");
                offset + 1 
            },
            OpCode::OP_ADD=>{
                println!("OP_ADD");
                offset + 1
            }
            _=>todo!()
        }
    }
}
