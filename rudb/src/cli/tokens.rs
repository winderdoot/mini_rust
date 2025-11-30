/* Parsing helpers */

/* This is a bit dumb but I don't have time to think about how to better design parsing.
 * Actually I should have made a Token trait and implemented it for str&. You could chain methods
 * on the token trait and have readable syntax for what is supposed to follow the token and stuff.
 * Damn... Pest was the way to go after all...
 */

use std::iter::Peekable;

use crate::cli::errors::ParseErr;

pub fn split_and_keep<'a>(text: &'a str, sep: &'a str, skip: bool) -> Vec<(&'a str, bool)> {
    if skip {
        return vec![(text, true)]
    }

    let mut parts = Vec::new();
    let mut last_end = 0;

    for (start, matched) in text.match_indices(sep) {
        if start > last_end {
            parts.push((&text[last_end..start], false));
        }
        parts.push((matched, true));
        last_end = start + matched.len();
    }

    if last_end < text.len() {
        parts.push((&text[last_end..], false));
    }

    parts.into_iter().filter(|(s, skip)| !s.is_empty()).collect()
}

pub fn token_stream<'a, I: Iterator<Item = &'a str>>(string: &'a str) -> Peekable<impl Iterator<Item = &'a str>> 
{
    string
        .split_whitespace()
        .flat_map(|tok| split_and_keep(tok, ",", false))
        .flat_map(|(tok, skip)| split_and_keep(tok, ":", skip))
        .flat_map(|(tok, skip)| split_and_keep(tok, "<=", skip))
        .flat_map(|(tok, skip)| split_and_keep(tok, ">=", skip))
        .flat_map(|(tok, skip)| split_and_keep(tok, "!=", skip))
        .flat_map(|(tok, skip)| split_and_keep(tok, "=", skip))
        .flat_map(|(tok, skip)| split_and_keep(tok, "<", skip))
        .flat_map(|(tok, skip)| split_and_keep(tok, ">", skip))
        .map(|(tok, _)| tok)
        .peekable()
}

pub fn matches_charset<'a>(token: &'a str, charset: &str) -> Result<&'a str, ParseErr> {
    match token.chars().find(|c| !charset.contains(*c)) {
        Some(c) => Err(ParseErr::FieldInvalidChar(c)),
        None => Ok(token),
    }
}

/// Advances the iterator, expecting a certain token
pub fn next_token<'a, I>(iter: &mut I, prev: &str, expect: &str) -> Result<I::Item, ParseErr>
where 
    I: Iterator<Item = &'a str> 
{
    iter
        .next()
        .ok_or(ParseErr::MissingToken { prev: prev.to_string(), missing: expect.to_string() })
}


/// Advances the iterator and checks if next token matches a value
pub fn expect_token<'a, I>(iter: &mut I, prev: &str, expect: &str) -> Result<I::Item, ParseErr>
where 
    I: Iterator<Item = &'a str>
{
    iter
        .next()
        .ok_or_else(|| ParseErr::MissingToken { prev: prev.to_string(), missing: expect.to_string() })
        .and_then(|tok| {
            if tok.eq_ignore_ascii_case(expect) {
                return Ok(tok);
            }
            Err(ParseErr::MissingToken { prev: prev.to_string(), missing: expect.to_string() })
        })
}

/// Advances the iterator, expecting a token and a trailing separator, that could also be the next token afterwads.
/// In the latter case, consumes the separator token as well.
pub fn token_separator<'a, I>(iter: &mut Peekable<I>, expect: &str, sep: &str) -> Result<I::Item, ParseErr>
where
    I: Iterator<Item = &'a str>
{
    iter
        .next()
        .ok_or(ParseErr::ExpectedToken(format!("{expect}{sep}")))
        .and_then(|tok| {
            if tok.ends_with(sep) {
                return Ok(tok.trim_end_matches(sep));
            }
            if iter.peek().is_some_and(|next| *next == sep) {
                iter.next();
                return Ok(tok);
            }
            Err(ParseErr::ExpectedToken(format!("{expect}{sep}")))
        })
}

pub fn token_maybe_separator<'a, I>(iter: &mut Peekable<I>, expect: &str, sep: &str, found_sep: &mut bool) -> Result<I::Item, ParseErr>
where
    I: Iterator<Item = &'a str>
{
    iter
        .next()
        .ok_or(ParseErr::ExpectedToken(expect.to_string()))
        .and_then(|tok| {
            if tok.ends_with(sep) {
                *found_sep = true;
                return Ok(tok.trim_end_matches(sep));
            }
            if iter.peek().is_some_and(|next| *next == sep) {
                iter.next();
                *found_sep = true;
                return Ok(tok);
            }
            Ok(tok)
        })
}

// Previous separator functions allowed for the separator to be a postfix of the next token. This one expects it to be the next token, which it consumes
pub fn token_any_separator<'a, I>(iter: &mut Peekable<I>, expect: &str, separators: &[&'a str], found_sep: &mut &'a str) -> Result<I::Item, ParseErr>
where
    I: Iterator<Item = &'a str>
{
    Ok(
        iter
        .next()
        .ok_or(ParseErr::ExpectedToken(format!("{expect}[{}]", separators.join(", "))))
        .map(|tok| {
            let sep = *separators
            .iter()
            .find(|sep| {
                iter
                    .peek()
                    .is_some_and(|next| *next == **sep)
            })
            .ok_or_else(|| ParseErr::ExpectedToken(format!("{expect}[{}]", separators.join(", "))))?;
            *found_sep = sep;
            iter.next(); // Consume the separator
            
            Ok(tok)
        })
        .flatten()?
    )
}

pub fn expect_empty<'a, I>(iter: &mut Peekable<I>, expect: &str) -> Result<(), ParseErr>
where
    I: Iterator<Item = &'a str>
{
    match iter.next() {
        Some(tok) => Err(ParseErr::WrongToken { expected: expect.to_string(), got: tok.to_string() }),
        None => Ok(()),
    }
}

pub fn token_or_empty<'a, I>(iter: &mut Peekable<I>, prev: &str, expect: &str, empty: &mut bool) -> Result<Option<I::Item>, ParseErr>
where 
    I: Iterator<Item = &'a str> 
{
    match iter.peek() {
        Some(tok) => {
            *empty = false;
            if tok.eq_ignore_ascii_case(expect) {
                return Ok(iter.next());
            }
            else {
                return Err(ParseErr::MissingToken { prev: prev.to_string(), missing: expect.to_string() });
            }
        },
        None => {
            *empty = true;
            Ok(None)
        },
    }
}