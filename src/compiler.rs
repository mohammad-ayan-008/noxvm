#![allow(warnings)]
use std::{cell::RefCell, default, panic, rc::Rc, slice::SliceIndex, usize};

use crate::{
    chunk::{Chunk, OpCode},
    scanner::{self, Scanner},
    token::{Kind, Token},
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
        LeftParen         => run(Some(Compiler::grouping as ParseFn), None, Presidence::PREC_NONE),
        RightParen        => run(None, None, Presidence::PREC_NONE),
        LeftBrace         => run(None, None, Presidence::PREC_NONE),
        RightBrace        => run(None, None, Presidence::PREC_NONE),
        Comma             => run(None, None, Presidence::PREC_NONE),
        Dot               => run(None, None, Presidence::PREC_NONE),
        Minus             => run(Some(Compiler::unary as ParseFn), Some(Compiler::binary as ParseFn), Presidence::PREC_TERM),
        Plus              => run(None, Some(Compiler::binary as ParseFn), Presidence::PREC_TERM),
        Slash             => run(None, Some(Compiler::binary as ParseFn), Presidence::PREC_FACTOR),
        Star              => run(None, Some(Compiler::binary as ParseFn), Presidence::PREC_FACTOR),
        Semicolon         => run(None, None, Presidence::PREC_NONE),

        Bang              => run(None, None, Presidence::PREC_NONE),
        BangEqual         => run(None, None, Presidence::PREC_NONE),
        Equal             => run(None, None, Presidence::PREC_NONE),
        EqualEqual        => run(None, None, Presidence::PREC_NONE),
        Greater           => run(None, None, Presidence::PREC_NONE),
        GreaterEqual      => run(None, None, Presidence::PREC_NONE),
        Less              => run(None, None, Presidence::PREC_NONE),
        LessEqual         => run(None, None, Presidence::PREC_NONE),

        IdentifierLiteral => run(None, None, Presidence::PREC_NONE),
        StringLiteral     => run(None, None, Presidence::PREC_NONE),
        NumberLiteral     => run(Some(Compiler::number as ParseFn), None, Presidence::PREC_NONE),

        And               => run(None, None, Presidence::PREC_NONE),
        Class             => run(None, None, Presidence::PREC_NONE),
        Else              => run(None, None, Presidence::PREC_NONE),
        False             => run(None, None, Presidence::PREC_NONE),
        For               => run(None, None, Presidence::PREC_NONE),
        Fun               => run(None, None, Presidence::PREC_NONE),
        If                => run(None, None, Presidence::PREC_NONE),
        Nil               => run(None, None, Presidence::PREC_NONE),
        Or                => run(None, None, Presidence::PREC_NONE),
        Print             => run(None, None, Presidence::PREC_NONE),
        Return            => run(None, None, Presidence::PREC_NONE),
        Super             => run(None, None, Presidence::PREC_NONE),
        This              => run(None, None, Presidence::PREC_NONE),
        True              => run(None, None, Presidence::PREC_NONE),
        Var               => run(None, None, Presidence::PREC_NONE),
        While             => run(None, None, Presidence::PREC_NONE),
        Error             => run(None, None, Presidence::PREC_NONE),
        Eof               => run(None, None, Presidence::PREC_NONE),    }
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

#[derive(Clone,PartialEq, PartialOrd)]
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


#[derive(Clone,Default)]
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

    pub fn new(source:String)-> Self{
        Self { scanner: Scanner::new(source), current: None,parser:Parser::default()}
    }
    pub fn compile(&mut self, chunk: Rc<RefCell<Chunk>>) -> bool {
        self.current = Some(chunk);
        self.parser.panic_mode = false;
        self.parser.has_error = false;
        self.advance();

        self.expression();

        self.consume(Kind::Eof, "Expected End of expression".to_owned());
        self.endCompiler();
        !self.parser.has_error
    }

    fn expression(&mut self) {
        self.parsePrecedence(Presidence::PREC_ASSIGNMENT);
    }

    fn number(&mut self) {
        let val = self.parser.previous.string.parse::<f64>().unwrap();
        self.emit_constant(val);
    }

    fn emit_constant(&mut self, value: f64) {
        let byte = self.make_constnat(value);
        self.emit_Bytes(OpCode::Op_Constnats as u8, byte);
    }

    fn make_constnat(&mut self, value: f64) -> u8 {
        let constnat = self.current
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
        self.parsePrecedence(Presidence::PREC_ASSIGNMENT);
        let operator_type = self.parser.previous.kind.clone();
        self.expression();

        match operator_type {
            Kind::Minus => self.emitByte(OpCode::OP_NEGATE as u8),
            _ => (),
        }
    }



    pub fn binary(&mut self){

        let token_kind = self.parser.previous.kind.clone();
        let rule = rule_for_token(&token_kind);
        self.parsePrecedence(rule.precedence.next().unwrap());

        use Kind::*;
        match token_kind {
            Plus => self.emitByte(OpCode::OP_ADD as u8),
            Minus => self.emitByte(OpCode::OP_NEGATE as u8),
            Star => self.emitByte(OpCode::OP_MULTIPLY as u8),
            Slash => self.emitByte(OpCode::OP_DIVIDE as u8),
        _ => panic!("Unreachable: unexpected operator in binary()"),
        }
    }

    fn parsePrecedence(&mut self, preseidence: Presidence) {
        self.advance();
        let fun = rule_for_token(&self.parser.previous.kind).prefix;
        if fun == None{
            eprint!("Expected Expreession");
            return;
        }
        fun.unwrap()(self);

        while preseidence <= rule_for_token(&self.parser.current.kind).precedence{
            self.advance();
            let infix = rule_for_token(&self.parser.previous.kind).infix.unwrap();
            infix(self)
        }
    }

    fn emit_Bytes(&mut self, byte_1: u8, byte_2: u8) {
        self.emitByte(byte_1);
        self.emitByte(byte_2);
    }

    fn endCompiler(&mut self) {
        self.emit_return();
        if !self.parser.has_error {
           self.current.as_ref().unwrap().borrow_mut().disassembleChunk("code");
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
        self.parser.panic_mode = true;
        eprintln!("[Line {}] Error", token.line);

        if self.parser.panic_mode {
            return;
        }
        if token.kind == Kind::Eof {
            eprintln!("at end");
        } else if token.kind == Kind::Error {
        } else {
            eprintln!("at {} {}", token.index_in_source, token.string);
        }
        self.parser.has_error = true;
    }
}
