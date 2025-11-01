use proc_macro2::TokenStream;
use syn::parse::Parse;
use syn::{Token, parse2};

mod render;

#[derive(Debug, Clone)]
pub enum Method {
    Insert(Token![.], TokenStream),
    Replace(TokenStream),
}

impl Parse for Method {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let res = if input.peek(Token![.]) {
            let dot_token: Token![.] = input.parse()?;
            let ts: TokenStream = input.parse()?;

            Self::Insert(dot_token, ts)
        } else {
            Self::Replace(input.parse()?)
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
