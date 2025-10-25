use proc_macro2::TokenStream;
use syn::parse::Parse;
use syn::{Token, parse2};

mod render;

#[derive(Debug, Clone)]
pub enum Method {
    Insert(Token![.], TokenStream),
    Replace(TokenStream),
}

// Result型を返す処理だが失敗するケースは考えにくい
// エラーが起きないと言える根拠は各 ? がある処理の直前にコメントを掲載した
impl Parse for Method {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let res = if input.peek(Token![.]) {
            // ドットは必ず存在する
            let dot_token: Token![.] = input.parse()?;
            // TokenStream への変換が失敗することは考えにくい
            let ts: TokenStream = input.parse()?;

            Self::Insert(dot_token, ts)
        } else {
            // TokenStream への変換が失敗することは考えにくい
            Self::Replace(input.parse()?)
        };

        Ok(res)
    }
}

impl From<TokenStream> for Method {
    fn from(value: TokenStream) -> Self {
        // TokenStream から Method への変換
        // Result型を返す処理だが失敗するケースは考えにくいため expect でアンラップ
        parse2(value).unwrap_or_else(|e| {
            panic!(
                "unexpected token stream for Method: {}

This error is unexpected for the developers.
If possible, please post an issue in the repository.
link: `https://github.com/anotherhollow1125/hooq`
",
                e
            )
        })
    }
}
