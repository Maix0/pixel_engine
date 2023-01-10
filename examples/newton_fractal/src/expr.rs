#![allow(unused)]
mod token_stream;
use token_stream::Token;

use bumpalo::Bump;

#[derive(Debug, PartialEq)]
enum Expr<'arena> {
    RealNumber {
        val: f64,
    },
    ImaginaryNumber {
        val: f64,
    },
    ComplexNumber {
        val: num::Complex<f64>,
    },
    Variable {
        name: &'arena mut str,
    },
    FunctionCall {
        ident: &'arena mut str,
        args: bumpalo::collections::Vec<'arena, &'arena mut Expr<'arena>>,
    },
    Operator {
        op: Operator,
        rhs: &'arena mut Expr<'arena>,
        lhs: &'arena mut Expr<'arena>,
    },
}

impl<'arena> Expr<'arena> {
    #[allow(clippy::mut_from_ref)]
    fn clone_in(&self, arena: &'arena Bump) -> &'arena mut Self {
        use Expr::*;
        arena.alloc(match self {
            RealNumber { val } => RealNumber { val: *val },
            ImaginaryNumber { val } => ImaginaryNumber { val: *val },
            ComplexNumber { val } => ComplexNumber { val: *val },
            Variable { name } => Variable {
                name: arena.alloc_str(name),
            },
            FunctionCall { ident, args } => FunctionCall {
                ident: arena.alloc_str(ident),
                args: bumpalo::collections::FromIteratorIn::from_iter_in(
                    args.iter().map(|c| c.clone_in(arena)),
                    arena,
                ),
            },
            Operator { op, rhs, lhs } => Operator {
                op: *op,
                rhs: rhs.clone_in(arena),
                lhs: lhs.clone_in(arena),
            },
        })
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(u16)]
pub enum Operator {
    Plus = 1,
    Minus = 2,

    Multiply = 11,
    Divide = 12,
    Modulo = 13,

    Pow = 21,

    UnaryMinus = 31,
    UnaryPlus = 32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Associativity {
    Right,
    Left,
}

impl Operator {
    fn as_str(self) -> &'static str {
        match self {
            Self::Pow => "^",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Divide => "/",
            Self::Multiply => "*",
            Self::Modulo => "%",
            Self::UnaryMinus => "-",
            Self::UnaryPlus => "+",
        }
    }

    fn from_str(input: &str) -> Option<Self> {
        match input {
            "^" => Some(Self::Pow),
            "+" => Some(Self::Plus),
            "-" => Some(Self::Minus),
            "/" => Some(Self::Divide),
            "*" => Some(Self::Multiply),
            "%" => Some(Self::Modulo),
            _ => None,
        }
    }

    fn associativity(&self) -> Associativity {
        match self {
            Self::Pow => Associativity::Left,
            _ => Associativity::Right,
        }
    }

    fn class(self) -> u8 {
        self as u8 / 10
    }
}

fn function_pass<'input>(
    mut iter: std::iter::Peekable<
        impl Iterator<Item = Result<token_stream::Token<'input>, InvalidToken<'input>>> + 'input,
    >,
) -> impl Iterator<Item = Result<token_stream::Token<'input>, InvalidToken<'input>>> + 'input {
    let iter = {
        let mut need_sep = Option::<u8>::None;
        std::iter::from_fn(move || {
            if let Some(n) = need_sep.as_mut() {
                *n -= 1;
                if *n == 0 {
                    need_sep = None;
                    Some(Ok(token_stream::Token::Whitespace))
                } else {
                    iter.next()
                }
            } else {
                let next = iter.next();
                match &next {
                    Some(Ok(token_stream::Token::Ident(word))) if word.len() > 1 => {
                        if let Some(Ok(token_stream::Token::LeftParenthesis)) = iter.peek() {
                            need_sep = Some(2);
                        }
                    }
                    Some(Ok(token_stream::Token::Comma)) => {
                        need_sep = Some(1);
                    }
                    _ => {}
                };
                next
            }
        })
    };

    // fn function_pass_end<'input>(
    //     mut iter: std::iter::Peekable<
    //         impl Iterator<Item = Result<token_stream::Token<'input>, InvalidToken<'input>>> + 'input,
    //     >,
    //     done: &std::cell::Cell<bool>,
    // ) -> impl Iterator<Item = Result<token_stream::Token<'input>, InvalidToken<'input>>> + 'input
    // {
    //     let mut paren_count = 0u8;
    //     let mut search_end = false;
    //     std::iter::from_fn(move || {
    //         let peek = iter.peek();
    //         if search_end {
    //             match peek {
    //                 Some(Ok(Token::RightParenthesis)) if paren_count == 1 => {
    //                     paren_count = 0;
    //                     done.set(true);
    //                     search_end = false;
    //                     return Some(Ok(Token::Whitespace));
    //                 }
    //                 Some(Ok(Token::Comma)) => {
    //                     search_end = false;
    //                     done.set(true);
    //                     return Some(Ok(Token::Whitespace));
    //                 }
    //                 Some(Ok(Token::RightParenthesis)) => {
    //                     paren_count = paren_count
    //                         .checked_sub(1)
    //                         .ok_or(AstBuildError::MissingParenthesis)?;
    //                 }
    //                 Some(Ok(Token::LeftParenthesis)) => {
    //                     paren_count = paren_count
    //                         .checked_add(1)
    //                         .ok_or(AstBuildError::MissingParenthesis)?;
    //                 }
    //             }
    //         }

    //     if matches!(peek, Some(Ok(token_stream::Token::Whitespace))) {
    //         if search_end {
    //             done.set(false);
    //             let mut child_whitespace = std::cell::Cell::new(true);
    //             let mut sub_iter = iter.by_ref().take_while(|e| !done.get());
    //             let dyn_sub_iter = (&mut iter
    //                 as &mut dyn Iterator<Item = Result<Token<'input>, InvalidToken<'input>>>);
    //         } else {
    //             search_end = true;
    //             paren_count = 1;
    //         }
    //     }
    //     iter.next()
    // })
    // }
    // let done = true.into();
    // function_pass_end(iter.peekable(), &done)
    iter
}

fn implicit_multiple_pass<'input>(
    mut iter: std::iter::Peekable<
        impl Iterator<Item = Result<token_stream::Token<'input>, InvalidToken<'input>>> + 'input,
    >,
) -> impl Iterator<Item = Result<token_stream::Token<'input>, InvalidToken<'input>>> + 'input {
    let mut need_sep = Option::<u8>::None;
    std::iter::from_fn(move || {
        if let Some(n) = need_sep.as_mut() {
            *n -= 1;
            if *n == 0 {
                need_sep = None;
                Some(Ok(token_stream::Token::Operator(Operator::Multiply)))
            } else {
                iter.next()
            }
        } else {
            let next = iter.next();
            if matches!(&next, Some(Ok(token_stream::Token::Ident(w))) if w.len() == 1)
                || matches!(
                    &next,
                    Some(Ok(
                        token_stream::Token::Literal(_) | token_stream::Token::RightParenthesis
                    ))
                )
            {
                if let Some(Ok(
                    token_stream::Token::LeftParenthesis
                    | token_stream::Token::Ident(_)
                    | token_stream::Token::Literal(_),
                )) = iter.peek()
                {
                    need_sep = Some(1);
                }
            }
            next
        }
    })
}

fn unary_pass<'input>(
    mut iter: std::iter::Peekable<
        impl Iterator<Item = Result<token_stream::Token<'input>, InvalidToken<'input>>> + 'input,
    >,
) -> impl Iterator<Item = Result<token_stream::Token<'input>, InvalidToken<'input>>> + 'input {
    let _next = iter.peek_mut().map(|next| match next {
        Ok(token_stream::Token::Operator(op @ Operator::Minus)) => {
            *op = Operator::UnaryMinus;
        }
        Ok(token_stream::Token::Operator(op @ Operator::Plus)) => {
            *op = Operator::UnaryPlus;
        }
        _ => (),
    });
    std::iter::from_fn(move || {
        let next = iter.next();
        if let Some(Ok(
            token_stream::Token::Operator(_)
            | token_stream::Token::Comma
            | token_stream::Token::Whitespace
            | token_stream::Token::LeftParenthesis,
        )) = next
        {
            match iter.peek_mut() {
                Some(Ok(token_stream::Token::Operator(op @ Operator::Minus))) => {
                    *op = Operator::UnaryMinus
                }
                Some(Ok(token_stream::Token::Operator(op @ Operator::Plus))) => {
                    *op = Operator::UnaryPlus
                }
                _ => (),
            }
        }
        next
    })
}

pub use token_stream::InvalidToken;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstBuildError<'input> {
    InvalidToken(InvalidToken<'input>),
    MissingParenthesis,
    MissingOperator,
    MissingOperand,
    UnkownError,
}

impl<'input> From<InvalidToken<'input>> for AstBuildError<'input> {
    fn from(value: InvalidToken<'input>) -> Self {
        Self::InvalidToken(value)
    }
}

impl<'arena> std::fmt::Display for Expr<'arena> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            self.to_string_inner(f)
        } else {
            self.to_string_inner_min_parens(f, None)
        }
    }
}

fn print<T: std::fmt::Debug + ?Sized>(t: &T, level: u16) {
    println!("{:width$}[{level}]{t:?}", "", width = (level * 4) as usize);
}

// thread_local! {static CURRENT_LEVEL: std::cell::Cell<u16> = 0.into();}

impl<'arena> Expr<'arena> {
    pub fn parse<'input, 'words: 'input + 'word, 'word: 'input>(
        arena: &'arena bumpalo::Bump,
        input: &'input str,
        reserved_words: &'words [&'word str],
    ) -> Result<&'arena mut Expr<'arena>, AstBuildError<'input>> {
        let iter = token_stream::parse_tokens(input, reserved_words);
        let iter = function_pass(iter.peekable());
        let iter = implicit_multiple_pass(iter.peekable());
        let iter = unary_pass(iter.peekable());
        let mut iter = iter.fuse();
        // .inspect(|t| print(&t, CURRENT_LEVEL.with(std::cell::Cell::get)));

        Self::parse_iter(arena, iter, &(true.into()), 1)
    }

    fn parse_iter<'input, 'words: 'input + 'word, 'word: 'input>(
        arena: &'arena bumpalo::Bump,
        mut iter: impl Iterator<Item = Result<token_stream::Token<'input>, InvalidToken<'input>>>,
        check_func_sep: &std::cell::Cell<bool>,
        level: u16,
    ) -> Result<&'arena mut Expr<'arena>, AstBuildError<'input>> {
        let mut output = Vec::<&mut Self>::new();
        let mut operator = Vec::<token_stream::Token<'input>>::new();
        let mut was_function_call = false;
        loop {
            use token_stream::Token;
            if let Some(token) = iter.next() {
                //print(&format_args!("Output Buffer: {output:?}"), level);
                match token? {
                    Token::Whitespace => {
                        check_func_sep.set(false);
                        let parens_count = std::cell::Cell::new(1u16);
                        let child_check_func_sep = std::cell::Cell::new(true);
                        let error = std::cell::Cell::new(false);
                        let mut sub_iter = iter
                            .by_ref()
                            .inspect(|t| {
                                // print(
                                //     &((child_whitespace.get() && !matches!(t, &Ok(Token::Comma)))
                                //         || parens_count.get() != 0),
                                // );
                                match t {
                                    Ok(Token::LeftParenthesis) => {
                                        parens_count.set(match parens_count.get().checked_add(1) {
                                            Some(n) => n,
                                            None => {
                                                error.set(true);
                                                255
                                            }
                                        });
                                    }
                                    Ok(Token::RightParenthesis) => {
                                        parens_count.set(match parens_count.get().checked_sub(1) {
                                            Some(n) => n,
                                            None => {
                                                error.set(true);
                                                dbg!(255)
                                            }
                                        });
                                    }
                                    _ => (),
                                }
                            })
                            .take_while(|t| parens_count.get() != 0 || !child_check_func_sep.get());
                        // print("LEVEL START", level);
                        let ast = Self::parse_iter(
                            arena,
                            &mut sub_iter
                                as &mut dyn Iterator<
                                    Item = Result<Token<'input>, InvalidToken<'input>>,
                                >,
                            &child_check_func_sep,
                            level + 1,
                        );
                        // print(
                        //     &format_args!(
                        //         "LEVEL END: {}",
                        //         if error.get() { "Error" } else { "No Error" }
                        //     ),
                        //     level + 1,
                        // );
                        if error.get() {
                            return Err(dbg!(AstBuildError::UnkownError));
                        }
                        check_func_sep.set(true);
                        match output.last_mut() {
                            Some(Expr::FunctionCall { args, .. }) => {
                                args.push(ast?);
                            }
                            t => {
                                // print(&output, level);
                                return Err(AstBuildError::MissingOperator);
                            }
                        }
                    }
                    Token::Literal(v) => output.push(arena.alloc(Expr::RealNumber { val: v })),
                    Token::Ident(name) if name.len() == 1 => {
                        output.push(arena.alloc(Expr::Variable {
                            name: arena.alloc_str(name),
                        }))
                    }
                    Token::Ident(name) => {
                        was_function_call = true;
                        // print("FUNCTION CALL", level);
                        output.push(arena.alloc(Expr::FunctionCall {
                            ident: arena.alloc_str(name),
                            args: bumpalo::collections::Vec::with_capacity_in(2, arena),
                        }))
                    }

                    Token::Comma => {
                        loop {
                            let Some(op) = operator.pop() else {
                                // print("Missing Parenthesis Error", level);
                                break;
                            };
                            match op {
                                Token::LeftParenthesis => break,
                                Token::Operator(
                                    o @ (Operator::UnaryMinus | Operator::UnaryPlus),
                                ) => {
                                    let rhs = output.pop().ok_or(AstBuildError::MissingOperand)?;
                                    output.push(arena.alloc(Expr::Operator {
                                        op: o,
                                        lhs: arena.alloc(Expr::RealNumber { val: 0.0 }),
                                        rhs,
                                    }));
                                }
                                Token::Operator(o) => {
                                    let rhs = output.pop().ok_or(AstBuildError::MissingOperand)?;
                                    let lhs = output.pop().ok_or(AstBuildError::MissingOperand)?;

                                    output.push(arena.alloc(Expr::Operator { op: o, rhs, lhs }));
                                }
                                _ => (),
                            }
                        }
                        // print(&format_args!("Comma: {}", output.len()), level);
                        return output.pop().ok_or(match output.len() {
                            0 => AstBuildError::UnkownError,
                            _ => AstBuildError::MissingOperator,
                        });
                    }
                    t @ Token::LeftParenthesis if !was_function_call => operator.push(t),
                    t @ Token::LeftParenthesis => was_function_call = false,
                    Token::Operator(op1) => {
                        loop {
                            let Some(peek) = operator.last() else {break;};
                            match peek {
                                Token::LeftParenthesis => break,
                                Token::Operator(op2)
                                    if op2.class() > op1.class()
                                        || (op1.class() == op2.class()
                                            && op1.associativity() == Associativity::Left) =>
                                {
                                    let op = operator.pop().unwrap();
                                    match op {
                                        Token::Operator(
                                            o @ (Operator::UnaryMinus | Operator::UnaryPlus),
                                        ) => {
                                            let rhs = output
                                                .pop()
                                                .ok_or(AstBuildError::MissingOperand)?;
                                            output.push(arena.alloc(Expr::Operator {
                                                op: o,
                                                lhs: arena.alloc(Expr::RealNumber { val: 0.0 }),
                                                rhs,
                                            }));
                                        }
                                        Token::Operator(o) => {
                                            let rhs = output
                                                .pop()
                                                .ok_or(AstBuildError::MissingOperand)?;
                                            let lhs = output
                                                .pop()
                                                .ok_or(AstBuildError::MissingOperand)?;

                                            output.push(arena.alloc(Expr::Operator {
                                                op: o,
                                                rhs,
                                                lhs,
                                            }));
                                        }
                                        _ => (),
                                    }
                                }
                                _ => break,
                                // dbg!("Done");
                                // return Err(AstBuildError::InvalidToken(InvalidToken {
                                //     span: None,
                                // }));
                                //}
                            }
                        }
                        operator.push(Token::Operator(op1));
                    }
                    Token::RightParenthesis => loop {
                        let Some(op) = operator.pop() else {
                            // print("Missing Parenthesis Error", level);
                            return Err(dbg!(AstBuildError::MissingParenthesis));
                        };
                        match op {
                            Token::LeftParenthesis => break,
                            Token::Operator(o @ (Operator::UnaryMinus | Operator::UnaryPlus)) => {
                                let rhs = output.pop().ok_or(AstBuildError::MissingOperand)?;
                                output.push(arena.alloc(Expr::Operator {
                                    op: o,
                                    lhs: arena.alloc(Expr::RealNumber { val: 0.0 }),
                                    rhs,
                                }));
                            }
                            Token::Operator(o) => {
                                let rhs = output.pop().ok_or(AstBuildError::MissingOperand)?;
                                let lhs = output.pop().ok_or(AstBuildError::MissingOperand)?;

                                output.push(arena.alloc(Expr::Operator { op: o, rhs, lhs }));
                            }
                            _ => (),
                        }
                    },
                }
            } else {
                for op in operator.into_iter().rev() {
                    match op {
                        Token::LeftParenthesis => return Err(AstBuildError::MissingParenthesis),
                        Token::Operator(o @ (Operator::UnaryMinus | Operator::UnaryPlus)) => {
                            let rhs = output.pop().ok_or(AstBuildError::MissingOperand)?;
                            output.push(arena.alloc(Expr::Operator {
                                op: o,
                                lhs: arena.alloc(Expr::RealNumber { val: 0.0 }),
                                rhs,
                            }));
                        }
                        Token::Operator(o) => {
                            let rhs = output.pop().ok_or(AstBuildError::MissingOperand)?;
                            let lhs = output.pop().ok_or(AstBuildError::MissingOperand)?;

                            output.push(arena.alloc(Expr::Operator { op: o, rhs, lhs }));
                        }
                        Token::Comma | Token::Whitespace => { /* No-op but still an operator */ }
                        _ => (),
                    }
                }
                break;
            }
        }
        //print(&format_args!("End: {}", output.len()), level);
        output.pop().ok_or(match output.len() {
            0 => AstBuildError::UnkownError,
            _ => AstBuildError::MissingOperator,
        })
    }

    fn to_string_inner_min_parens(
        &self,
        mut buf: impl std::fmt::Write,
        parent_precedence: Option<u8>,
    ) -> std::fmt::Result {
        use std::fmt::Write;
        match self {
            Expr::FunctionCall { ident, args } => {
                write!(buf, "{ident}(")?;
                for arg in args.iter().take(args.len() - 1) {
                    arg.to_string_inner_min_parens(&mut buf as &mut dyn std::fmt::Write, None)?;
                    write!(buf, ", ")?;
                }
                if let Some(arg) = args.last() {
                    arg.to_string_inner_min_parens(&mut buf as &mut dyn std::fmt::Write, None)?;
                }
                write!(buf, ")")?;
            }
            Expr::RealNumber { val } if val.is_sign_negative() => write!(buf, "({val})")?,
            Expr::RealNumber { val } => write!(buf, "{val}")?,
            Expr::ImaginaryNumber { val } if val.is_sign_negative() => write!(buf, "({val}i)")?,
            Expr::ImaginaryNumber { val } => write!(buf, "{val}i")?,
            Expr::ComplexNumber { val }
                if val.re.is_sign_negative() || val.im.is_sign_negative() =>
            {
                write!(buf, "({val})")?;
            }
            Expr::ComplexNumber { val } => write!(buf, "{val}")?,
            Expr::Variable { name } => write!(buf, "{name}")?,
            Expr::Operator {
                op: op @ (Operator::UnaryMinus | Operator::UnaryPlus),
                rhs,
                ..
            } => {
                if parent_precedence.map(|p| op.class() < p).unwrap_or(false) {
                    write!(buf, "(")?;
                    write!(buf, "{}", op.as_str())?;
                    rhs.to_string_inner_min_parens(
                        &mut buf as &mut dyn std::fmt::Write,
                        Some(op.class()),
                    )?;
                    write!(buf, ")")?;
                } else {
                    write!(buf, "{}", op.as_str())?;
                    rhs.to_string_inner_min_parens(
                        &mut buf as &mut dyn std::fmt::Write,
                        Some(op.class()),
                    )?;
                }
            }
            Expr::Operator { op, rhs, lhs } => {
                if parent_precedence.map(|p| op.class() < p).unwrap_or(false) {
                    write!(buf, "(")?;
                    lhs.to_string_inner_min_parens(
                        &mut buf as &mut dyn std::fmt::Write,
                        Some(op.class()),
                    )?;
                    write!(buf, " {} ", op.as_str())?;
                    rhs.to_string_inner_min_parens(
                        &mut buf as &mut dyn std::fmt::Write,
                        Some(op.class()),
                    )?;
                    write!(buf, ")")?;
                } else {
                    lhs.to_string_inner_min_parens(
                        &mut buf as &mut dyn std::fmt::Write,
                        Some(op.class()),
                    )?;
                    write!(buf, " {} ", op.as_str())?;
                    rhs.to_string_inner_min_parens(
                        &mut buf as &mut dyn std::fmt::Write,
                        Some(op.class()),
                    )?;
                }
            }
        }
        Ok(())
    }

    fn to_string_inner(&self, mut buf: impl std::fmt::Write) -> std::fmt::Result {
        use std::fmt::Write;
        match self {
            Expr::FunctionCall { ident, args } => {
                write!(buf, "{ident}(")?;
                for arg in args.iter().take(args.len() - 1) {
                    arg.to_string_inner(&mut buf as &mut dyn std::fmt::Write)?;
                    write!(buf, ", ")?;
                }
                if let Some(arg) = args.last() {
                    arg.to_string_inner(&mut buf as &mut dyn std::fmt::Write)?;
                }
                write!(buf, ")")?;
            }
            Expr::RealNumber { val } => write!(buf, "({val})")?,
            Expr::ImaginaryNumber { val } => write!(buf, "({val}i)")?,
            Expr::ComplexNumber { val } => write!(buf, "({val})")?,
            Expr::Variable { name } => write!(buf, "{name}")?,
            Expr::Operator {
                op: op @ (Operator::UnaryMinus | Operator::UnaryPlus),
                rhs,
                ..
            } => {
                write!(buf, "({}", op.as_str())?;
                rhs.to_string_inner(&mut buf as &mut dyn std::fmt::Write)?;
                write!(buf, ")")?;
            }
            Expr::Operator { op, rhs, lhs } => {
                write!(buf, "(")?;
                lhs.to_string_inner(&mut buf as &mut dyn std::fmt::Write)?;
                write!(buf, " {} ", op.as_str())?;
                rhs.to_string_inner(&mut buf as &mut dyn std::fmt::Write)?;
                write!(buf, ")")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::token_stream::{token_stream_to_string, Token::*};
    use super::*;

    #[test]
    fn function_sep() {
        let input = "max(1, 5)";
        let stream = token_stream::parse_tokens(input, token_stream::RESTRICTED_WORD);
        let first_pass = function_pass(stream.peekable());

        let res: Result<Vec<_>, _> = first_pass.collect();

        assert_eq!(
            res.unwrap(),
            vec![
                Ident("max"),
                LeftParenthesis,
                Whitespace,
                Literal(1.0),
                Comma,
                Whitespace,
                Literal(5.0),
                RightParenthesis
            ]
        );
    }

    #[test]
    fn implicit_multiple() {
        let input = "a(1) + 1(1) + 1a + aa + (1)(1)1";
        let stream = token_stream::parse_tokens(input, token_stream::RESTRICTED_WORD);
        let first_pass = implicit_multiple_pass(stream.peekable());

        let iter = first_pass
            .flat_map(|token| [Ok(Whitespace), token].into_iter())
            .skip(1);
        let res = token_stream_to_string(iter);

        assert_eq!(
            res.unwrap(),
            "a * ( 1 ) + 1 * ( 1 ) + 1 * a + a * a + ( 1 ) * ( 1 ) * 1"
        );
    }

    #[test]
    fn unary() {
        let input = "-(-1) + -(+a)";

        let stream = token_stream::parse_tokens(input, token_stream::RESTRICTED_WORD);
        let iter = stream
            .flat_map(|token| [Ok(Whitespace), token].into_iter())
            .skip(1);
        let res = token_stream_to_string(iter);

        assert_eq!(res.unwrap(), "- ( - 1 ) + - ( + a )");
    }
    #[cfg(test)]
    mod ast {
        use super::super::Operator::{self as Operator_, *};
        use super::Bump;
        use super::Expr::{self, *};

        macro_rules! ast_test {
            ($name:ident: $input:literal $(=)?) => {
                #[test]
                fn $name() {
                    let arena = bumpalo::Bump::with_capacity(1024);
                    let expr = Expr::parse(&arena, $input, super::token_stream::RESTRICTED_WORD);

                    let expr = expr.unwrap();

                    dbg!(expr.to_string());
                    panic!();
                }
            };

            ($name:ident: $input:literal = $output:literal) => {
                #[test]
                fn $name() {
                    println!("{}", $input);
                    let arena = bumpalo::Bump::with_capacity(1024);
                    let expr = Expr::parse(&arena, $input, super::token_stream::RESTRICTED_WORD);

                    let expr = expr.unwrap();
                    println!("==================================================");

                    let same_expr =
                        Expr::parse(&arena, $output, super::token_stream::RESTRICTED_WORD);

                    let same_expr = same_expr.unwrap();

                    assert_eq!(expr.to_string(), $output);

                    assert_eq!(same_expr.to_string(), $output);
                }
            };
        }

        ast_test! {simple_addition: "1+1" = "1 + 1"}
        ast_test! {simple_substraction: "1-1" = "1 - 1"}
        ast_test! {simple_multiplication: "1*1" = "1 * 1"}
        ast_test! {simple_division: "1/1" = "1 / 1"}
        ast_test! {simple_modulo: "1%1" = "1 % 1"}
        ast_test! {simple_unary_minus: "--1" = "--1"}
        ast_test! {simple_unary_plus: "++1" = "++1"}

        ast_test! {mult1: "4 + 2 * 3" = "4 + 2 * 3"}
        ast_test! {implicit_multi1: "2a2" = "2 * a * 2"}

        ast_test! {complex1: "3 + 4 * 2 / (1 - 5) ^ 2 ^ 3" = "3 + 4 * 2 / (1 - 5) ^ 2 ^ 3"}

        ast_test! {function: "max(exp(7, 10), 3)" = "max(exp(7, 10), 3)"}
        ast_test! {function2: "max(2exp(7, 10), 3)" = "max(2 * exp(7, 10), 3)"}
        ast_test! {function3:
        "exp(exp(exp(exp(exp(exp(1), exp(1))) + 56, 2exp(exp(exp(exp(exp(1), exp(1))), exp(exp(exp(1), exp(exp(exp(1), exp(1))))))))), exp(exp(exp(exp(exp(exp(exp(5 + 7 + 54), exp(5 + 7 + 54))), exp(5 + 7 + 54))), exp(5 + 7 + 54))))" =
        "exp(exp(exp(exp(exp(exp(1), exp(1))) + 56, 2 * exp(exp(exp(exp(exp(1), exp(1))), exp(exp(exp(1), exp(exp(exp(1), exp(1))))))))), exp(exp(exp(exp(exp(exp(exp(5 + 7 + 54), exp(5 + 7 + 54))), exp(5 + 7 + 54))), exp(5 + 7 + 54))))"}
        ast_test! {function4: "max(1, 2, 4, 4, 5, 7, 30)" = "max(1, 2, 4, 4, 5, 7, 30)"}
    }
}
