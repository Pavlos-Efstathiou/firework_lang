use std::{borrow::Cow, fs};
mod lexer;

fn main() {
    let source_code = fs::read_to_string("test.firework").unwrap();
    let mut lexer = lexer::Lexer::new(&Cow::Borrowed(&source_code));

    for _ in 0..(&source_code).len() {
        lexer.read_next_token()
    }
    println!("{:?}", &lexer.tokens)
}

#[test]
fn test() {
    use lexer::Token::*;
    let source_code = fs::read_to_string("test.firework").unwrap();
    let mut lexer = lexer::Lexer::new(&Cow::Borrowed(&source_code));

    for _ in 0..(&source_code).len() {
        lexer.read_next_token()
    }

    assert_eq!(
        lexer.tokens,
        [
            Number(6),
            Div,
            Number(2),
            Mul,
            LeftParen,
            Number(1),
            Plus,
            Number(2),
            RightParen
        ]
        .repeat(200)
    )
}
