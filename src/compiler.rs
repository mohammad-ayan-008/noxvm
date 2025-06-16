use crate::{
    chunk::{Chunk, OpCode},
    scanner::{self, Scanner},
    token::{Kind, Token},
};



type ParseFn = fn(&mut Compiler);

#[derive(Clone)]
pub struct ParseRule {
    pub prefix: Option<ParseFn>,
    pub infix: Option<ParseFn>,
    pub precedence: Precedence,
}



#[derive(Clone)]
pub enum Precedence{
    None,
    Term,     // + -
    Factor,   // * /
    Unary,    // !, -
    Primary,
}

#[derive(Clone)]
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



#[derive(Clone)]
pub struct Parser {
    current: Token,
    previous: Token,
    has_error: bool,
    panic_mode: bool,
}

pub struct Compiler<'a> {
    scanner: Scanner,
    parser: Parser,
    current: Option<&'a mut Chunk>,
}

impl<'a> Compiler<'a> {
    pub fn compile(&mut self, chunk: &'a mut Chunk) -> bool {
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
        let constnat = self.current.as_deref_mut().unwrap().addConstant(value) as u8;
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

    fn parsePrecedence(&mut self,preseidence:Presidence){

    }

    fn emit_Bytes(&mut self, byte_1: u8, byte_2: u8) {
        self.emitByte(byte_1);
        self.emitByte(byte_2);
    }

    fn endCompiler(&mut self) {
        self.emit_return()
    }
    fn emit_return(&mut self) {
        self.emitByte(OpCode::Return as u8);
    }
    fn emitByte(&mut self, byte: u8) {
        self.current
            .as_deref_mut()
            .unwrap()
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
