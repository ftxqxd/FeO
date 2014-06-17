use std::str::CharRange;

#[allow(non_camel_case_types)]
#[deriving(PartialEq, Eq, Show)]
pub enum Token {
    LitNum(String, String),
    LitStr(String),
    LitStrRaw(String),
    LitByteStr(Vec<u8>),
    LitByteStrRaw(Vec<u8>),
    LitChar(char),
    LitBool(bool),
    Ident(String),
    LParen,
    RParen,
    LSqbr,
    RSqbr,
    LBrace,
    RBrace,
    Eq,
    Lt,
    Le,
    EqEq,
    Gt,
    Ge,
    AndAnd,
    OrOr,
    XorXor,
    Not,
    Tilde,
    BinOp(BinOp),
    BinOpEq(BinOp),
    At,
    Dot,
    DotDot,
    DotDotDot,
    Comma,
    Semicolon,
    Colon,
    T_PAAMAYIM_NEKUDOTAYIM,
    LArrow,
    RArrow,
    FatArrow,
    Octothorpe,
    Dollar,
    Eof,
}

#[deriving(PartialEq, Eq, Show)]
pub enum BinOp {
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    Xor,
    And,
    Or,
    ShiftLeft,
    ShiftRight,
}

pub struct Tokens<'a> {
    str: &'a str,
    pos: uint,
}

impl<'a> Tokens<'a> {
    pub fn from_str(str: &'a str) -> Tokens<'a> {
        Tokens {
            str: str,
            pos: 0,
        }
    }

    fn char_at(&self, pos: uint) -> Option<char> {
        if pos >= self.str.len() {
            None
        } else {
            Some(self.str.char_at(pos))
        }
    }

    fn char_range_at(&self, pos: uint) -> Option<CharRange> {
        if pos >= self.str.len() {
            None
        } else {
            Some(self.str.char_range_at(pos))
        }
    }
}

impl<'a> Iterator<Token> for Tokens<'a> {
    fn next(&mut self) -> Option<Token> {
        while self.pos < self.str.len() {
            let CharRange { ch, next: pos } = self.str.char_range_at(self.pos);
            self.pos = pos;
            if ch.is_whitespace() { continue; }
            let (next, mut nextpos) = match self.char_range_at(self.pos) {
                Some(CharRange { ch, next: nextpos }) => (ch, nextpos),
                None => ('\0', self.str.len()),
            };
            match (ch, next) {
                ('(', _) => return Some(LParen),
                (')', _) => return Some(RParen),
                ('[', _) => return Some(LSqbr),
                (']', _) => return Some(RSqbr),
                ('{', _) => return Some(LBrace),
                ('}', _) => return Some(RBrace),
                ('=', '=') => {
                    self.pos = nextpos;
                    return Some(EqEq)
                }
                ('=', '>') => {
                    self.pos = nextpos;
                    return Some(FatArrow)
                }
                ('=', _) => return Some(Eq),
                ('>', '=') => {
                    self.pos = nextpos;
                    return Some(Ge)
                }
                ('>', '>') => {
                    self.pos = nextpos;
                    let (next, nextpos) = match self.char_range_at(self.pos) {
                        Some(CharRange { ch, next: nextpos }) => (ch, nextpos),
                        None => ('\0', self.str.len()),
                    };
                    match next {
                        '=' => {
                            self.pos = nextpos;
                            return Some(BinOpEq(ShiftRight))
                        }
                        _ => return Some(BinOp(ShiftRight)),
                    }
                }
                ('>', _) => return Some(Gt),
                ('<', '=') => {
                    self.pos = nextpos;
                    return Some(Le)
                }
                ('<', '<') => {
                    self.pos = nextpos;
                    let (next, nextpos) = match self.char_range_at(self.pos) {
                        Some(CharRange { ch, next: nextpos }) => (ch, nextpos),
                        None => ('\0', self.str.len()),
                    };
                    match next {
                        '=' => {
                            self.pos = nextpos;
                            return Some(BinOpEq(ShiftLeft))
                        }
                        _ => return Some(BinOp(ShiftLeft)),
                    }
                }
                ('<', '-') => {
                    self.pos = nextpos;
                    return Some(LArrow)
                }
                ('<', _) => return Some(Lt),
                ('&', '&') => {
                    self.pos = nextpos;
                    return Some(AndAnd)
                }
                ('&', '=') => {
                    self.pos = nextpos;
                    return Some(BinOpEq(And))
                }
                ('&', _) => return Some(BinOp(And)),
                ('|', '|') => {
                    self.pos = nextpos;
                    return Some(OrOr)
                }
                ('|', '=') => {
                    self.pos = nextpos;
                    return Some(BinOpEq(Or))
                }
                ('|', _) => return Some(BinOp(Or)),
                ('^', '^') => {
                    self.pos = nextpos;
                    return Some(XorXor)
                }
                ('^', '=') => {
                    self.pos = nextpos;
                    return Some(BinOpEq(Xor))
                }
                ('^', _) => return Some(BinOp(Xor)),
                ('!', _) => return Some(Not),
                ('~', _) => return Some(Tilde),
                ('+', '=') => {
                    self.pos = nextpos;
                    return Some(BinOpEq(Plus))
                }
                ('+', _) => return Some(BinOp(Plus)),
                ('-', '=') => {
                    self.pos = nextpos;
                    return Some(BinOpEq(Minus))
                }
                ('-', '>') => {
                    self.pos = nextpos;
                    return Some(RArrow)
                }
                ('-', _) => return Some(BinOp(Minus)),
                ('*', '=') => {
                    self.pos = nextpos;
                    return Some(BinOpEq(Times))
                }
                ('*', _) => return Some(BinOp(Times)),
                ('/', '=') => {
                    self.pos = nextpos;
                    return Some(BinOpEq(Divide))
                }
                ('/', _) => return Some(BinOp(Divide)),
                ('%', '=') => {
                    self.pos = nextpos;
                    return Some(BinOpEq(Modulo))
                }
                ('%', _) => return Some(BinOp(Modulo)),
                ('@', _) => return Some(At),
                ('.', '.') => {
                    self.pos = nextpos;
                    let (next, nextpos) = match self.char_range_at(self.pos) {
                        Some(CharRange { ch, next: nextpos }) => (ch, nextpos),
                        None => ('\0', self.str.len()),
                    };
                    match next {
                        '.' => {
                            self.pos = nextpos;
                            return Some(DotDotDot)
                        }
                        _ => return Some(DotDot),
                    }
                }
                ('.', _) => return Some(Dot),
                (',', _) => return Some(Comma),
                (';', _) => return Some(Semicolon),
                (':', ':') => {
                    self.pos = nextpos;
                    return Some(T_PAAMAYIM_NEKUDOTAYIM)
                }
                (':', _) => return Some(Colon),
                ('#', _) => return Some(Octothorpe),
                ('$', _) => return Some(Dollar),
                // Identifier
                (mut c, _) if c == '_' || c.is_alphabetic() => {
                    let mut s = format!("{}", c);
                    c = self.char_range_at(self.pos).map(|x| x.ch).unwrap_or('\0');
                    let mut i = 0;
                    while c == '_' || c.is_alphanumeric() {
                        println!("{}", c);
                        i += 1;
                        if i == 10 { fail!() }
                        s.push_char(c);
                        self.pos = nextpos;
                        match self.char_range_at(self.pos) {
                            Some(CharRange { ch, next }) => {
                                nextpos = next;
                                c = ch;
                            }
                            None => break,
                        }
                    }
                    match s {
                        ref s if s.as_slice() == "true" => return Some(LitBool(true)),
                        ref s if s.as_slice() == "false" => return Some(LitBool(false)),
                        s => return Some(Ident(s)),
                    }
                }
                // Char literal
                // TODO: escapes
                ('\'', _)  => {
                    let mut c;
                    match self.char_range_at(self.pos) {
                        Some(CharRange { ch, next }) => {
                            nextpos = next;
                            c = ch;
                        }
                        None => fail!("unterminated char literal"),
                    }
                    self.pos = nextpos;
                    match self.char_range_at(self.pos) {
                        Some(CharRange { ch: '\'', next }) => {
                            nextpos = next;
                        }
                        Some(CharRange { ch: c, .. }) =>
                            fail!("expected `'`, found `{}`", c),
                        _ => fail!("unterminated char literal"),
                    }
                    self.pos = nextpos;
                    return Some(LitChar(c))
                }
                // String literal
                // TODO: escapes, raw, byte
                ('"', _) => {
                    let mut s = String::new();
                    while self.char_at(nextpos) != Some('\"') {
                        let c;
                        match self.char_range_at(self.pos) {
                            Some(CharRange { ch, next }) => {
                                nextpos = next;
                                c = ch;
                            }
                            None => fail!("unterminated string literal"),
                        }
                        s.push_char(c);
                        self.pos = nextpos;
                    }
                    match self.char_range_at(self.pos) {
                        Some(CharRange { ch: '"', next }) => {
                            nextpos = next;
                        }
                        Some(CharRange { ch: c, .. }) =>
                            fail!("expected `\"`, found `{}`", c),
                        _ => fail!("unterminated string literal"),
                    }
                    self.pos = nextpos;
                    return Some(LitStr(s))
                }
                // Parse number
                // TODO: `.3`
                (c, _) if c.is_digit() || c == '.' => {
                    let mut s1 = format!("{}", c);
                    while self.char_at(self.pos).unwrap_or('\0').is_digit()
                       || self.char_at(self.pos) == Some('_') {
                        let mut c: char;
                        match self.char_range_at(self.pos) {
                            Some(CharRange { ch, next }) => {
                                nextpos = next;
                                c = ch;
                            }
                            None => break,
                        }
                        println!("s1 “{}” + ‘{}’", s1, c);
                        s1.push_char(c);
                        self.pos = nextpos;
                    }
                    if !(self.char_at(self.pos) == Some('.')) {
                        return Some(LitNum(s1, String::new()))
                    }
                    self.pos += 1;
                    let mut s2 = String::new();
                    while self.char_at(self.pos).unwrap_or('\0').is_digit()
                       || self.char_at(self.pos) == Some('_')
                       || self.char_at(self.pos) == Some('.') {
                        let mut c: char;
                        match self.char_range_at(self.pos) {
                            Some(CharRange { ch, next }) => {
                                nextpos = next;
                                c = ch;
                            }
                            None => break,
                        }
                        println!("s2 “{}” + ‘{}’", s2, c);
                        s2.push_char(c);
                        self.pos = nextpos;
                    }
                    return Some(LitNum(s1, s2))
                }
                _ => unimplemented!(),
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! token_test {
        ($i:ident: $e:expr => $($f:expr),*) => {
            #[test]
            fn $i() {
                let toks: Vec<Token> = Tokens::from_str($e).collect();
                assert_eq!(toks, vec![$($f),*]);
            }
        }
    }
    
    token_test!(brackets: "(\r[{  \t} ] \n)" => LParen, LSqbr, LBrace, RBrace, RSqbr, RParen)

    token_test!(cmp:
        "= = ==< << == = => ==<== == >= > >>" =>
            Eq, Eq, EqEq, Lt, BinOp(ShiftLeft), EqEq, Eq, FatArrow, EqEq, Le, Eq, EqEq, Ge, Gt, BinOp(ShiftRight)
    )

    token_test!(boolean: "& &&^ || |^^ !" => BinOp(And), AndAnd, BinOp(Xor), OrOr, BinOp(Or), XorXor, Not)

    token_test!(augment:
        "+ = += -= *= /= %= >>= <<= |= &= ^= ^^=" =>
            BinOp(Plus), Eq, BinOpEq(Plus), BinOpEq(Minus), BinOpEq(Times), BinOpEq(Divide), BinOpEq(Modulo),
            BinOpEq(ShiftRight), BinOpEq(ShiftLeft), BinOpEq(Or), BinOpEq(And), BinOpEq(Xor), XorXor, Eq
    )

    token_test!(miscellaneous:
        "@..... .. . ...~ $# ; , :::: : :: ." =>
            At, DotDotDot, DotDot, DotDot, Dot, DotDotDot, Tilde, Dollar, Octothorpe, Semicolon,
            Comma, T_PAAMAYIM_NEKUDOTAYIM, T_PAAMAYIM_NEKUDOTAYIM, Colon, T_PAAMAYIM_NEKUDOTAYIM,
            Dot
    )

    token_test!(ident:
        "$éllo_36a /false true _" =>
            Dollar, Ident("éllo_36a".to_string()), BinOp(Divide), LitBool(false), LitBool(true),
            Ident("_".to_string())
    )

    token_test!(char:
        "/'h'$ 'e'" =>
            BinOp(Divide), LitChar('h'), Dollar, LitChar('e')
    )

    token_test!(string:
        r#" "hello" $ "wórld"~ "# =>
            LitStr("hello".to_string()), Dollar, LitStr("wórld".to_string()), Tilde
    )

    token_test!(num:
        "5 1. 3.4" =>
            LitNum("5".to_string(), "".to_string()), LitNum("1".to_string(), "".to_string()),
            LitNum("3".to_string(), "4".to_string())
    )
}