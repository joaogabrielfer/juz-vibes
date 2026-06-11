use pyx_bytecode::{Constant, EncodeError, Instruction, Program};

pub fn compile_source(source: &str) -> Result<Vec<u8>, CompileError> {
    let value = parse_minimal_main(source)?;
    let mut program = Program::new();

    if let Ok(value) = u8::try_from(value) {
        program.push(Instruction::PushInt8(value));
    } else {
        let index = program.add_constant(Constant::Integer(value))?;
        program.push(Instruction::PushConst(index));
    }

    program.push(Instruction::Halt);
    Ok(program.encode()?)
}

fn parse_minimal_main(source: &str) -> Result<i64, CompileError> {
    let tokens = lex(source)?;
    let mut parser = Parser::new(tokens);
    parser.expect_keyword("ini")?;
    parser.expect_ident("main")?;
    parser.expect_symbol(":=")?;
    parser.expect_symbol("{")?;
    let value = parser.expect_integer()?;
    parser.expect_symbol("}")?;
    parser.expect_eof()?;
    Ok(value)
}

fn lex(source: &str) -> Result<Vec<Token>, CompileError> {
    let mut tokens = Vec::new();
    let mut chars = source.char_indices().peekable();

    while let Some((start, ch)) = chars.next() {
        match ch {
            ch if ch.is_whitespace() => {}
            '/' if chars.peek().is_some_and(|(_, next)| *next == '/') => {
                for (_, next) in chars.by_ref() {
                    if next == '\n' {
                        break;
                    }
                }
            }
            '/' if chars.peek().is_some_and(|(_, next)| *next == '*') => {
                chars.next();
                let mut prev = '\0';
                let mut closed = false;
                for (_, next) in chars.by_ref() {
                    if prev == '*' && next == '/' {
                        closed = true;
                        break;
                    }
                    prev = next;
                }
                if !closed {
                    return Err(CompileError::new(start, "unterminated block comment"));
                }
            }
            ':' if chars.peek().is_some_and(|(_, next)| *next == '=') => {
                chars.next();
                tokens.push(Token::symbol(":=", start));
            }
            '{' | '}' => tokens.push(Token::symbol(ch.to_string(), start)),
            '0'..='9' => {
                let mut literal = ch.to_string();
                while let Some((_, next)) = chars.peek() {
                    if next.is_ascii_digit() {
                        literal.push(*next);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let value = literal
                    .parse::<i64>()
                    .map_err(|_| CompileError::new(start, "integer literal is too large"))?;
                tokens.push(Token {
                    kind: TokenKind::Integer(value),
                    start,
                });
            }
            ch if is_ident_start(ch) => {
                let mut ident = ch.to_string();
                while let Some((_, next)) = chars.peek() {
                    if is_ident_continue(*next) {
                        ident.push(*next);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token {
                    kind: TokenKind::Ident(ident),
                    start,
                });
            }
            _ => {
                return Err(CompileError::new(
                    start,
                    format!("unsupported character `{ch}` in minimal compiler"),
                ));
            }
        }
    }

    tokens.push(Token {
        kind: TokenKind::Eof,
        start: source.len(),
    });
    Ok(tokens)
}

fn is_ident_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_ident_continue(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphanumeric()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    kind: TokenKind,
    start: usize,
}

impl Token {
    fn symbol(value: impl Into<String>, start: usize) -> Self {
        Self {
            kind: TokenKind::Symbol(value.into()),
            start,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenKind {
    Ident(String),
    Integer(i64),
    Symbol(String),
    Eof,
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expect_keyword(&mut self, keyword: &str) -> Result<(), CompileError> {
        self.expect_ident(keyword)
    }

    fn expect_ident(&mut self, expected: &str) -> Result<(), CompileError> {
        let token = self.advance();
        match &token.kind {
            TokenKind::Ident(actual) if actual == expected => Ok(()),
            _ => Err(CompileError::new(
                token.start,
                format!("expected `{expected}` in minimal `ini main` program"),
            )),
        }
    }

    fn expect_symbol(&mut self, expected: &str) -> Result<(), CompileError> {
        let token = self.advance();
        match &token.kind {
            TokenKind::Symbol(actual) if actual == expected => Ok(()),
            _ => Err(CompileError::new(
                token.start,
                format!("expected `{expected}` in minimal `ini main` program"),
            )),
        }
    }

    fn expect_integer(&mut self) -> Result<i64, CompileError> {
        let token = self.advance();
        match token.kind {
            TokenKind::Integer(value) => Ok(value),
            _ => Err(CompileError::new(
                token.start,
                "expected integer literal in minimal `ini main` body",
            )),
        }
    }

    fn expect_eof(&mut self) -> Result<(), CompileError> {
        let token = self.advance();
        match token.kind {
            TokenKind::Eof => Ok(()),
            _ => Err(CompileError::new(
                token.start,
                "unexpected tokens after minimal `ini main` program",
            )),
        }
    }

    fn advance(&mut self) -> Token {
        let token = self
            .tokens
            .get(self.current)
            .cloned()
            .unwrap_or_else(|| self.tokens.last().expect("parser has EOF token").clone());
        self.current += 1;
        token
    }
}

#[derive(Debug)]
pub struct CompileError {
    offset: usize,
    message: String,
}

impl CompileError {
    fn new(offset: usize, message: impl Into<String>) -> Self {
        Self {
            offset,
            message: message.into(),
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<EncodeError> for CompileError {
    fn from(value: EncodeError) -> Self {
        Self::new(0, value.to_string())
    }
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at byte {}", self.message, self.offset)
    }
}

impl std::error::Error for CompileError {}

#[cfg(test)]
mod tests {
    use super::compile_source;

    #[test]
    fn compiles_minimal_main_with_small_integer() {
        let bytes = compile_source("ini main := { 42 }").expect("source should compile");

        assert!(bytes.ends_with(&[0x02, 0x03, 0x00, 0x00, 0x00, 0x01, 42, 0x63, 0xff]));
    }

    #[test]
    fn compiles_large_integer_through_constant_pool() {
        let bytes = compile_source("ini main := { 300 }").expect("source should compile");

        assert!(
            bytes
                .windows(8)
                .any(|window| window == 300_i64.to_le_bytes())
        );
        assert!(bytes.ends_with(&[0x02, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x63, 0xff]));
    }

    #[test]
    fn accepts_comments() {
        let source = "/* leading */ ini main := { 1 // trailing\n }";

        compile_source(source).expect("comments should lex");
    }
}
