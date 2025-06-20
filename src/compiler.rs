#![allow(warnings)]
use std::{cell::RefCell, default, env::set_var, panic, rc::Rc, slice::SliceIndex, usize};

use crate::{
    chunk::{Chunk, OpCode},
    scanner::{self, Scanner},
    token::{Kind, Token},
    value::{Value, ValueType},
};

type ParseFn = for<'a> fn(&'a mut Compiler);

#[derive(Clone)]
pub struct ParseRule {
    pub prefix: Option<ParseFn>,
    pub infix: Option<ParseFn>,
    pub precedence: Presidence,
}

fn rule_for_token(token: &Kind) -> ParseRule {
    use Kind::*;
    match token {
        LeftParen => run(
            Some(Compiler::grouping as ParseFn),
            None,
            Presidence::PREC_NONE,
        ),
        RightParen => run(None, None, Presidence::PREC_NONE),
        LeftBrace => run(None, None, Presidence::PREC_NONE),
        RightBrace => run(None, None, Presidence::PREC_NONE),
        Comma => run(None, None, Presidence::PREC_NONE),
        Dot => run(None, None, Presidence::PREC_NONE),
        Minus => run(
            Some(Compiler::unary as ParseFn),
            Some(Compiler::binary as ParseFn),
            Presidence::PREC_TERM,
        ),
        Plus => run(
            None,
            Some(Compiler::binary as ParseFn),
            Presidence::PREC_TERM,
        ),
        Slash => run(
            None,
            Some(Compiler::binary as ParseFn),
            Presidence::PREC_FACTOR,
        ),
        Star => run(
            None,
            Some(Compiler::binary as ParseFn),
            Presidence::PREC_FACTOR,
        ),
        Semicolon => run(None, None, Presidence::PREC_NONE),

        Bang => run(Some(Compiler::unary), None, Presidence::PREC_UNARY),
        BangEqual => run(None, Some(Compiler::binary), Presidence::PREC_EQUALITY),
        Equal => run(None, None, Presidence::PREC_NONE),
        EqualEqual => run(None, Some(Compiler::binary), Presidence::PREC_EQUALITY),
        Greater => run(None, Some(Compiler::binary), Presidence::PREC_COMPARISON),
        GreaterEqual => run(None, Some(Compiler::binary), Presidence::PREC_COMPARISON),
        Less => run(None, Some(Compiler::binary), Presidence::PREC_COMPARISON),
        LessEqual => run(None, Some(Compiler::binary), Presidence::PREC_COMPARISON),

        IdentifierLiteral => run(None, None, Presidence::PREC_NONE),
        StringLiteral => run(Some(Compiler::string), None, Presidence::PREC_NONE),
        NumberLiteral => run(Some(Compiler::number), None, Presidence::PREC_NONE),

        And => run(None, None, Presidence::PREC_NONE),
        Class => run(None, None, Presidence::PREC_NONE),
        Else => run(None, None, Presidence::PREC_NONE),
        False => run(Some(Compiler::literal), None, Presidence::PREC_NONE),
        For => run(None, None, Presidence::PREC_NONE),
        Fun => run(None, None, Presidence::PREC_NONE),
        If => run(None, None, Presidence::PREC_NONE),
        Nil => run(Some(Compiler::literal), None, Presidence::PREC_NONE),
        Or => run(None, None, Presidence::PREC_NONE),
        Print => run(None, None, Presidence::PREC_NONE),
        Return => run(None, None, Presidence::PREC_NONE),
        Super => run(None, None, Presidence::PREC_NONE),
        This => run(None, None, Presidence::PREC_NONE),
        True => run(Some(Compiler::literal), None, Presidence::PREC_NONE),
        Var => run(None, None, Presidence::PREC_NONE),
        While => run(None, None, Presidence::PREC_NONE),
        Error => run(None, None, Presidence::PREC_NONE),
        Eof => run(None, None, Presidence::PREC_NONE),
    }
}

fn run(prefix: Option<ParseFn>, infix: Option<ParseFn>, presidence: Presidence) -> ParseRule {
    ParseRule {
        prefix,
        infix,
        precedence: presidence,
    }
}

#[derive(Clone)]
pub enum Precedence {
    None,
    Term,   // + -
    Factor, // * /
    Unary,  // !, -
    Primary,
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
#[repr(usize)]
pub enum Presidence {
    PREC_NONE,
    PREC_ASSIGNMENT, // =
    PREC_OR,         // or
    PREC_AND,        // and
    PREC_EQUALITY,   // == !=
    PREC_COMPARISON, // < > <= >=
    PREC_TERM,       // + -
    PREC_FACTOR,     // * /
    PREC_UNARY,      // ! -
    PREC_CALL,       // . ()
    PREC_PRIMARY,
}
impl Presidence {
    pub fn next(&self) -> Option<Presidence> {
        use Presidence::*;

        match self {
            PREC_NONE => Some(PREC_ASSIGNMENT),
            PREC_ASSIGNMENT => Some(PREC_OR),
            PREC_OR => Some(PREC_AND),
            PREC_AND => Some(PREC_EQUALITY),
            PREC_EQUALITY => Some(PREC_COMPARISON),
            PREC_COMPARISON => Some(PREC_TERM),
            PREC_TERM => Some(PREC_FACTOR),
            PREC_FACTOR => Some(PREC_UNARY),
            PREC_UNARY => Some(PREC_CALL),
            PREC_CALL => Some(PREC_PRIMARY),
            PREC_PRIMARY => None, // No next after highest precedence
        }
    }
}

#[derive(Clone, Default)]
pub struct Parser {
    current: Token,
    previous: Token,
    has_error: bool,
    panic_mode: bool,
}

pub struct Compiler {
    scanner: Scanner,
    parser: Parser,
    current: Option<Rc<RefCell<Chunk>>>,
}

impl Compiler {
    pub fn new(source: String) -> Self {
        Self {
            scanner: Scanner::new(source),
            current: None,
            parser: Parser::default(),
        }
    }

    pub fn string(&mut self) {
        let str = self.parser.previous.string.clone();
        self.emit_constant(Value::from(ValueType::VAL_STRING(str)));
    }

    pub fn literal(&mut self) {
        match self.parser.previous.kind {
            Kind::True => self.emitByte(OpCode::OP_TRUE as u8),
            Kind::Nil => self.emitByte(OpCode::OP_NIL as u8),
            Kind::False => self.emitByte(OpCode::OP_FALSE as u8),
            _ => unreachable!(),
        }
    }
    pub fn compile(&mut self, chunk: Rc<RefCell<Chunk>>) -> bool {
        self.current = Some(chunk);
        self.parser.panic_mode = false;
        self.parser.has_error = false;
        self.advance();

        while !self.match_token(Kind::Eof) {
            self.declaration();
            if self.parser.panic_mode {
                break; // or synchronize if you want to recover
            }
        }

        self.consume(Kind::Eof, "Expected End of expression".to_owned());
        self.endCompiler();
        !self.parser.has_error
    }

    fn declaration(&mut self) {
        self.statement();
    }
    fn statement(&mut self) {
        if self.match_token(Kind::Print) {
            self.printStatement();
        }
    }

    fn printStatement(&mut self) {
        self.consume(Kind::LeftParen, "Expected ( before Expression ".to_string());
        self.expression();
        self.consume(Kind::RightParen, "Expected ( after Expression ".to_string());
        self.consume(Kind::Semicolon, "Expected ; after Expression ".to_string());
        self.emitByte(OpCode::OP_PRINT as u8);
    }

    fn expression(&mut self) {
        self.parsePrecedence(Presidence::PREC_ASSIGNMENT);
    }

    fn number(&mut self) {
        //println!("{}",self.parser.previous.string);
        let val = self.parser.previous.string.parse::<f64>().unwrap();
        self.emit_constant(Value::from(val));
    }

    fn emit_constant(&mut self, value: Value) {
        let byte = self.make_constnat(value);
        self.emit_Bytes(OpCode::Op_Constnats as u8, byte);
    }

    fn match_token(&mut self, token: Kind) -> bool {
        if !self.check(token) {
            return false;
        };
        self.advance();
        return true;
    }
    fn check(&self, token: Kind) -> bool {
        self.parser.current.kind == token
    }

    fn make_constnat(&mut self, value: Value) -> u8 {
        let constnat = self
            .current
            .as_ref()
            .unwrap()
            .borrow_mut()
            .addConstant(value) as u8;

        if constnat > u8::MAX {
            eprintln!("Too many constants");
            return 0;
        }
        return constnat;
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(Kind::RightParen, "Expected ) after expression".to_owned());
    }

    pub fn unary(&mut self) {
        let operator_type = self.parser.previous.kind.clone();
        self.parsePrecedence(Presidence::PREC_UNARY);
        match operator_type {
            Kind::Minus => self.emitByte(OpCode::OP_NEGATE as u8),
            Kind::Bang => self.emitByte(OpCode::OP_NOT as u8),
            _ => (),
        }
    }

    pub fn binary(&mut self) {
        let token_kind = self.parser.previous.kind.clone();
        let rule = rule_for_token(&token_kind);
        self.parsePrecedence(rule.precedence.next().unwrap());

        use Kind::*;
        match token_kind {
            Plus => self.emitByte(OpCode::OP_ADD as u8),
            Minus => self.emitByte(OpCode::OP_SUBTRACT as u8),
            Bang => self.emitByte(OpCode::OP_NOT as u8),
            Star => self.emitByte(OpCode::OP_MULTIPLY as u8),
            Slash => self.emitByte(OpCode::OP_DIVIDE as u8),
            EqualEqual => self.emitByte(OpCode::OP_EQUAL as u8),
            BangEqual => self.emit_Bytes(OpCode::OP_EQUAL as u8, OpCode::OP_NOT as u8),
            Greater => self.emitByte(OpCode::OP_GREATER as u8),
            Less => self.emitByte(OpCode::OP_LESS as u8),
            LessEqual => self.emit_Bytes(OpCode::OP_LESS as u8, OpCode::OP_NOT as u8),
            GreaterEqual => self.emit_Bytes(OpCode::OP_GREATER as u8, OpCode::OP_NOT as u8),
            LessEqual => self.emit_Bytes(OpCode::OP_LESS as u8, OpCode::OP_NOT as u8),
            a => panic!("Unreachable: unexpected operator in binary({:?})", a),
        }
    }

    fn parsePrecedence(&mut self, precedence: Presidence) {
        self.advance();

        let prefix = rule_for_token(&self.parser.previous.kind).prefix;
        if prefix == None {
            println!("Expected expression {:?}", self.parser.previous.kind);
            return;
        }
        prefix.unwrap()(self);

        while precedence <= rule_for_token(&self.parser.current.kind).precedence {
            // println!("While loop: {:?} <= {:?}", precedence, rule_for_token(&self.parser.current.kind).precedence);
            self.advance();
            let infix = rule_for_token(&self.parser.previous.kind).infix.unwrap();
            infix(self);
        }
    }

    fn emit_Bytes(&mut self, byte_1: u8, byte_2: u8) {
        self.emitByte(byte_1);
        self.emitByte(byte_2);
    }

    fn endCompiler(&mut self) {
        self.emit_return();
        if !self.parser.has_error {
            self.current
                .as_ref()
                .unwrap()
                .borrow_mut()
                .disassembleChunk("code");
        }
    }
    fn emit_return(&mut self) {
        self.emitByte(OpCode::Return as u8);
    }
    fn emitByte(&mut self, byte: u8) {
        self.current
            .as_ref()
            .unwrap()
            .borrow_mut()
            .write_chunk(byte, self.parser.previous.line);
    }
    pub fn consume(&mut self, token: Kind, message: String) {
        if self.parser.current.kind == token {
            self.advance();
            return;
        }

        self.errorAtCurrent(&message);
    }

    pub fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();
        loop {
            self.parser.current = self.scanner.next();
            if self.parser.current.kind != Kind::Error {
                break;
            }
            let data = &self.parser.current.string.clone();
            self.errorAtCurrent(data);
        }
    }

    fn errorAtCurrent(&mut self, data: &String) {
        self.errorAt(data);
    }

    fn errorAt(&mut self, data: &String) {
        let token = &self.parser.current;

        // Don't bail out before printing!
        eprintln!("[Line {}] Error", token.line);

        if token.kind == Kind::Eof {
            eprintln!(" at end: {}", data);
        } else if token.kind == Kind::Error {
            eprintln!(" error: {}", data);
        } else {
            eprintln!(" at '{}': {}", token.string, data);
        }

        self.parser.panic_mode = true;
        self.parser.has_error = true;
    }
}
