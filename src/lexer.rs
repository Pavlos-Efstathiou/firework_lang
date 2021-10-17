use std::borrow::Cow;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Token {
    Number(usize),
    Unknown(char),
    Whitespace,
    LeftParen,
    RightParen,
    Minus,
    Plus,
    Mul,
    Div,
}

#[derive(Debug)]
pub struct Lexer {
    pub source_code: String,
    pub current_char: char,
    pub position: usize,
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(code: &Cow<str>) -> Self {
        Self {
            source_code: Cow::to_string(code),
            current_char: Cow::to_string(code).chars().nth(0).unwrap(),
            position: 0,
            tokens: Vec::new(),
        }
    }

    pub fn read_next_token(&mut self) {
        if self.position < self.source_code.len() {
            self.current_char = self.source_code.chars().nth(self.position).unwrap();

            let current_token: Token = match self.current_char {
                curr if curr.is_digit(10) => Token::Number(0),
                curr if curr.is_whitespace() => Token::Whitespace,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Mul,
                '/' => Token::Div,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                unknown => Token::Unknown(unknown),
            };

            let final_token = self.handle_multichar_token(current_token);

            if current_token == Token::Whitespace {
            } else {
                self.tokens.push(final_token);
            }
        } else {
        }
    }

    fn handle_multichar_token(&mut self, kind: Token) -> Token {
        match kind {
            Token::Number(_) => {
                // Bad, bad code
                let mut number = String::from("");

                for ch in self.source_code.split_at(self.position).1.chars() {
                    if ch.is_digit(10) {
                        self.current_char = ch;
                        number.push(ch);
                        self.position += 1;
                    } else {
                        break;
                    }
                }
                Token::Number(number.parse::<usize>().unwrap())
            }
            token => {
                self.position += 1;
                token
            }
        }
    }
}
