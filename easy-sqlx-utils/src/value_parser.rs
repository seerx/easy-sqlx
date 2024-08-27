use std::ops::Deref;

use proc_macro2::{Group, Punct, Span};
use quote::{ToTokens, TokenStreamExt};
use syn::{
    parenthesized, parse::{Parse, ParseBuffer, ParseStream}, punctuated::Punctuated, token::Comma, Error, Lit, Token
};

pub fn parse_next<T: FnOnce() -> Result<R, syn::Error>, R: Sized>(
    input: ParseStream,
    next: T,
) -> Result<R, syn::Error> {
    input.parse::<Token![=]>()?;
    next()
}

pub fn parse_groups<T, R>(input: ParseStream) -> syn::Result<R>
where
    T: Sized,
    T: Parse,
    R: FromIterator<T>,
{
    Punctuated::<Group, Comma>::parse_terminated(input).and_then(|groups| {
        groups
            .into_iter()
            .map(|group| syn::parse2::<T>(group.stream()))
            .collect::<syn::Result<R>>()
    })
}

pub fn parse_punctuated_within_parenthesis<T>(
    input: ParseStream,
) -> syn::Result<Punctuated<T, Comma>>
where
    T: Parse,
{
    let content;
    parenthesized!(content in input);
    Punctuated::<T, Comma>::parse_terminated(&content)
}

/// Tokenizes slice or Vec of tokenizable items as array either with reference (`&[...]`)
/// or without correctly to OpenAPI JSON.
#[derive(Debug)]
pub enum Array<'a, T>
where
    T: Sized + ToTokens,
{
    Owned(Vec<T>),
    #[allow(dead_code)]
    Borrowed(&'a [T]),
}

impl<T> Array<'_, T> where T: ToTokens + Sized {}

impl<V> FromIterator<V> for Array<'_, V>
where
    V: Sized + ToTokens,
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        Self::Owned(iter.into_iter().collect())
    }
}

impl<'a, T> Deref for Array<'a, T>
where
    T: Sized + ToTokens,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Owned(vec) => vec.as_slice(),
            Self::Borrowed(slice) => slice,
        }
    }
}

impl<T> ToTokens for Array<'_, T>
where
    T: Sized + ToTokens,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let values = match self {
            Self::Owned(values) => values.iter(),
            Self::Borrowed(values) => values.iter(),
        };
        tokens.append(Group::new(
            proc_macro2::Delimiter::Bracket,
            values
                .fold(Punctuated::new(), |mut punctuated, item| {
                    punctuated.push_value(item);
                    punctuated.push_punct(Punct::new(',', proc_macro2::Spacing::Alone));

                    punctuated
                })
                .to_token_stream(),
        ));

        // tokens.append();
    }
}

// column.col_name = info_stream
//     .parse::<syn::Lit>()
//     .map(|a| {
//         match a {
//             Lit::Str(str) => {
//                 return Some(str.value());
//             }
//             _ => {
//                 // return Err(Error::new(ident.span(), EXPECTED_ATTRIBUTE));
//             }
//         }
//         Some("".to_string())
//         // column.name = a.span()
//     })?;
pub fn parse_string(
    stream: ParseBuffer,
    field: &'static str,
    attr: &'static str,
) -> syn::Result<String> {
    stream
        .parse::<syn::Lit>()
        .map(|lit| match lit {
            Lit::Str(val) => {
                return Ok(val.value());
            }
            _ => {
                return Err(Error::new(
                    Span::call_site(),
                    format!("field {}'s attr '{}' expect String", field, attr),
                ));
            }
        })
        .map_err(|err| {
            return Error::new(
                Span::call_site(),
                format!("field {}'s attr '{}' parse error: {}", field, attr, err),
            );
        })
        .unwrap()
}
