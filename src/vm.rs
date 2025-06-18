use std::{cell::RefCell, collections::VecDeque, env::set_var, mem::offset_of, rc::Rc, usize, vec};

use crate::{
    chunk::{Chunk, OpCode},
    compiler::Compiler,
    value::{Value, ValueType},
};

pub struct VM {
    chunk: Option<Rc<RefCell<Chunk>>>,
    pub ip: usize,
    pub stack: VecDeque<Value>,
    compiler: Compiler,
}
pub enum InterpretResult {
    INTERPRET_OK,
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR,
}

#[allow(non_camel_case_types, non_snake_case)]
impl VM {
    pub fn new(source: String) -> Self {
        Self {
            chunk: None,
            ip: 0,
            stack: VecDeque::with_capacity(256),
            compiler: Compiler::new(source),
        }
    }

    pub fn interpret(&mut self, chunk: Rc<RefCell<Chunk>>) -> InterpretResult {
        if !self.compiler.compile(chunk.clone()) {
            return InterpretResult::INTERPRET_COMPILE_ERROR;
        }
        self.chunk = Some(chunk.clone());
        self.run().unwrap()
    }

    pub fn run(&mut self) -> Option<InterpretResult> {
        loop {
            //print!("          ");
           // println!(" stack {:?}", self.stack);
            self.chunk
                .as_ref()
                .unwrap()
                .borrow_mut()
                .disassembleInstruction(self.ip);
            let instruction = self.read_byte();
            match OpCode::try_from(instruction).unwrap() {
                OpCode::Return => {
                    let val = self.stack.pop_back().unwrap();
                    match val.type_v  {
                        ValueType::VAL_NUMBER(a)=>println!("{}",a),
                        ValueType::VAL_BOOL(a)=>println!("{}",a),
                        ValueType::VAL_NIL=>println!("nil")
                    }
                    return Some(InterpretResult::INTERPRET_OK);
                }
                OpCode::OP_NIL => self.stack.push_back(Value::nil_value()),
                OpCode::OP_FALSE => self
                    .stack
                    .push_back(Value::from(ValueType::VAL_BOOL(false))),
                OpCode::OP_TRUE => self.stack.push_back(Value::from(ValueType::VAL_BOOL(true))),
                OpCode::Op_Constnats => {
                    let value = self.read_constant();
                    if let ValueType::VAL_NUMBER(value) = value.type_v {
                        self.stack.push_back(Value::from(value));
                        println!("{:>4?}", value);
                    }
                }
                OpCode::OP_NEGATE => {
                    let value = self.stack.pop_back().unwrap();
                    if !self.peek(0).is_number() {
                        self.runtime_Error("Operand must be a number");
                        return Some(InterpretResult::INTERPRET_RUNTIME_ERROR);
                    }
                    self.stack.push_back(-value);
                }
                a @ OpCode::OP_ADD => self.binar_op(a),
                a @ OpCode::OP_DIVIDE => self.binar_op(a),
                a @ OpCode::OP_SUBTRACT => self.binar_op(a),
                a @ OpCode::OP_MULTIPLY => self.binar_op(a),
                OpCode::OP_NOT => {
                    let val = self.stack.pop_back().unwrap();
                    self.stack
                        .push_back(Value::from(ValueType::VAL_BOOL(Self::is_falsely(val))));
                }
                OpCode::OP_EQUAL => {
                    let val1 = self.stack.pop_back().unwrap();
                    let val2 = self.stack.pop_back().unwrap();
                    self.stack
                        .push_back(Value::from(ValueType::VAL_BOOL(Self::valueEqual(
                            val1, val2,
                        ))));
                }

                a @ OpCode::OP_GREATER => self.binar_op(a),
                a @ OpCode::OP_LESS => self.binar_op(a),
                _ => todo!(),
            }
        }
    }
    fn valueEqual(val1: Value, val2: Value) -> bool {
        if val1.type_v != val2.type_v {
            return false;
        }
        match val1.type_v {
            ValueType::VAL_BOOL(_) => val1.as_bool() == val2.as_bool(),
            ValueType::VAL_NIL => true,
            ValueType::VAL_NUMBER(_) => val1.as_number() == val2.as_number(),
        }
    }

    fn is_falsely(value: Value) -> bool {
        value.is_nil() || value.is_bool() && !value.as_bool()
    }
    pub fn runtime_Error(&mut self, msg: &str) {
        eprintln!("{}", msg);
        if self.ip > 0 {
            let ins = self.ip - 1;
            let chunk = self.chunk.as_ref().unwrap().clone();
            if ins < chunk.borrow_mut().lines.len() {
                let line = chunk.borrow_mut().lines[ins];
                eprintln!("Line[{}] in script", line);
            } else {
                eprintln!("Line[uknown] in script");
            }
        }
        self.stack.clear();
    }
    #[inline]
    fn binar_op(&mut self, bi_op: OpCode) {
        match bi_op {
            OpCode::OP_ADD => {
                let value_1 = self.stack.pop_back().unwrap();
                let value_2 = self.stack.pop_back().unwrap();
                self.stack.push_back(value_2 + value_1);
            }
            OpCode::OP_DIVIDE => {
                let value_1 = self.stack.pop_back().unwrap();
                let value_2 = self.stack.pop_back().unwrap();
                self.stack.push_back(value_2 / value_1);
            }
            OpCode::OP_SUBTRACT => {
                let value_1 = self.stack.pop_back().unwrap();
                let value_2 = self.stack.pop_back().unwrap();
                self.stack.push_back(value_2 - value_1);
            }
            OpCode::OP_MULTIPLY => {
                let value_1 = self.stack.pop_back().unwrap();
                let value_2 = self.stack.pop_back().unwrap();
                self.stack.push_back(value_2 * value_1);
            }
            OpCode::OP_GREATER => {
                let val1 = self.stack.pop_back().unwrap();
                let val2 = self.stack.pop_back().unwrap();
                if !val1.is_number() || !val2.is_number() {
                    self.runtime_Error("LHS != RHS");
                }
                self.stack.push_back(Value::from(ValueType::VAL_BOOL(val1.as_number() < val2.as_number())));
            }
            OpCode::OP_LESS => {
                let val1 = self.stack.pop_back().unwrap();
                let val2 = self.stack.pop_back().unwrap();
                if !val1.is_number() || !val2.is_number() {
                    self.runtime_Error("LHS != RHS");
                }
                self.stack.push_back(Value::from(ValueType::VAL_BOOL(val1.as_number() > val2.as_number())));

            }
            a => panic!("Unable to parse the Binary operation"),
        }
    }
    pub fn peek(&self, index: usize) -> Value {
        let len = self.stack.len();
        self.stack
            .get(len - 1 - index)
            .cloned()
            .unwrap_or_else(|| panic!("stack under flow"))
    }
    pub fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.as_ref().unwrap().borrow_mut().code[self.ip];
        self.ip += 1;
        byte
    }

    pub fn read_constant(&mut self) -> Value {
        let byte = self.read_byte();
        self.chunk.as_ref().unwrap().borrow_mut().constants.values[byte as usize].clone()
    }
}
