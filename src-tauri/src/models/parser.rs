use crate::models::{expression, operation};

pub struct Parser {
    chars: Vec<char>,
    pos: usize,
}

impl Parser {
    fn new(s: &str) -> Self {
        Self {
            chars: s.chars().collect(),
            pos: 0,
        }
    }
    pub fn parse(s: &str) -> Result<expression::Expr, String> {
        if s.trim().is_empty() {
            return Err("expressão vazia".into());
        }
        let mut p = Self::new(s);
        let e = p.parse_or()?;
        p.spaces();
        if p.pos != p.chars.len() {
            return Err(p.error(&format!("símbolo inesperado '{}'", p.chars[p.pos])));
        }
        Ok(e)
    }
    fn spaces(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }
    fn take(&mut self, c: char) -> bool {
        self.spaces();
        if self.pos < self.chars.len() && self.chars[self.pos] == c {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    fn take_text(&mut self, text: &str) -> bool {
        self.spaces();
        let symbols: Vec<char> = text.chars().collect();
        if self.chars[self.pos..].starts_with(&symbols) {
            self.pos += symbols.len();
            true
        } else {
            false
        }
    }
    fn error(&self, s: &str) -> String {
        format!("{} na posição {}", s, self.pos)
    }
    fn parse_or(&mut self) -> Result<expression::Expr, String> {
        let mut r = self.parse_xor()?;
        while self.take('+') || self.take('∨') || self.take_text("||") || self.take('|') {
            r = expression::bin(r, operation::Op::Or, self.parse_xor()?);
        }
        Ok(r)
    }
    fn parse_xor(&mut self) -> Result<expression::Expr, String> {
        let mut r = self.parse_and()?;
        while self.take('^') || self.take('⊕') || self.take('≢') {
            r = expression::bin(r, operation::Op::Xor, self.parse_and()?);
        }
        Ok(r)
    }
    fn parse_and(&mut self) -> Result<expression::Expr, String> {
        let mut r = self.parse_unary()?;
        loop {
            if self.take('.') || self.take('*') || self.take('·') || self.take('∧') || self.take_text("&&") || self.take('&') || self.starts_unary() {
                r = expression::bin(r, operation::Op::And, self.parse_unary()?);
            } else {
                return Ok(r);
            }
        }
    }
    fn starts_unary(&mut self) -> bool {
        self.spaces();
        self.pos < self.chars.len()
            && (matches!(self.chars[self.pos], '(' | '~' | '!' | '0' | '1' | '_')
                || self.chars[self.pos].is_alphabetic())
    }
    fn parse_unary(&mut self) -> Result<expression::Expr, String> {
        if self.take('~') || self.take('!') || self.take('¬') {
            return Ok(expression::not(self.parse_unary()?));
        }
        if self.take('(') {
            let mut r = self.parse_or()?;
            if !self.take(')') {
                return Err(self.error("era esperado ')'"));
            }
            while self.take('\'') {
                r = expression::not(r);
            }
            return Ok(r);
        }
        self.spaces();
        if self.pos >= self.chars.len() {
            return Err(self.error("era esperado um operando"));
        }
        let c = self.chars[self.pos];
        let mut r = if c == '0' || c == '1' {
            self.pos += 1;
            expression::Expr::Const(c == '1')
        } else if c.is_alphabetic() || c == '_' {
            let start = self.pos;
            self.pos += 1;
            while self.pos < self.chars.len()
                && (self.chars[self.pos].is_alphanumeric() || self.chars[self.pos] == '_')
            {
                self.pos += 1;
            }
            expression::Expr::Var(self.chars[start..self.pos].iter().collect())
        } else {
            return Err(self.error("era esperado um operando"));
        };
        while self.take('\'') {
            r = expression::not(r);
        }
        Ok(r)
    }
}
