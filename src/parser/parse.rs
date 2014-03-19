/*!
This module contains utilities for parsing FeO source code.
*/

use std::vec_ng::Vec;

#[deriving(Eq, Show)]
pub enum FeOExpr {
    /// String literal (e.g. "hello world")
    StrLiteral(~str),
    /// Number literal (e.g. 42)
    NumLiteral(f64),
    /// List literal (e.g. [1, 2, "three"])
    ListLiteral(Vec<FeOExpr>),
    /// Tuple literal (e.g. (1, 2, "three"))
    TupleLiteral(Vec<FeOExpr>),
    /// Boolean literal (e.g. true)
    BoolLiteral(bool),
    /// Identifier (e.g. foo)
    Identifier(~str),
    /// Binary operation (e.g. 1 + 2)
    BinOp(~str, ~FeOExpr, ~FeOExpr),
    /// Unary operation (e.g. -1)
    UnrOp(~str, ~FeOExpr),
    /// Variable declaration (e.g. let x = 1)
    Declare(~str, ~FeOExpr), // TODO: patterns
    /// Variable assignment (e.g. x = 2)
    Assign(~str, ~FeOExpr), // TODO: patterns
    /// Conditional (e.g. if cond { 1 } else { 2 })
    Conditional(~FeOExpr, ~FeOExpr, ~FeOExpr),
    /// For-loop (e.g. for i in [1,2,3].iter() {})
    ForLoop(~str, ~FeOExpr, Vec<FeOExpr>), // TODO: patterns
    /// While-loop (e.g. while true {})
    WhileLoop(~FeOExpr, Vec<FeOExpr>),
    /// Function declaration (e.g. fn func(x) { x * 2 })
    FnDecl(~str, Vec<~str>, Vec<FeOExpr>), // TODO: patterns
    /// Class declaration (e.g. class Cow: Animal, Object {})
    ClassDecl(~str, Vec<~FeOExpr>, Vec<FeOExpr>),
    /// Block (e.g. {print(1); print(2)})
    Block(Vec<FeOExpr>),
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

    fn expect(str: &str, pos: uint, expect: char, ok: FeOExpr) -> ParserResult {
        if pos < str.len() && str[pos] as char == expect {
            Ok((ok, pos + 1))
        } else if pos < str.len() {
            Err((pos, format!("expected `{}`, found `{}`", expect, str[pos] as char)))
        } else {
            Err((pos, format!("expected `{}`, found EOF", expect)))
        }
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

    fn expr(str: &str, mut pos: uint) -> ParserResult {
        match str[pos] as char {
            // Number literal
            '0'..'9' => {
                let mut buf = ~"";
                while pos < str.len() && "0123456789.".contains_char(str[pos] as char) {
                    buf.push_str(format!("{}", str[pos] as char));
                    pos += 1;
                }
                Ok((NumLiteral(from_str(buf).unwrap()), pos))
            },
            // Block expression
            '{' => {
                pos += 1; // Skip `{`
                pos = any_whitespace(str, pos);
                let s = run_parser!(str, pos, stmts);
                pos = any_whitespace(str, pos);
                expect(str, pos, '}', s)
            },
            // Identifier
            'a'..'z' | 'A'..'Z' | '_' => {
                let mut buf = format!("{}", str[pos] as char);
                pos += 1;
                while pos < str.len() && (str[pos] as char).is_alphanumeric() {
                    buf.push_str(format!("{}", str[pos] as char));
                    pos += 1;
                }
                Ok((Identifier(buf), pos))
            },
            // Unknown; throw an error
            c => Err((pos, format!("unexpected char `{}`", c)))
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
}