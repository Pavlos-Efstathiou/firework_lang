use rustyline::error::ReadlineError;
use rustyline::Editor;
mod lexer;

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("🎆 >> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if !line.is_empty() {
                    let mut lexer = lexer::Lexer::new(&line);
                    lexer.lex();
                    println!("{:?}", lexer.tokens);
                } else {
                    print!("");
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

#[cfg(test)]
mod tests {
    use firework_lang::lexer;
    use firework_lang::lexer::Token::*;
    use std::fs;

    #[test]
    fn read_from_file() {
        let source_code = fs::read_to_string("firework_tests/test.firework").unwrap();
        let mut lexer = lexer::Lexer::new(&source_code);

        lexer.lex();

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

    #[test]
    fn whitespace() {
        let whitespace = "33 + 6 - (1 * 4)";
        let no_whitespace: &str = &whitespace
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        let mut lexer_whitespace = lexer::Lexer::new(whitespace);
        let mut lexer_no_whitespace = lexer::Lexer::new(no_whitespace);

        lexer_whitespace.lex();
        lexer_no_whitespace.lex();

        assert_eq!(lexer_whitespace.tokens, lexer_no_whitespace.tokens);
    }
}
