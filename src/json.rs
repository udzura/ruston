use std::{collections::HashMap, fmt::Display, hash::Hash};

#[derive(Debug, Clone)]
pub struct Token {
    pub start: usize,
    pub current: usize,
    pub lexeme: String,
    pub ty: TokenType,
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
    String,
    Null,
    True,
    False,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Null,
    True,
    False,
    Int(i32),
    Str(String),
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
        pub stream: String,
        pub start: usize,
        pub current: usize,
        pub line: usize,

        pub tokens: Vec<Token>,
    }

    impl Lex {
        pub fn run(stream: String) -> Result<Self, JsonError> {
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
                            self.push_token(TokenType::String);
                            break;
                        }
                    },
                    '\n' => {
                        self.line += 1;
                        self.start = self.current
                    }
                    ' ' | '\t' => self.start = self.current,
                    '1'..='9' => {
                        while is_digit(self.peek()?) {
                            self.advance()?;
                        }
                        self.push_token(TokenType::Number);
                    }
                    c => return Err(JsonError::raise(format!("unexpected token {}", c))),
                }
                if self.reached_end() {
                    break;
                }
            }
            Ok(())
        }

        fn peek(&mut self) -> Result<char, JsonError> {
            self.stream
                .chars()
                .collect::<Vec<char>>()
                .get(self.current)
                .map(|c| *c)
                .ok_or_else(|| JsonError::raise("EoF"))
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

        // fn match_one(&mut self, c: char) -> Result<bool, JsonError> {
        //     Ok(true)
        // }

        fn advance(&mut self) -> Result<char, JsonError> {
            let c = self.peek()?;
            // eprintln!("process: {}", c);
            self.current += 1;
            if self.is_end_of_stream() {
                return Err(JsonError::raise("Unexpected EoS"));
            }
            Ok(c)
        }

        fn is_end_of_stream(&mut self) -> bool {
            self.current >= self.stream.len()
        }

        fn reached_end(&mut self) -> bool {
            self.current == (self.stream.len() - 1)
        }

        fn push_token(&mut self, ty: TokenType) {
            let token = Token {
                start: self.start,
                current: self.current,
                lexeme: (&self.stream[self.start..self.current]).to_string(),
                ty,
            };

            // eprintln!("pusing: {:?}: {}", &token.ty, &token.lexeme);
            self.tokens.push(token);
            self.start = self.current
        }
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }
}
