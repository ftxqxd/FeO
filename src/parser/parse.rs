/*!
This module contains utilities for parsing FeO source code.
*/

use std::vec_ng::Vec;

#[deriving(Eq, Show)]
pub enum FeOExpr {
    /// String literal (e.g. `"hello world"`)
    StrLiteral(~str),
    /// Number literal (e.g. `42`)
    NumLiteral(f64),
    /// List literal (e.g. `[1, 2, "three"]`)
    ListLiteral(Vec<FeOExpr>),
    /// Tuple literal (e.g. `(1, 2, "three")`)
    TupleLiteral(Vec<FeOExpr>),
    /// Boolean literal (e.g. `true`)
    BoolLiteral(bool),
    /// Identifier (e.g. `foo`)
    Identifier(~str),
    /// Binary operation (e.g. `1 + 2`)
    BinOp(~str, ~FeOExpr, ~FeOExpr),
    /// Unary operation (e.g. `-1`)
    UnrOp(~str, ~FeOExpr),
    /// Variable declaration (e.g. `let x = 1`)
    Declare(~str, ~FeOExpr), // TODO: patterns
    /// Variable assignment (e.g. `x = 2`)
    Assign(~str, ~FeOExpr), // TODO: patterns
    /// Conditional (e.g. `if cond { 1 } else { 2 }`)
    Conditional(~FeOExpr, ~FeOExpr, ~FeOExpr),
    /// For-loop (e.g. `for i in [1,2,3].iter() {}`)
    ForLoop(~str, ~FeOExpr, Vec<FeOExpr>), // TODO: patterns
    /// While-loop (e.g. `while true {}`)
    WhileLoop(~FeOExpr, Vec<FeOExpr>),
    /// Function declaration (e.g. `fn func(x) { x * 2 }`)
    FnDecl(~str, Vec<~str>, Vec<FeOExpr>), // TODO: patterns
    /// Class declaration (e.g. `class Cow: Animal + Object {}`)
    ClassDecl(~str, Vec<~FeOExpr>, Vec<FeOExpr>),
    /// Block (e.g. `{ print(1); print(2) }`)
    Block(Vec<FeOExpr>),
    /// Nothing (for empty files)
    Nothing,
}

type ParserResult = Result<(FeOExpr, uint), (uint, ~str)>;

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

pub fn parse(str: &str) -> Result<FeOExpr, ~str> {
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
                    return Err((pos, format!("expected `\\}`, found `{}`", c))),
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
        pos += 1;
        pos = any_whitespace(str, pos);
        let s = run_parser!(str, pos, stmts);
        pos = any_whitespace(str, pos);
        expect(str, pos, '}', s)
    }

    fn expr(str: &str, mut pos: uint) -> ParserResult {
        match str[pos] as char {
            // Number literal
            '0'..'9' => {
                let mut buf = ~"";
                while pos < str.len()
                   && "0123456789.".contains_char(str[pos] as char) {
                    buf.push_str(format!("{}", str[pos] as char));
                    pos += 1;
                }
                Ok((NumLiteral(from_str(buf).unwrap()), pos))
            },
            // Block expression
            '{' => {
                // "{" (<stmt>);* "}"
                block(str, pos)
            },
            // Conditional
            'i' if pos + 1 < str.len() && str[pos + 1] as char == 'f' => {
                // "if" <expr> <block> "else" <block>
                pos += 2; // Skip `if`
                pos = any_whitespace(str, pos);
                let cond = run_parser!(str, pos, expr);
                pos = any_whitespace(str, pos);
                let yes = run_parser!(str, pos, block);
                pos = any_whitespace(str, pos);
                try!(expect_str(str, pos, "else", Nothing));
                pos += 4; // Skip `else`
                pos = any_whitespace(str, pos);
                let no = run_parser!(str, pos, block);
                Ok((Conditional(~cond, ~yes, ~no), pos))
            },
            // Identifier or Boolean literal
            'a'..'z' | 'A'..'Z' | '_' => {
                let mut buf = format!("{}", str[pos] as char);
                pos += 1;
                while pos < str.len() && (str[pos] as char).is_alphanumeric() {
                    buf.push_str(format!("{}", str[pos] as char));
                    pos += 1;
                }
                if buf == ~"false" {
                    Ok((BoolLiteral(false), pos))
                } else if buf == ~"true" {
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
                   Ok(Block(vec!(NumLiteral(1.2), Block(vec!(NumLiteral(3.4),
                        NumLiteral(5.6)))))));
    }

    #[test]
    fn parse_identifier() {
        assert_eq!(parse("hello; world"),
                  Ok(Block(vec!(Identifier(~"hello"), Identifier(~"world")))));
    }

    #[test]
    fn parse_trailing_semicolon() {
        assert_eq!(parse("{1; 2;}"),
                   Ok(Block(vec!(Block(vec!(NumLiteral(1.0), NumLiteral(2.0),
                        TupleLiteral(vec!())))))));
    }

    #[test]
    fn parse_if_else() {
        assert_eq!(parse("if 1 { 2 } else { 3 }"),
                   Ok(Block(vec!(Conditional(~NumLiteral(1.0), ~Block(vec!(
                        NumLiteral(2.0))), ~Block(vec!(NumLiteral(3.0))))))));
    }
}