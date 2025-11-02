use proc_macro2::TokenStream;
#[cfg(feature = "consume-question")]
use proc_macro2::TokenTree;
use syn::parse::Parse;
use syn::{Token, parse2};

mod render;

#[derive(Debug, Clone)]
pub enum Method {
    Insert(Token![.], TokenStream, Option<Token![!]>),
    Replace(TokenStream, Option<Token![!]>),
}

#[cfg(not(feature = "consume-question"))]
impl Parse for Method {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let res = if input.peek(Token![.]) {
            let dot_token: Token![.] = input.parse()?;
            let ts: TokenStream = input.parse()?;

            Self::Insert(dot_token, ts, None)
        } else {
            Self::Replace(input.parse()?, None)
        };

        Ok(res)
    }
}

#[cfg(feature = "consume-question")]
impl Parse for Method {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let res = if input.peek(Token![.]) {
            let dot_token: Token![.] = input.parse()?;
            let ts: TokenStream = input.parse()?;

            let (ts, exc) = split_tail_exclamation(ts);

            Self::Insert(dot_token, ts, exc)
        } else {
            let (ts, exc) = split_tail_exclamation(input.parse()?);

            Self::Replace(ts, exc)
        };

        Ok(res)
    }
}

impl TryFrom<TokenStream> for Method {
    type Error = syn::Error;

    fn try_from(value: TokenStream) -> Result<Self, Self::Error> {
        parse2(value)
    }
}

#[cfg(feature = "consume-question")]
fn split_tail_exclamation(ts: TokenStream) -> (TokenStream, Option<Token![!]>) {
    let mut tokens = ts.clone().into_iter().collect::<Vec<_>>();

    let is_exc = matches!(
        tokens.last(),
        Some(TokenTree::Punct(punct)) if punct.as_char() == '!'
    );

    if !is_exc {
        return (ts, None);
    }

    let exc = tokens.pop().expect("just checked");
    let exc: Token![!] = Token![!](exc.span());
    let new_ts = TokenStream::from_iter(tokens);
    (new_ts, Some(exc))
}
