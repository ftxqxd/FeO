/*!
This module contains utilities for parsing FeO source code.
*/

#[deriving(PartialEq, Show)]
pub enum FeOExpr {
    /// String literal (e.g. `"hello world"`)
    StrLiteral(String),
    /// Number literal (e.g. `42`)
    NumLiteral(f64),
    /// List literal (e.g. `[1, two, "three"]`)
    ListLiteral(Vec<FeOExpr>),
    /// Tuple literal (e.g. `(1, two, "three")`)
    TupleLiteral(Vec<FeOExpr>),
    /// Boolean literal (e.g. `true`)
    BoolLiteral(bool),
    /// Identifier (e.g. `foo`)
    Identifier(String),
    /// Function call (e.g. `f(x, y, z)`)
    Call(Box<FeOExpr>, Vec<FeOExpr>),
    /// Subscription (e.g. `list[0]`)
    Index(Box<FeOExpr>, Box<FeOExpr>),
    /// Property lookup (e.g. `x.y`)
    Lookup(Box<FeOExpr>, String),
    /// Binary operation (e.g. `1 + 2`)
    BinOp(String, Box<FeOExpr>, Box<FeOExpr>),
    /// Unary operation (e.g. `-1`)
    UnrOp(String, Box<FeOExpr>),
    /// Variable declaration (e.g. `let x = 1`)
    Declare(String, Box<FeOExpr>), // TODO: patterns
    /// Variable assignment (e.g. `x = 2`)
    Assign(String, Box<FeOExpr>), // TODO: patterns
    /// Conditional (e.g. `if cond { 1 } else { 2 }`)
    Conditional(Box<FeOExpr>, Box<FeOExpr>, Box<FeOExpr>),
    /// For-loop (e.g. `for i in [1, 2, 3].iter() {}`)
    ForLoop(String, Box<FeOExpr>, Vec<FeOExpr>), // TODO: patterns
    /// While-loop (e.g. `while true {}`)
    WhileLoop(Box<FeOExpr>, Vec<FeOExpr>),
    /// Function declaration (e.g. `fn f(x) { x * 2 }`)
    FnDecl(String, Vec<String>, Vec<FeOExpr>), // TODO: patterns
    /// Class declaration (e.g. `class Cow: Animal + Object {}`)
    ClassDecl(String, Vec<Box<FeOExpr>>, Vec<FeOExpr>),
    /// Block (e.g. `{ print(1); print(2) }`)
    Block(Vec<FeOExpr>),
    /// Nothing (for empty files)
    Nothing,
}

type ParserResult = Result<(FeOExpr, uint), (uint, String)>;

macro_rules! run_parser {
    ($str:ident, $i:ident, $f:ident) => {
        match $f($str, $i) {
            Err((pos, msg)) => return Err((pos, msg)),
            Ok((r, pos1)) => {
                $i = pos1;
                r
            },
        }
    }
}

// TODO: make sure that block-based expressions don't require semicolons

pub fn parse(str: &str) -> Result<FeOExpr, String> {
    return match stmts(str, 0) {
        Err((pos, msg)) => {
            Err(format!("pos {}: {}", pos, msg))
        },
        Ok((_, pos)) if pos < str.len() => {
            Err(format!("pos {}: unexpected char {}", pos, str[pos] as char))
        },
        Ok((re, _)) => {
            Ok(re)
        }
    };

    fn expect(str: &str, pos: uint, expct: char, ok: FeOExpr) -> ParserResult {
        if pos < str.len() && str[pos] as char == expct {
            Ok((ok, pos + 1))
        } else if pos < str.len() {
            Err((pos, format!("expected `{}`, found `{}`", expct,
                              str[pos] as char)))
        } else {
            Err((pos, format!("expected `{}`, found EOF", expct)))
        }
    }

    fn expect_str(str: &str, pos: uint, expct: &str, mut ok: FeOExpr)
                  -> ParserResult {
        let mut new_pos = pos;
        for c in expct.chars() {
            ok = try!(expect(str, new_pos, c, ok)).val0();
            new_pos += 1;
        }
        Ok((ok, new_pos))
    }

    fn any_whitespace(str: &str, mut pos: uint) -> uint {
        while pos < str.len() && (str[pos] as char).is_whitespace() {
            pos += 1;
        }
        pos
    }

    fn stmts(str: &str, mut pos: uint) -> ParserResult {
        let mut vec = vec!();
        let mut last_expr = false;
        while pos < str.len() {
            pos = any_whitespace(str, pos);
            match str[pos] as char {
                // End of block (no trailing semicolon)
                '}' if last_expr => break,
                // End of block (with trailing semicolon)
                '}' => {
                    vec.push(TupleLiteral(vec!()));
                    break;
                },
                // Anything but `}` at end of block
                c if last_expr =>
                    return Err((pos, format!("expected `}}`, found `{}`", c))),
                // Expression
                _ => vec.push(run_parser!(str, pos, expr)),
            }
            pos = any_whitespace(str, pos);
            let thing = expect(str, pos, ';', Block(vec!()));
            if thing.is_err() {
                last_expr = true;
            } else {
                let (_, pos1) = thing.unwrap();
                pos = pos1;
            }
        }
        Ok((Block(vec), pos))
    }

    fn block(str: &str, mut pos: uint) -> ParserResult {
        try!(expect(str, pos, '{', Nothing));
        pos += 1; // Skip `{`
        pos = any_whitespace(str, pos);
        let s = run_parser!(str, pos, stmts);
        pos = any_whitespace(str, pos);
        expect(str, pos, '}', s)
    }

    fn expr(str: &str, mut pos: uint) -> ParserResult {
        // TODO: allow `f(x)(y)`
        let e = run_parser!(str, pos, base);
        pos = any_whitespace(str, pos);
        if pos < str.len() && str[pos] as char == '(' {
            let mut v = vec!();
            loop {
                pos += 1; // Skip `(`
                pos = any_whitespace(str, pos);
                if pos < str.len() && str[pos] as char == ')' {
                    // Empty parameter list
                    break;
                }
                v.push(run_parser!(str, pos, expr));
                pos = any_whitespace(str, pos);
                if pos < str.len() && str[pos] as char != ',' {
                    break;
                }
            }
            expect(str, pos, ')', Call(box e, v))
        } else {
            Ok((e, pos))
        }
    }

    fn base(str: &str, mut pos: uint) -> ParserResult {
        match str[pos] as char {
            // Parenthesised expression
            '(' => {
                pos += 1; // Skip `(`
                let inner = run_parser!(str, pos, expr);
                expect(str, pos, ')', inner)
            },
            // Number literal
            '0'..'9' => {
                let mut buf = String::new();
                while pos < str.len()
                   && "0123456789.".contains_char(str[pos] as char) {
                    buf.push_str(format!("{}", str[pos] as char).as_slice());
                    pos += 1;
                }
                Ok((NumLiteral(from_str(buf.as_slice()).unwrap()), pos))
            },
            // Block expression
            '{' => {
                // "{" (<stmt>);* ";"? "}"
                block(str, pos)
            },
            // Conditional
            'i' if pos + 2 < str.len() && str[pos + 1] as char == 'f'
                            && (str[pos + 2] as char).is_whitespace() => {
                // "if" <expr> <block> "else" (<block>|<conditional>)
                pos += 2; // Skip `if`
                pos = any_whitespace(str, pos);
                let cond = run_parser!(str, pos, expr);
                pos = any_whitespace(str, pos);
                let yes = run_parser!(str, pos, block);
                pos = any_whitespace(str, pos);
                let success = expect_str(str, pos, "else", Nothing);
                match success {
                    Ok(_) => { // Else-condition exists
                        pos += 4; // Skip `else`
                        pos = any_whitespace(str, pos);
                        let no = run_parser!(str, pos, expr);
                        match no {
                            Block(_) | Conditional(..) => {},
                            _ =>
                                return Err((pos,
                                    "`else` should be followed by `if` or a block".to_string()))
                        }
                        Ok((Conditional(box cond, box yes, box no), pos))
                    },
                    Err(_) => { // No else-condition
                        Ok((Conditional(box cond, box yes, box TupleLiteral(vec!())),
                            pos))
                    },
                }
            },
            // Identifier or Boolean literal
            // (extra `'i'` is a workaround for mozilla/rust#13027)
            'i' | 'a'..'z' | 'A'..'Z' | '_' => {
                let mut buf = format!("{}", str[pos] as char);
                pos += 1;
                while pos < str.len() && (str[pos] as char).is_alphanumeric() {
                    buf.push_str(format!("{}", str[pos] as char).as_slice());
                    pos += 1;
                }
                if buf == "false".to_string() {
                    Ok((BoolLiteral(false), pos))
                } else if buf == "true".to_string() {
                    Ok((BoolLiteral(true), pos))
                } else {
                    Ok((Identifier(buf), pos))
                }
            },
            // Unknown; throw an error
            c => Err((pos, format!("unexpected char `{}`", c))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_int() {
        assert_eq!(parse("1.2"),
                   Ok(Block(vec!(NumLiteral(1.2)))));
    }

    #[test]
    fn parse_multiple_exprs() {
        assert_eq!(parse("1.2; 3.4"),
                   Ok(Block(vec!(NumLiteral(1.2), NumLiteral(3.4)))));
    }

    #[test]
    fn parse_block() {
        assert_eq!(parse("1.2; {3.4; 5.6}"),
                   Ok(Block(vec!(NumLiteral(1.2), Block(vec!(NumLiteral(3.4),
                        NumLiteral(5.6)))))));
    }

    #[test]
    fn parse_whitespace() {
        assert_eq!(parse("  \n 1.2  ; \t  {  \r 3.4  ;  5.6  \n\n  } \r\n  "),
                   parse("1.2;{3.4;5.6}"));
        assert_eq!(parse(" \n f   (   3  \t , \r  5   );    hello ; world  "),
                   parse("f(3,5);hello;world"));
    }

    #[test]
    fn parse_identifier() {
        assert_eq!(parse("ifhello; world"),
                  Ok(Block(vec!(Identifier("ifhello".to_string()),
                                Identifier("world".to_string())))));
    }

    #[test]
    fn parse_trailing_semicolon() {
        assert_eq!(parse("{1; 2;}"),
                   Ok(Block(vec!(Block(vec!(NumLiteral(1.0), NumLiteral(2.0),
                        TupleLiteral(vec!())))))));
    }

    #[test]
    fn parse_if() {
        assert_eq!(parse("if 1 { 2 }"),
                   Ok(Block(vec!(Conditional(box NumLiteral(1.0,), box Block(vec!(
                        NumLiteral(2.0))), box TupleLiteral(vec!()))))));
    }

    #[test]
    fn parse_if_else() {
        assert_eq!(parse("if 1 { 2 } else { 3 }"),
                   Ok(Block(vec!(Conditional(box NumLiteral(1.0), box Block(vec!(
                        NumLiteral(2.0))), box Block(vec!(NumLiteral(3.0))))))));
    }

    #[test]
    fn parse_if_else_if_else() {
        assert_eq!(parse("if 1 { 2 } else if 2 { 3 } else { 4 }"),
                   Ok(Block(vec!(Conditional(box NumLiteral(1.0), box Block(vec!(
                        NumLiteral(2.0))), box Conditional(box NumLiteral(2.0),
                            box Block(vec!(NumLiteral(3.0))),
                            box Block(vec!(NumLiteral(4.0)))))))));
    }

    #[test]
    fn parse_if_else_expr() {
        assert_eq!(parse("if 1 { 2 } else 3"),
            Err("pos 17: `else` should be followed by `if` or a block".to_string()));
    }

    #[test]
    fn parse_paren_expr() {
        assert_eq!(parse("(((3))); (4); ((5)); 6; ((7));"),
                   parse("3; 4; 5; 6; 7;"));
    }

    #[test]
    fn parse_fn_call() {
        assert_eq!(parse("1.3 ( )"),
                   Ok(Block(vec!(Call(box NumLiteral(1.3), vec!())))));
        assert_eq!(parse("f ( 3 , 4 )"),
                   Ok(Block(vec!(Call(box Identifier("f".to_string()),
                        vec!(NumLiteral(3.0), NumLiteral(4.0)))))));
    }
}
