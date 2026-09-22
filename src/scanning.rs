//! This is an EBNF grammar for the lexical component of the language.
//! letter  = 'a' | ... | 'z' | 'A' | ... | 'Z' ;
//! digit   = '0' | '1' | ... | '9' ;
//! ident   = ( letter | '_' ) { letter | digit | '_' } ;
//! intlit  = digit { digit } ;
//! keyword = 'if' | 'else' | 'return' | 'int' | 'null' ;

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

pub struct Lexer<'a> {
    src: &'a str,
    bytes: &'a [u8],
    pos: usize,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn lex(mut self) -> Vec<Token> {
        while !self.at_end() {
            let start = self.pos;
            let c = self.advance();
            let kind = match c {
                b' ' | b'\t' | b'\n' | b'\r' => continue,

                b'(' => TokenKind::LParen,
                b')' => TokenKind::RParen,
                b'{' => TokenKind::LBrace,
                b'}' => TokenKind::RBrace,
                b':' => TokenKind::Colon,
                b';' => TokenKind::Semi,
                b',' => TokenKind::Comma,
                b'.' => TokenKind::Dot,
                b'+' => TokenKind::Plus,
                b'-' => TokenKind::Minus,
                b'*' => TokenKind::Star,
                b'/' => TokenKind::Slash,
                b'&' => TokenKind::Amp,

                b'!' => {
                    if self.peek_equals(b'=') {
                        TokenKind::BangEq
                    } else {
                        TokenKind::Bang
                    }
                }
                b'=' => {
                    if self.peek_equals(b'=') {
                        TokenKind::EqEq
                    } else {
                        TokenKind::Eq
                    }
                }
                b'<' => {
                    if self.peek_equals(b'=') {
                        TokenKind::Le
                    } else {
                        TokenKind::Lt
                    }
                }
                b'>' => {
                    if self.peek_equals(b'=') {
                        TokenKind::Ge
                    } else {
                        TokenKind::Gt
                    }
                }

                b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.handle_ascii(start),
                b'0'..=b'9' => self.handle_numeric(start),

                _ => TokenKind::Error,
            };
            self.tokens.push(Token {
                kind,
                span: Span {
                    start: start as u32,
                    end: self.pos as u32,
                },
            });
        }
        self.tokens.push(Token {
            kind: TokenKind::Eof,
            span: Span {
                start: self.pos as u32,
                end: self.pos as u32,
            },
        });
        self.tokens
    }

    fn at_end(&self) -> bool {
        self.pos >= self.bytes.len()
    }

    fn advance(&mut self) -> u8 {
        let b = self.peek();
        self.pos += 1;
        b
    }

    fn handle_ascii(&mut self, start: usize) -> TokenKind {
        while self.peek().is_ascii_alphanumeric() || self.peek() == b'_' {
            self.pos += 1;
        }
        match &self.src[start..self.pos] {
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "return" => TokenKind::Return,
            "record" => TokenKind::Record,
            "int" => TokenKind::IntTy,
            s => TokenKind::Ident(s.to_string()),
        }
    }

    fn handle_numeric(&mut self, start: usize) -> TokenKind {
        while self.peek().is_ascii_digit() {
            self.pos += 1;
        }
        TokenKind::Int(self.src[start..self.pos].to_string())
    }

    fn peek(&self) -> u8 {
        self.bytes.get(self.pos).copied().unwrap_or(0)
    }

    fn peek_equals(&mut self, b: u8) -> bool {
        if self.peek() == b {
            self.pos += 1;
            true
        } else {
            false
        }
    }
}

pub enum TokenKind {
    Ident(String),
    Int(String),
    If,
    Else,
    Return,
    Record,
    IntTy,
    Null,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Colon,
    Semi,
    Comma,
    Dot,
    Eq,
    Plus,
    Minus,
    Star,
    Slash,
    Amp,
    Bang,
    EqEq,
    BangEq,
    Lt,
    Gt,
    Le,
    Ge,
    Eof,
    Error,
}

pub struct Span {
    pub start: u32,
    pub end: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_function() {
        let program = "main() : i64 {\n\
                                 return 0;\n\
                             }";
        println!("{:#?}", program);
    }
}
