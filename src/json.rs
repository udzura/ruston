use std::{collections::HashMap, fmt::Display, hash::Hash};

#[derive(Debug, Clone)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub lexeme: &'static [u8],
    pub ty: TokenType,
}

impl Token {
    fn is(&self, ty: TokenType) -> bool {
        self.ty == ty
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType {
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    Colon,
    Comma,
    Number,
    StrLit,
    Null,
    True,
    False,

    EoF,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Null,
    True,
    False,
    Int(i32),
    Str(&'static str),
    Array(Vec<Value>),
    Object(usize, HashMap<Value, Value>),
}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Object(val, _) => val.hash(state),
            _ => core::mem::discriminant(self).hash(state),
        };
    }
}

#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
}

impl JsonError {
    pub fn raise(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl std::error::Error for JsonError {}

pub mod lexer {
    use super::*;

    #[derive(Debug, Default)]
    pub struct Lex {
        pub stream: &'static [u8],
        pub start: usize,
        pub current: usize,
        pub line: usize,

        pub tokens: Vec<Token>,
    }

    impl Lex {
        pub fn run(stream: String) -> Result<Self, JsonError> {
            let stream: &'static [u8] = stream.as_bytes().to_owned().leak();
            let mut lex = Self {
                stream,
                ..Default::default()
            };

            lex.process()?;
            Ok(lex)
        }

        fn process(&mut self) -> Result<(), JsonError> {
            loop {
                match self.advance()? {
                    '{' => self.push_token(TokenType::BraceOpen),
                    '}' => self.push_token(TokenType::BraceClose),
                    '[' => self.push_token(TokenType::BracketOpen),
                    ']' => self.push_token(TokenType::BracketClose),
                    ':' => self.push_token(TokenType::Colon),
                    ',' => self.push_token(TokenType::Comma),
                    't' => {
                        self.matches("rue")?;
                        self.push_token(TokenType::True)
                    }
                    'f' => {
                        self.matches("alse")?;
                        self.push_token(TokenType::False)
                    }
                    'n' => {
                        self.matches("ull")?;
                        self.push_token(TokenType::Null)
                    }
                    '"' => loop {
                        let c = self
                            .advance()
                            .or_else(|_| Err(JsonError::raise("quote unmet")))?;
                        if c == '"' {
                            self.push_token(TokenType::StrLit);
                            break;
                        }
                    },
                    '\n' => {
                        self.line += 1;
                        self.start = self.current
                    }
                    ' ' | '\t' => self.start = self.current,
                    '0' => {
                        // '0' is just 0, not consider 0.1 ... or 0100 ...
                        // for now!
                        self.push_token(TokenType::Number);
                    }
                    '1'..='9' => {
                        while is_digit(self.peek()) {
                            self.advance()?;
                        }
                        self.push_token(TokenType::Number);
                    }
                    c => return Err(JsonError::raise(format!("unexpected token {}", c))),
                }
                if self.reached_end() {
                    self.push_token(TokenType::EoF);
                    break;
                }
            }
            Ok(())
        }

        fn peek(&mut self) -> Option<char> {
            self.stream.get(self.current).map(|c| (*c as char))
        }

        fn matches(&mut self, s: &str) -> Result<(), JsonError> {
            for c in s.chars() {
                let c2 = self.advance()?;
                if c2 != c {
                    return Err(JsonError::raise(format!("unexpected character {}", c2)));
                }
            }

            Ok(())
        }

        fn advance(&mut self) -> Result<char, JsonError> {
            let c = self.peek().ok_or_else(|| JsonError::raise("EoF"))?;
            // eprintln!("process: {}", c);
            self.current += 1;
            Ok(c)
        }

        fn reached_end(&mut self) -> bool {
            self.current >= self.stream.len()
        }

        fn push_token(&mut self, ty: TokenType) {
            let token = Token {
                start: self.start,
                end: self.current,
                lexeme: (&self.stream[self.start..self.current]),
                ty,
            };

            // eprintln!("pusing: {:?}: {}", &token.ty, &token.lexeme);
            self.tokens.push(token);
            self.start = self.current
        }
    }

    fn is_digit(c: Option<char>) -> bool {
        match c {
            Some(c) => c >= '0' && c <= '9',
            None => false,
        }
    }
}

pub mod parser {
    use super::*;

    #[derive(Debug)]
    pub struct Parser {
        pub stream: Vec<Token>,
        pub current: usize,
        pub value: Value,

        next_object_id: usize,
    }

    impl Parser {
        pub fn run(stream: Vec<Token>) -> Result<Self, JsonError> {
            let mut parser = Self {
                stream,
                current: 0,
                value: Value::Null,
                next_object_id: (1 << 16),
            };

            parser.process()?;
            Ok(parser)
        }

        fn process(&mut self) -> Result<(), JsonError> {
            self.value = self.process_value()?;

            if !self.reached_end()? {
                return Err(JsonError::raise("JSON has continued form"));
            }

            Ok(())
        }

        fn process_value(&mut self) -> Result<Value, JsonError> {
            use TokenType::*;
            let token = self.succ(1)?;
            let value = match &token.ty {
                Null => Value::Null,
                True => Value::True,
                False => Value::False,
                Number => Value::Int(String::from_utf8_lossy(token.lexeme).parse().unwrap()),
                StrLit => {
                    let len = token.lexeme.len();
                    let strval: Box<str> =
                        String::from_utf8_lossy(&token.lexeme[1..(len - 1)]).into();
                    Value::Str(&strval)
                }
                BracketOpen => self.array()?,
                BraceOpen => self.object()?,

                t => {
                    todo!("Unexpected token: {:?}", &t)
                }
            };
            Ok(value)
        }

        fn array(&mut self) -> Result<Value, JsonError> {
            use TokenType::*;
            let mut ar = Vec::new();
            loop {
                match self.peek(0) {
                    Some(t) => match &t.ty {
                        EoF => return Err(JsonError::raise("EoF")),
                        BracketClose => {
                            self.succ(1)?;
                            break;
                        }
                        _ => {
                            let v = self.process_value()?;
                            ar.push(v);
                            match self.peek(0) {
                                None => return Err(JsonError::raise("EoF")),
                                Some(t) => {
                                    if t.is(Comma) {
                                        self.succ(1)?;
                                        let n = self.peek(0);
                                        if n.is_none() {
                                            return Err(JsonError::raise("EoF"));
                                        }
                                        if n.unwrap().is(BracketClose) {
                                            return Err(JsonError::raise("Unexpected ]"));
                                        }
                                    }
                                }
                            }
                        }
                    },
                    None => return Err(JsonError::raise("EoF")),
                }
            }
            Ok(Value::Array(ar))
        }

        fn object(&mut self) -> Result<Value, JsonError> {
            use TokenType::*;
            let mut ha = HashMap::new();
            loop {
                match self.peek(0) {
                    Some(t) => match &t.ty {
                        EoF => return Err(JsonError::raise("EoF")),
                        BraceClose => {
                            self.succ(1)?;
                            break;
                        }
                        _ => {
                            let k = self.process_value()?;
                            match self.peek(0) {
                                None => return Err(JsonError::raise("EoF")),
                                Some(t) => {
                                    if t.is(Colon) {
                                        self.succ(1)?;
                                    } else {
                                        return Err(JsonError::raise(format!(
                                            "Unexpected token {:?}",
                                            t.ty
                                        )));
                                    }
                                }
                            }
                            let v = self.process_value()?;
                            ha.insert(k, v);
                            match self.peek(0) {
                                None => return Err(JsonError::raise("EoF")),
                                Some(t) => match t.ty {
                                    BraceClose => {
                                        continue;
                                    }
                                    Comma => {
                                        self.succ(1)?;
                                        continue;
                                    }
                                    other => {
                                        return Err(JsonError::raise(format!(
                                            "Unexpected token {:?}",
                                            other
                                        )));
                                    }
                                },
                            }
                        }
                    },
                    None => return Err(JsonError::raise("EoF")),
                }
            }
            self.next_object_id += 1;
            Ok(Value::Object(self.next_object_id, ha))
        }

        fn peek(&mut self, count: usize) -> Option<&Token> {
            self.stream.get(self.current + count)
        }

        fn previous(&mut self) -> Option<&Token> {
            if self.current == 0 {
                return None;
            }
            self.stream.get(self.current - 1)
        }

        fn succ(&mut self, count: usize) -> Result<&Token, JsonError> {
            self.current += count;
            self.previous().ok_or_else(|| JsonError::raise("EoF"))
        }

        fn reached_end(&mut self) -> Result<bool, JsonError> {
            self.peek(0)
                .map(|tok| tok.is(TokenType::EoF))
                .ok_or_else(|| JsonError::raise("EoF"))
        }
    }
}

pub fn parse(json: impl Into<String>) -> Value {
    let tokens = self::lexer::Lex::run(json.into()).unwrap().tokens;
    let value = self::parser::Parser::run(tokens).unwrap().value;
    value
}
