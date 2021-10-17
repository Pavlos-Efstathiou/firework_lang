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
    pub fn new(code: &str) -> Self {
        Self {
            source_code: code.to_string(),
            current_char: code.chars().next().unwrap(),
            position: 0,
            tokens: Vec::new(),
        }
    }

    pub fn lex(&mut self) {
        for _ in 0..(self.source_code.len()) {
            self.next_token();
        }
    }

    #[allow(dead_code)]
    pub fn next_char(&self) -> char {
        return self.source_code.chars().nth(self.position).unwrap();
    }

    pub fn next_token(&mut self) {
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
            Token::Number(_) => Token::Number(
                self.source_code
                    .to_owned()
                    .split_at(self.position)
                    .1
                    .chars()
                    .take_while(|c| c.is_digit(10))
                    .map(|c| {
                        // If I don't include a map here the output is incorrect
                        self.position += 1;
                        c
                    })
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap(),
            ),
            token => {
                self.position += 1;
                token
            }
        }
    }
}
