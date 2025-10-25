use proc_macro2::TokenStream;
use syn::parse::Parse;
use syn::{Token, parse2};

mod render;

#[derive(Debug, Clone)]
pub enum Method {
    Insert(Token![.], TokenStream),
    Replace(TokenStream, Option<Token![!]>),
}

// Result型を返す処理だが失敗するケースは考えにくい
impl Parse for Method {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let res = if input.peek(Token![.]) {
            // ドットは必ず存在する
            let dot_token: Token![.] = input.parse()?;
            // TokenStream への変換が失敗することは考えにくい
            let ts: TokenStream = input.parse()?;
            Self::Insert(dot_token, ts)
        } else {
            // TODO: 末尾の!もサポート
            // TokenStream への変換が失敗することは考えにくい
            Self::Replace(input.parse()?, None)
        };

        Ok(res)
    }
}

impl From<TokenStream> for Method {
    fn from(value: TokenStream) -> Self {
        // TokenStream から Method への変換
        // Result型を返す処理だが失敗するケースは考えにくいため expect でアンラップ
        parse2(value).expect("unexpected token stream for Method")
    }
}
