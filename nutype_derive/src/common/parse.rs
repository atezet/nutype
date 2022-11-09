use std::{any::type_name, fmt::Debug, str::FromStr};

use proc_macro2::{Group, Ident, Span, TokenStream, TokenTree};

use crate::models::RawNewtypeMeta;

/// ## Example
/// Input (token stream):
///     = 123
/// Output (parsed value):
///    123
pub fn parse_value_as_number<T, ITER>(mut iter: ITER) -> Result<(T, ITER), syn::Error>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
    ITER: Iterator<Item = TokenTree>,
{
    let token_eq = iter.next().expect("Expected token `=`");
    assert_eq!(token_eq.to_string(), "=", "Expected token `=`");

    let (num_str, span) = read_number(&mut iter)?;

    let value: T = sanitize_number(&num_str).parse::<T>().map_err(|_err| {
        let msg = format!("Expected {}, got `{}`", type_name::<T>(), num_str);
        syn::Error::new(span, msg)
    })?;

    Ok((value, iter))
}

fn read_number<ITER>(iter: &mut ITER) -> Result<(String, Span), syn::Error>
where
    ITER: Iterator<Item = TokenTree>,
{
    let mut output = String::with_capacity(16);
    let mut token_value = iter.next().expect("Expected number");
    let mut span = token_value.span();
    let mut t = token_value.to_string();

    // If it starts with `-` (negative number), add it to output and parse the next token.
    if t == "-" {
        output.push_str(&t);
        token_value = iter.next().expect("Expected number");
        span = span.join(token_value.span()).unwrap();
        t = token_value.to_string();
    }

    output.push_str(&t);
    Ok((output, span))
}

fn sanitize_number(val: &str) -> String {
    val.replace('_', "")
}

pub fn try_unwrap_ident(token: TokenTree) -> Result<Ident, syn::Error> {
    match token {
        TokenTree::Ident(ident) => Ok(ident),
        _ => {
            let error = syn::Error::new(token.span(), "#[nutype] expected ident");
            Err(error)
        }
    }
}

pub fn try_unwrap_group(token: TokenTree) -> Result<Group, syn::Error> {
    match token {
        TokenTree::Group(group) => Ok(group),
        _ => {
            let error = syn::Error::new(token.span(), "#[nutype] expected ident");
            Err(error)
        }
    }
}

pub fn parse_nutype_attributes<S, V>(
    parse_sanitize_attrs: impl Fn(TokenStream) -> Result<Vec<S>, syn::Error>,
    parse_validate_attrs: impl Fn(TokenStream) -> Result<Vec<V>, syn::Error>,
) -> impl FnOnce(TokenStream) -> Result<RawNewtypeMeta<S, V>, syn::Error> {
    move |input: TokenStream| {
        let mut output = RawNewtypeMeta {
            sanitizers: vec![],
            validators: vec![],
        };

        let mut iter = input.into_iter();

        loop {
            let token = match iter.next() {
                Some(t) => t,
                None => {
                    return Ok(output);
                }
            };

            let ident = try_unwrap_ident(token)?;

            match ident.to_string().as_ref() {
                "sanitize" => {
                    let token = iter.next().unwrap();
                    let group = try_unwrap_group(token)?;

                    let sanitize_stream = group.stream();
                    output.sanitizers = parse_sanitize_attrs(sanitize_stream)?;
                }
                "validate" => {
                    let token = iter.next().unwrap();
                    let group = try_unwrap_group(token)?;
                    let validate_stream = group.stream();
                    output.validators = parse_validate_attrs(validate_stream)?;
                }
                unknown => {
                    let msg = format!("Unknown #[nutype] option: `{unknown}`");
                    let error = syn::Error::new(ident.span(), msg);
                    return Err(error);
                }
            }
        }
    }
}
