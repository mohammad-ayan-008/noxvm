use crate::token::{Kind, Token};
use std::collections::VecDeque;
pub struct Scanner {
    characters: VecDeque<char>,
    line: usize,
    index: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            characters: source.chars().collect(),
            line: 1,
            index: 0,
        }
    }

    pub fn next(&mut self) -> Token {
        self.consume_whitespaces();

        match self.peek(0) {
            Some('{') => self.make_token(Kind::LeftBrace, 1),
            Some('}') => self.make_token(Kind::RightBrace, 1),
            Some('(') => self.make_token(Kind::LeftParen, 1),
            Some(')') => self.make_token(Kind::RightParen, 1),
            Some(',') => self.make_token(Kind::Comma, 1),
            Some('.') => self.make_token(Kind::Dot, 1),
            Some('-') => self.make_token(Kind::Minus, 1),
            Some('+') => self.make_token(Kind::Plus, 1),
            Some('/') => self.make_token(Kind::Slash, 1),
            Some('*') => self.make_token(Kind::Star, 1),
            Some(';') => self.make_token(Kind::Semicolon, 1),
            Some('!') if self.peek(1) == Some(&'=') => self.make_token(Kind::BangEqual, 2),
            Some('!') => self.make_token(Kind::Bang, 1),
            Some('=') if self.peek(1) == Some(&'=') => self.make_token(Kind::EqualEqual, 2),
            Some('=') => self.make_token(Kind::Equal, 1),
            Some('>') if self.peek(1) == Some(&'=') => self.make_token(Kind::GreaterEqual, 2),
            Some('>') => self.make_token(Kind::Greater, 1),
            Some('<') if self.peek(1) == Some(&'=') => self.make_token(Kind::LessEqual, 2),
            Some('<') => self.make_token(Kind::Less, 1),

            Some('a'..='z') | Some('A'..='Z') | Some('_') => self.indentifier_literal(),
            Some('0'..='9') => self.number_literal(),
            Some('"') => self.string_literal(),
            Some(_) => self.make_token(Kind::Error, 1),
            _ => self.make_token(Kind::Eof, 0),
        }
    }

    fn string_literal(&mut self) -> Token {
        let mut len = 1;
        while self.peek(len) != Some(&'"') && len <= self.characters.len() {
            len += 1;
        }

        if len >= self.characters.len() {
            self.make_token(Kind::Error, len)
        } else {
            self.make_token(Kind::StringLiteral, len + 1)
        }
    }
    fn number_literal(&mut self) -> Token {
        let mut len = 1;
        while let Some(ch) = self.peek(len) {
            if ch.is_ascii_digit() {
                len += 1;
            } else {
                break;
            }
        }

        if self.peek(len) == Some(&'.') && self.peek(len + 1).unwrap().is_ascii_digit() {
            len += 2;
            while let Some(ch) = self.peek(len) {
                if ch.is_ascii_digit() {
                    len += 1;
                }
            }
        }

        self.make_token(Kind::NumberLiteral, len)
    }

    fn indentifier_literal(&mut self) -> Token {
        let mut length = 1;
        while let Some(ch) = self.peek(length) {
            if ch.is_ascii_digit() || ch.is_alphanumeric() || ch == &'_' {
                length += 1;
            } else {
                break;
            }
        }

        let kind = match self
            .peek(0)
            .expect("Expected a character at the beginning of an identifier")
        {
            'a' if length == 3 && self.starts_with("and") => Kind::And,
            'c' if length == 5 && self.starts_with("class") => Kind::Class,
            'e' if length == 4 && self.starts_with("else") => Kind::Else,

            'f' if length == 5 && self.starts_with("false") => Kind::False,
            'f' if length == 3 && self.starts_with("for") => Kind::For,
            'f' if length == 3 && self.starts_with("fun") => Kind::Fun,

            'i' if length == 2 && self.starts_with("if") => Kind::If,
            'n' if length == 3 && self.starts_with("nil") => Kind::Nil,
            'o' if length == 2 && self.starts_with("or") => Kind::Or,
            'p' if length == 5 && self.starts_with("print") => Kind::Print,
            'r' if length == 6 && self.starts_with("return") => Kind::Return,
            's' if length == 5 && self.starts_with("super") => Kind::Super,

            't' if length == 4 && self.starts_with("this") => Kind::This,
            't' if length == 4 && self.starts_with("true") => Kind::True,

            'v' if length == 3 && self.starts_with("var") => Kind::Var,
            'w' if length == 5 && self.starts_with("while") => Kind::While,

            _ => Kind::IdentifierLiteral,
        };

        self.make_token(kind, length)
    }

    fn starts_with(&self, prefix: &str) -> bool {
        for (i, ch) in prefix.char_indices() {
            if self.peek(i) != Some(&ch) {
                return false;
            }
        }
        return true;
    }

    fn make_token(&mut self, kind: Kind, count: usize) -> Token {
        Token {
            kind,
            line: self.line,
            index_in_source: self.index,
            string: self.read_front(count),
        }
    }

    fn read_front(&mut self, count: usize) -> String {
        let mut string = String::new();
        for _ in 0..count {
            if let Some(ch) = self.advance() {
                string.push(ch);
            }
        }
        string
    }
    pub fn advance(&mut self) -> Option<char> {
        self.index += 1;
        self.characters.pop_front()
    }
    pub fn peek(&self, count: usize) -> Option<&char> {
        self.characters.get(count)
    }

    pub fn consume_whitespaces(&mut self) {
        loop {
            match self.peek(0) {
                Some(c) => match c {
                    ' ' | '\t' | '\r' => {
                        self.advance();
                        continue;
                    }
                    '\n' => {
                        self.advance();
                        self.line += 1;
                        continue;
                    }
                    '/' if self.peek(1) == Some(&'/') => {
                        while self.advance() != Some('\n') {}
                        continue;
                    }
                    _ => return,
                },
                None => return,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner;
    use crate::token::Kind;

    #[test]
    fn number_literals() {
        single_token_test(String::from("123"), Kind::NumberLiteral);
        single_token_test(String::from("123.1"), Kind::NumberLiteral);
        single_token_test(String::from("123.456"), Kind::NumberLiteral);
        single_token_test(String::from("0.456"), Kind::NumberLiteral);
        single_token_test(String::from("0.0"), Kind::NumberLiteral);
    }

    #[test]
    fn string_literals() {
        single_token_test(String::from("\"\""), Kind::StringLiteral);
        single_token_test(String::from("\"a string literal\""), Kind::StringLiteral);
    }

    #[test]
    fn identifier_literals() {
        single_token_test(String::from("x"), Kind::IdentifierLiteral);
        single_token_test(String::from("While"), Kind::IdentifierLiteral);
        single_token_test(String::from("_"), Kind::IdentifierLiteral);
        single_token_test(String::from("_1"), Kind::IdentifierLiteral);
        single_token_test(String::from("_abc123"), Kind::IdentifierLiteral);
    }

    #[test]
    fn keywords() {
        single_token_test(String::from("and"), Kind::And);
        single_token_test(String::from("or"), Kind::Or);
        single_token_test(String::from("class"), Kind::Class);
        single_token_test(String::from("fun"), Kind::Fun);
        single_token_test(String::from("var"), Kind::Var);
        single_token_test(String::from("if"), Kind::If);
        single_token_test(String::from("else"), Kind::Else);
        single_token_test(String::from("while"), Kind::While);
        single_token_test(String::from("for"), Kind::For);
        single_token_test(String::from("true"), Kind::True);
        single_token_test(String::from("false"), Kind::False);
        single_token_test(String::from("nil"), Kind::Nil);
        single_token_test(String::from("print"), Kind::Print);
        single_token_test(String::from("return"), Kind::Return);
        single_token_test(String::from("super"), Kind::Super);
        single_token_test(String::from("this"), Kind::This);
    }

    #[test]
    fn other_tokens() {
        single_token_test(String::from("{"), Kind::LeftBrace);
        single_token_test(String::from("}"), Kind::RightBrace);
        single_token_test(String::from("("), Kind::LeftParen);
        single_token_test(String::from(")"), Kind::RightParen);
        single_token_test(String::from(","), Kind::Comma);
        single_token_test(String::from("."), Kind::Dot);
        single_token_test(String::from("-"), Kind::Minus);
        single_token_test(String::from("+"), Kind::Plus);
        single_token_test(String::from("*"), Kind::Star);
        single_token_test(String::from("/"), Kind::Slash);
        single_token_test(String::from(";"), Kind::Semicolon);
        single_token_test(String::from("!"), Kind::Bang);
        single_token_test(String::from("!="), Kind::BangEqual);
        single_token_test(String::from("="), Kind::Equal);
        single_token_test(String::from("=="), Kind::EqualEqual);
        single_token_test(String::from(">"), Kind::Greater);
        single_token_test(String::from(">="), Kind::GreaterEqual);
        single_token_test(String::from("<"), Kind::Less);
        single_token_test(String::from("<="), Kind::LessEqual);
    }

    #[test]
    fn whitespace_and_comments() {
        let source = "
            // This is a comment
            while (true) // another comment
                print \"hey   \"
        ";

        let mut scanner = scanner::Scanner::new(String::from(source));
        assert_eq!(scanner.next().kind, Kind::While);
        assert_eq!(scanner.next().kind, Kind::LeftParen);
        assert_eq!(scanner.next().kind, Kind::True);
        assert_eq!(scanner.next().kind, Kind::RightParen);
        assert_eq!(scanner.next().kind, Kind::Print);
        assert_eq!(scanner.next().kind, Kind::StringLiteral);
    }

    #[test]
    fn empty_file() {
        let mut scanner = scanner::Scanner::new(String::new());
        assert_eq!(scanner.next().kind, Kind::Eof);
    }

    fn single_token_test(source: String, expected_kind: Kind) {
        let mut scanner = scanner::Scanner::new(source.clone());
        let token = scanner.next();

        assert_eq!(token.kind, expected_kind);
        assert_eq!(token.string, source);
        assert_eq!(scanner.next().kind, Kind::Eof, "Expected Eof.");
    }
}
