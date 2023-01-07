mod token_stream;
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
        name: char,
    },
    FunctionCall {
        ident: bumpalo::collections::String<'arena>,
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
            Variable { name } => Variable { name: *name },
            FunctionCall { ident, args } => FunctionCall {
                ident: unsafe {
                    bumpalo::collections::String::from_utf8_unchecked(
                        bumpalo::collections::FromIteratorIn::from_iter_in(ident.bytes(), arena),
                    )
                },
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
enum Operator {
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
                    token_stream::Token::LeftParenthesis | token_stream::Token::Ident(_),
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
    let next = iter.peek_mut().map(|next| match next {
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
        match next {
            Some(Ok(
                token_stream::Token::Operator(_)
                | token_stream::Token::Comma
                | token_stream::Token::Whitespace
                | token_stream::Token::LeftParenthesis,
            )) => match iter.peek_mut() {
                Some(Ok(token_stream::Token::Operator(op @ Operator::Minus))) => {
                    *op = Operator::UnaryMinus
                }
                Some(Ok(token_stream::Token::Operator(op @ Operator::Plus))) => {
                    *op = Operator::UnaryPlus
                }
                _ => (),
            },
            _ => (),
        }
        next
    })
}

pub use token_stream::InvalidToken;

impl<'arena> Expr<'arena> {
    pub fn parse<'input>(
        arena: &'arena bumpalo::Bump,
        input: &'input str,
        reserved_words: &[&str],
    ) -> Result<&'arena mut Expr<'arena>, token_stream::InvalidToken<'input>> {
        let iter = token_stream::parse_tokens(input, reserved_words);
        let iter = function_pass(iter.peekable());
        let iter = implicit_multiple_pass(iter.peekable());
        let iter = unary_pass(iter.peekable());

        todo!()
    }

    fn shunting_yard<'input>(
        arena: &'arena bumpalo::Bump,
        iter: impl Iterator<Item = Result<token_stream::Token<'input>, InvalidToken<'input>>>,
    ) -> Result<&'arena mut Expr<'arena>, token_stream::InvalidToken<'input>> {
        todo!()
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

        assert!(res.is_ok());

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
        let input = "a(1) + 1(1) + 1a + aa + (1)(1)";
        let stream = token_stream::parse_tokens(input, token_stream::RESTRICTED_WORD);
        let first_pass = implicit_multiple_pass(stream.peekable());

        let iter = first_pass
            .flat_map(|token| [Ok(Whitespace), token].into_iter())
            .skip(1);
        let res = token_stream_to_string(iter);
        assert!(res.is_ok());

        assert_eq!(
            res.unwrap(),
            "a * ( 1 ) + 1 * ( 1 ) + 1 * a + a * a + ( 1 ) * ( 1 )"
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
        assert!(res.is_ok());

        assert_eq!(res.unwrap(), "- ( - 1 ) + - ( + a )");
    }
}
