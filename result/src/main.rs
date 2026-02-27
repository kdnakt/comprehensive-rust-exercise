use std::iter::Peekable;
use std::str::Chars;

use anyhow::Result;

/// 算術演算子。
#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Add,
    Sub,
}

/// 式言語のトークン
#[derive(Debug, PartialEq)]
enum Token {
    Number(String),
    Identifier(String),
    Operator(Op),
}

/// 式言語の式
#[derive(Debug, PartialEq)]
enum Expression {
    /// 変数への参照。
    Var(String),
    /// リテラル数値。
    Number(u32),
    /// バイナリ演算。
    Operation(Box<Expression>, Op, Box<Expression>),
}

fn tokenize(input: &str) -> Tokenizer {
    return Tokenizer(input.chars().peekable());
}

struct Tokenizer<'a>(Peekable<Chars<'a>>);

impl<'a> Tokenizer<'a> {
    fn collect_number(&mut self, first_char: char) -> Token {
        let mut num = String::from(first_char);
        while let Some(&c @ '0'..='9') = self.0.peek() {
            num.push(c);
            self.0.next();
        }
        Token::Number(num)
    }

    fn collect_identifier(&mut self, first_char: char) -> Token {
        let mut ident = String::from(first_char);
        while let Some(&c @ ('a'..='z' | '_' | '0'..='9')) = self.0.peek() {
            ident.push(c);
            self.0.next();
        }
        Token::Identifier(ident)
    }
}

#[derive(Debug, thiserror::Error)]
enum TokenizerError {
    #[error("Unexpected character {0}")]
    UnexpectedCharacter(char),
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, TokenizerError>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.0.next()?;
        Some(Ok(match c {
            '0'..='9' => self.collect_number(c),
            'a'..='z' => self.collect_identifier(c),
            '+' => Token::Operator(Op::Add),
            '-' => Token::Operator(Op::Sub),
            _ => return Some(Err(TokenizerError::UnexpectedCharacter(c))),
        }))
    }
}

#[derive(Debug, thiserror::Error)]
enum ParseError {
    #[error("Unexpected token {0:?}")]
    UnexpectedToken(Token),
    #[error("Unexpected end of input")]
    UnexpectedEOF,
    #[error("Tokenizer error: {0}")]
    TokenizerError(#[from] TokenizerError),
}

fn parse(input: &str) -> Result<Expression, ParseError> {
    let mut tokens = tokenize(input);

    fn parse_expr<'a>(tokens: &mut Tokenizer<'a>) -> Result<Expression, ParseError> {
        let tok = tokens.next().ok_or(ParseError::UnexpectedEOF)??;
        let expr = match tok {
            Token::Number(num) => {
                let v = num.parse().expect("Invalid 32-bit integer");
                Expression::Number(v)
            }
            Token::Identifier(ident) => Expression::Var(ident),
            Token::Operator(_) => return Err(ParseError::UnexpectedToken(tok)),
        };
        // バイナリ演算が存在する場合はパースします。
        match tokens.next() {
            None => Ok(expr),
            Some(Ok(Token::Operator(op))) => Ok(Expression::Operation(
                Box::new(expr),
                op,
                Box::new(parse_expr(tokens)?),
            )),
            Some(Err(e)) => Err(e.into()),
            Some(Ok(token)) => Err(ParseError::UnexpectedToken(token)),
        }
    }

    parse_expr(&mut tokens)
}

fn main() -> Result<()> {
    let expr = parse("10+foo+20-30")?;
    println!("{expr:?}");
    Ok(())
}
