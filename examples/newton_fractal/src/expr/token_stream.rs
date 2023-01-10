pub const RESTRICTED_WORD: &[&str] = &[
    "min", "max", "sqrt", "cbrt", "log", "ln", "exp", "atan", "tan", "acos", "cos", "asin", "sin",
    "cosh", "sinh", "tanh",
];

enum IterOnceOrMultiple<T, Iter: Iterator<Item = T>> {
    Once(std::iter::Once<T>),
    Multiple(Iter),
}

impl<T, Iter: Iterator<Item = T>> Iterator for IterOnceOrMultiple<T, Iter> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Once(item) => item.next(),
            Self::Multiple(iter) => iter.next(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Token<'input> {
    Whitespace = Token::WHITESPACE,
    Literal(f64) = Token::LITERAL,
    Ident(&'input str) = Token::IDENT,
    Operator(super::Operator) = Token::OPERATOR,
    LeftParenthesis = Token::LEFT_PARENS,
    RightParenthesis = Token::RIGHT_PARENS,
    Comma = Token::COMMA,
}

pub enum IterEither<L, R> {
    Left(L),
    Right(R),
}

impl<T, L: Iterator<Item = T>, R: Iterator<Item = T>> Iterator for IterEither<L, R> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Left(i) => i.next(),
            Self::Right(i) => i.next(),
        }
    }
}

impl<'input> Token<'input> {
    pub const WHITESPACE: u8 = 0;
    pub const LITERAL: u8 = 1;
    pub const IDENT: u8 = 2;
    pub const OPERATOR: u8 = 3;
    pub const LEFT_PARENS: u8 = 4;
    pub const RIGHT_PARENS: u8 = 5;
    pub const COMMA: u8 = 6;

    fn from_str(
        input: &'input str,
        reserved_words: &[&str],
    ) -> Result<impl Iterator<Item = Token<'input>>, InvalidToken<'input>> {
        use IterOnceOrMultiple::*;

        match input {
            " " => return Ok(Once(std::iter::once(Token::Whitespace))),
            "," => return Ok(Once(std::iter::once(Token::Comma))),
            "(" => return Ok(Once(std::iter::once(Token::LeftParenthesis))),
            ")" => return Ok(Once(std::iter::once(Token::RightParenthesis))),
            _ => (),
        }

        if let Some(op) = super::Operator::from_str(input) {
            return Ok(Once(std::iter::once(Token::Operator(op))));
        }

        if let Ok(val) = str::parse::<f64>(input) {
            return Ok(Once(std::iter::once(Self::Literal(val))));
        }

        if input.is_ascii() {
            return Ok(Multiple(
                reserved_words
                    .iter()
                    .find_map(|&word| input.ends_with(word).then_some((input, word)))
                    .map(|(input, word)| input.split_at(input.bytes().len() - word.bytes().len()))
                    .map(|(idents, word)| {
                        idents
                            .split("")
                            .filter(|s| !s.is_empty())
                            .chain(std::iter::once(word))
                    })
                    .map(|i| i.map(Token::Ident))
                    .map(IterEither::Left)
                    .unwrap_or_else(|| {
                        IterEither::Right(
                            input.split("").filter(|s| !s.is_empty()).map(Token::Ident),
                        )
                    }),
            ));
        }

        Err(InvalidToken { span: Some(input) })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InvalidToken<'input> {
    pub span: Option<&'input str>,
}

impl<'input> std::fmt::Display for InvalidToken<'input> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An invalid token has been encountered")
    }
}

pub fn parse_tokens<'input, 'words: 'input, 'words_slice: 'words>(
    input: &'input str,
    reserved_words: &'words_slice [&'words str],
) -> impl Iterator<Item = Result<Token<'input>, InvalidToken<'input>>> {
    let mut token_type = Token::WHITESPACE;
    let mut chars_index = input.char_indices();
    let mut cur_chr = 0usize;
    let mut stop = false;
    std::iter::from_fn(move || {
        let Some((mut index, mut chr)) = chars_index.next() else {
            if !stop {
                stop = true;
                return Some(Token::from_str(&input[cur_chr..], reserved_words));
            } else {
                return None;
            }
        };
        while get_chr_token_type(chr) == token_type {
            token_type = get_chr_token_type(chr);
            if let Some((r_index, r_chr)) = chars_index.next() {
                index = r_index;
                chr = r_chr;
            } else {
                index = input.bytes().len();
                break;
            };
        }
        token_type = get_chr_token_type(chr);
        let s = &input[cur_chr..index];
        cur_chr = index;
        Some(Token::from_str(s, reserved_words))
    })
    .flat_map(swap)
    .filter_map(|t| match t {
        Ok(Token::Whitespace) => None,
        Ok(Token::Ident(" ")) => None,
        Ok(Token::Ident("(")) => Some(Ok(Token::LeftParenthesis)),
        Ok(Token::Ident(")")) => Some(Ok(Token::RightParenthesis)),
        Ok(Token::Ident("+")) => Some(Ok(Token::Operator(super::Operator::Plus))),
        Ok(Token::Ident("-")) => Some(Ok(Token::Operator(super::Operator::Minus))),
        Ok(Token::Ident("*")) => Some(Ok(Token::Operator(super::Operator::Multiply))),
        Ok(Token::Ident("/")) => Some(Ok(Token::Operator(super::Operator::Divide))),
        Ok(Token::Ident("%")) => Some(Ok(Token::Operator(super::Operator::Modulo))),
        Ok(Token::Ident("^")) => Some(Ok(Token::Operator(super::Operator::Pow))),
        Ok(Token::Ident(",")) => Some(Ok(Token::Comma)),
        t => Some(t),
    })
}

fn get_chr_token_type(chr: char) -> u8 {
    match chr {
        w if w.is_whitespace() => Token::WHITESPACE,
        '0'..='9' | '.' => Token::LITERAL,
        '(' => Token::LEFT_PARENS,
        ')' => Token::RIGHT_PARENS,
        ',' => Token::COMMA,
        '+' | '-' | '/' | '%' | '*' | '^' => Token::OPERATOR,
        a if a.is_ascii_alphabetic() && !a.is_whitespace() => Token::IDENT,
        _ => 255,
    }
}

#[allow(clippy::type_complexity)]
enum SwapResult<I, E>
where
    I: Iterator,
{
    Ok(core::iter::Map<I, fn(I::Item) -> Result<I::Item, E>>),
    Err(core::iter::Once<Result<I::Item, E>>),
}

impl<I, E> Iterator for SwapResult<I, E>
where
    I: Iterator,
{
    type Item = Result<I::Item, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SwapResult::Ok(m) => m.next(),
            SwapResult::Err(e) => e.next(),
        }
    }
}

fn swap<I: Iterator, E>(r: Result<I, E>) -> impl Iterator<Item = Result<I::Item, E>> {
    match r {
        Ok(i) => SwapResult::Ok(i.map(Ok)),
        Err(e) => SwapResult::Err(core::iter::once(Err(e))),
    }
}

pub fn token_stream_to_string<'input>(
    iter: impl Iterator<Item = Result<Token<'input>, InvalidToken<'input>>> + 'input,
) -> Result<String, InvalidToken<'input>> {
    fn iter_to_str<'input>(
        iter: impl Iterator<Item = Result<Token<'input>, InvalidToken<'input>>> + 'input,
    ) -> impl Iterator<Item = Result<std::borrow::Cow<'input, str>, InvalidToken<'input>>> + 'input
    {
        use std::borrow::Cow;
        iter.map(|token| {
            token.map(|t| match t {
                Token::Ident(a) => Cow::Borrowed(a),
                Token::Comma => Cow::Borrowed(","),
                Token::LeftParenthesis => Cow::Borrowed("("),
                Token::RightParenthesis => Cow::Borrowed(")"),
                Token::Literal(v) => Cow::Owned(v.to_string()),
                Token::Operator(o) => Cow::Borrowed(o.as_str()),
                Token::Whitespace => Cow::Borrowed(" "),
            })
        })
    }

    let mut out = String::new();
    for s in iter_to_str(iter) {
        let s = s?;
        out.push_str(&s);
    }
    Ok(out)
}

#[cfg(test)]
mod test {
    use super::{super::Operator, parse_tokens, Token, RESTRICTED_WORD};
    mod simple {
        use super::*;
        #[test]
        fn empty() {
            let input = "";
            let parsed = parse_tokens(input, RESTRICTED_WORD).collect::<Result<Vec<_>, _>>();

            assert!(parsed.is_ok());
            let parsed = parsed.unwrap();
            assert_eq!(parsed, vec![]);
        }
        #[test]
        fn addition() {
            let input = "1+1";
            let parsed = parse_tokens(input, RESTRICTED_WORD).collect::<Result<Vec<_>, _>>();

            assert!(parsed.is_ok());
            let parsed = parsed.unwrap();
            assert_eq!(
                parsed,
                vec![
                    Token::Literal(1.0),
                    Token::Operator(Operator::Plus),
                    Token::Literal(1.0),
                ]
            );
        }
        #[test]
        fn substruction() {
            let input = "1-1";
            let parsed = parse_tokens(input, RESTRICTED_WORD).collect::<Result<Vec<_>, _>>();

            assert!(parsed.is_ok());
            let parsed = parsed.unwrap();
            assert_eq!(
                parsed,
                vec![
                    Token::Literal(1.0),
                    Token::Operator(Operator::Minus),
                    Token::Literal(1.0),
                ]
            );
        }
        #[test]
        fn multiplication() {
            let input = "1*1";
            let parsed = parse_tokens(input, RESTRICTED_WORD).collect::<Result<Vec<_>, _>>();

            assert!(parsed.is_ok());
            let parsed = parsed.unwrap();
            assert_eq!(
                parsed,
                vec![
                    Token::Literal(1.0),
                    Token::Operator(Operator::Multiply),
                    Token::Literal(1.0),
                ]
            );
        }
        #[test]
        fn division() {
            let input = "1/1";
            let parsed = parse_tokens(input, RESTRICTED_WORD).collect::<Result<Vec<_>, _>>();

            assert!(parsed.is_ok());
            let parsed = parsed.unwrap();
            assert_eq!(
                parsed,
                vec![
                    Token::Literal(1.0),
                    Token::Operator(Operator::Divide),
                    Token::Literal(1.0),
                ]
            );
        }
        #[test]
        fn modulo() {
            let input = "1%1";
            let parsed = parse_tokens(input, RESTRICTED_WORD).collect::<Result<Vec<_>, _>>();

            assert!(parsed.is_ok());
            let parsed = parsed.unwrap();
            assert_eq!(
                parsed,
                vec![
                    Token::Literal(1.0),
                    Token::Operator(Operator::Modulo),
                    Token::Literal(1.0),
                ]
            );
        }
    }
    mod simple_parenthesis {
        use super::*;
        #[test]
        fn empty() {
            let input = "()";
            let parsed = parse_tokens(input, RESTRICTED_WORD).collect::<Result<Vec<_>, _>>();

            assert!(parsed.is_ok());
            let parsed = parsed.unwrap();
            assert_eq!(
                parsed,
                vec![
                    Token::LeftParenthesis,
                    //
                    Token::RightParenthesis,
                ]
            );
        }
        #[test]
        fn addition() {
            let input = "(1+1)";
            let parsed = parse_tokens(input, RESTRICTED_WORD).collect::<Result<Vec<_>, _>>();

            assert!(parsed.is_ok());
            let parsed = parsed.unwrap();
            assert_eq!(
                parsed,
                vec![
                    Token::LeftParenthesis,
                    Token::Literal(1.0),
                    Token::Operator(Operator::Plus),
                    Token::Literal(1.0),
                    Token::RightParenthesis,
                ]
            );
        }
        #[test]
        fn substruction() {
            let input = "(1-1)";
            let parsed = parse_tokens(input, RESTRICTED_WORD).collect::<Result<Vec<_>, _>>();

            assert!(parsed.is_ok());
            let parsed = parsed.unwrap();
            assert_eq!(
                parsed,
                vec![
                    Token::LeftParenthesis,
                    Token::Literal(1.0),
                    Token::Operator(Operator::Minus),
                    Token::Literal(1.0),
                    Token::RightParenthesis,
                ]
            );
        }
        #[test]
        fn multiplication() {
            let input = "(1*1)";
            let parsed = parse_tokens(input, RESTRICTED_WORD).collect::<Result<Vec<_>, _>>();

            assert!(parsed.is_ok());
            let parsed = parsed.unwrap();
            assert_eq!(
                parsed,
                vec![
                    Token::LeftParenthesis,
                    Token::Literal(1.0),
                    Token::Operator(Operator::Multiply),
                    Token::Literal(1.0),
                    Token::RightParenthesis,
                ]
            );
        }
        #[test]
        fn division() {
            let input = "(1/1)";
            let parsed = parse_tokens(input, RESTRICTED_WORD).collect::<Result<Vec<_>, _>>();

            assert!(parsed.is_ok());
            let parsed = parsed.unwrap();
            assert_eq!(
                parsed,
                vec![
                    Token::LeftParenthesis,
                    Token::Literal(1.0),
                    Token::Operator(Operator::Divide),
                    Token::Literal(1.0),
                    Token::RightParenthesis,
                ]
            );
        }
        #[test]
        fn modulo() {
            let input = "(1%1)";
            let parsed = parse_tokens(input, RESTRICTED_WORD).collect::<Result<Vec<_>, _>>();

            assert!(parsed.is_ok());
            let parsed = parsed.unwrap();
            assert_eq!(
                parsed,
                vec![
                    Token::LeftParenthesis,
                    Token::Literal(1.0),
                    Token::Operator(Operator::Modulo),
                    Token::Literal(1.0),
                    Token::RightParenthesis,
                ]
            );
        }
    }
    mod more_parenthesis {
        use super::*;
        #[test]
        fn addition() {
            let input = "(1)+(1)";
            let parsed = parse_tokens(input, RESTRICTED_WORD).collect::<Result<Vec<_>, _>>();

            assert!(parsed.is_ok());
            let parsed = parsed.unwrap();
            assert_eq!(
                parsed,
                vec![
                    Token::LeftParenthesis,
                    Token::Literal(1.0),
                    Token::RightParenthesis,
                    Token::Operator(Operator::Plus),
                    Token::LeftParenthesis,
                    Token::Literal(1.0),
                    Token::RightParenthesis,
                ]
            );
        }
        #[test]
        fn substruction() {
            let input = "(1)-(1)";
            let parsed = parse_tokens(input, RESTRICTED_WORD).collect::<Result<Vec<_>, _>>();

            assert!(parsed.is_ok());
            let parsed = parsed.unwrap();
            assert_eq!(
                parsed,
                vec![
                    Token::LeftParenthesis,
                    Token::Literal(1.0),
                    Token::RightParenthesis,
                    Token::Operator(Operator::Minus),
                    Token::LeftParenthesis,
                    Token::Literal(1.0),
                    Token::RightParenthesis,
                ]
            );
        }
        #[test]
        fn multiplication() {
            let input = "(1)*(1)";
            let parsed = parse_tokens(input, RESTRICTED_WORD).collect::<Result<Vec<_>, _>>();

            assert!(parsed.is_ok());
            let parsed = parsed.unwrap();
            assert_eq!(
                parsed,
                vec![
                    Token::LeftParenthesis,
                    Token::Literal(1.0),
                    Token::RightParenthesis,
                    Token::Operator(Operator::Multiply),
                    Token::LeftParenthesis,
                    Token::Literal(1.0),
                    Token::RightParenthesis,
                ]
            );
        }
        #[test]
        fn division() {
            let input = "(1)/(1)";
            let parsed = parse_tokens(input, RESTRICTED_WORD).collect::<Result<Vec<_>, _>>();

            assert!(parsed.is_ok());
            let parsed = parsed.unwrap();
            assert_eq!(
                parsed,
                vec![
                    Token::LeftParenthesis,
                    Token::Literal(1.0),
                    Token::RightParenthesis,
                    Token::Operator(Operator::Divide),
                    Token::LeftParenthesis,
                    Token::Literal(1.0),
                    Token::RightParenthesis,
                ]
            );
        }
        #[test]
        fn modulo() {
            let input = "(1)%(1)";
            let parsed = parse_tokens(input, RESTRICTED_WORD).collect::<Result<Vec<_>, _>>();

            assert!(parsed.is_ok());
            let parsed = parsed.unwrap();
            assert_eq!(
                parsed,
                vec![
                    Token::LeftParenthesis,
                    Token::Literal(1.0),
                    Token::RightParenthesis,
                    Token::Operator(Operator::Modulo),
                    Token::LeftParenthesis,
                    Token::Literal(1.0),
                    Token::RightParenthesis,
                ]
            );
        }
    }

    mod idents {
        use super::*;
        mod reserved_words {
            use super::*;
            #[test]
            fn reserved_words() {
                RESTRICTED_WORD.iter().for_each(|word| {
                    let input = format!("randomword{word}");
                    let parsed =
                        parse_tokens(&input, RESTRICTED_WORD).collect::<Result<Vec<_>, _>>();

                    assert!(parsed.is_ok());
                    let parsed = parsed.unwrap();
                    assert_eq!(
                        parsed,
                        vec![
                            Token::Ident("r"),
                            Token::Ident("a"),
                            Token::Ident("n"),
                            Token::Ident("d"),
                            Token::Ident("o"),
                            Token::Ident("m"),
                            Token::Ident("w"),
                            Token::Ident("o"),
                            Token::Ident("r"),
                            Token::Ident("d"),
                            Token::Ident(word),
                        ]
                    );
                });
            }
        }
    }
}
