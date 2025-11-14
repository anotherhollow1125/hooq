use proc_macro2::{Delimiter, Span, TokenStream};
use quote::ToTokens;
use syn::Expr;
use syn::fold::{Fold, fold_lit_str};
use syn::spanned::Spanned;

use crate::impls::inert_attr::context::HookTargetKind;

// --- primitives start ---

/// Truncate string literals in the given expression.
/// eg. "01234567891234" -> "0123..1234"
/// Truncation is done line by line.
pub fn truncate_lit_str(ts: TokenStream, max_len: Option<usize>) -> syn::Result<TokenStream> {
    let expr: Expr = syn::parse2(ts)?;
    let expr = truncate_lit_str_inner(expr, max_len);
    Ok(expr.into_token_stream())
}

/// Convert the given expression into a pretty-printed string representation.
pub fn into_pretty_str(
    expr: Expr,
    PrettyStrSettings {
        with_line,
        top_padding,
        bottom_padding,
    }: PrettyStrSettings,
) -> String {
    let mut current_line: Option<usize> = None;
    let mut current_column: Option<usize> = None;

    let start_line = expr.span().start().line;
    let end_line = expr.span().end().line;

    let res = into_pretty_str_inner(
        expr.into_token_stream(),
        &mut current_line,
        &mut current_column,
        true,
    );

    if with_line {
        add_line_prefix(&res, start_line, end_line, top_padding, bottom_padding)
    } else {
        format!(
            "{}{res}{}",
            if top_padding { "\n" } else { "" },
            if bottom_padding { "\n" } else { "" }
        )
    }
}

// --- primitives end ---

#[derive(Clone, Copy)]
pub enum TruncateLitStrSetting {
    NoTruncation,
    Truncate(Option<usize>),
}

impl TruncateLitStrSetting {
    pub fn apply(self, expr: Expr) -> Expr {
        match self {
            TruncateLitStrSetting::NoTruncation => expr,
            TruncateLitStrSetting::Truncate(max_len) => truncate_lit_str_inner(expr, max_len),
        }
    }
}

#[derive(Clone, Copy)]
pub struct PrettyStrSettings {
    pub with_line: bool,
    pub top_padding: bool,
    pub bottom_padding: bool,
}

#[derive(Clone, Copy)]
pub struct ExcerptSetting {
    pub max_excerpted_line_num: usize,
    pub truncate_lit_str_setting: TruncateLitStrSetting,
    pub pretty_str_settings: PrettyStrSettings,
}

impl Default for ExcerptSetting {
    fn default() -> Self {
        Self {
            max_excerpted_line_num: 3,
            truncate_lit_str_setting: TruncateLitStrSetting::Truncate(None),
            pretty_str_settings: PrettyStrSettings {
                with_line: true,
                top_padding: false,
                bottom_padding: true,
            },
        }
    }
}

/// Convert the given token stream into a one-line pretty-printed string representation.
/// lit_str_max_len: If specified, string literals will be truncated to this length.
pub fn into_one_line_pretty_str(
    ts: TokenStream,
    truncate_lit_str_setting: TruncateLitStrSetting,
) -> syn::Result<String> {
    let expr: Expr = syn::parse2(ts)?;
    let expr = truncate_lit_str_setting.apply(expr);
    let expr_str = into_pretty_str(
        expr,
        PrettyStrSettings {
            with_line: false,
            top_padding: false,
            bottom_padding: false,
        },
    )
    .lines()
    .map(|line| line.trim())
    .collect::<Vec<&str>>()
    .join(" ");

    Ok(expr_str)
}

pub fn into_excerpted_line_pretty_str(
    ts: TokenStream,
    ExcerptSetting {
        max_excerpted_line_num,
        truncate_lit_str_setting,
        pretty_str_settings,
    }: ExcerptSetting,
) -> syn::Result<String> {
    if max_excerpted_line_num < 1 {
        return Err(syn::Error::new(
            Span::call_site(),
            "excerpted_line_num must be greater than or equal to 1",
        ));
    }

    // 欲しい行が1行の場合はもとに無関係に1行取る
    if max_excerpted_line_num == 1 {
        let expr = syn::parse2::<Expr>(ts)?;
        let expr = truncate_lit_str_setting.apply(expr);
        let expr_str = into_pretty_str(expr, pretty_str_settings);
        let expr_str = expr_str.lines().last().unwrap_or("");

        return Ok(expr_str.to_string());
    }

    // 以降は欲しい行が2行以上の場合
    // 後続のため、予めターゲット種別を調べておく
    let expr = syn::parse2::<Expr>(ts)?;
    let kind = match expr {
        Expr::Try(_) => HookTargetKind::Question,
        Expr::Return(_) => HookTargetKind::Return,
        _ => HookTargetKind::TailExpr,
    };
    let expr = truncate_lit_str_setting.apply(expr);
    let expr_str = into_pretty_str(expr, pretty_str_settings);

    // 対象の行が2行で収まる場合はすべて取る
    if expr_str.lines().count() <= 2 {
        return Ok(expr_str);
    }

    // 対象の行が3行以上の場合は、省略を行う
    match kind {
        // Question, TailExpr の場合は最終行に近い行から順に取る
        HookTargetKind::Question | HookTargetKind::TailExpr => {
            let expr_str = expr_str
                .lines()
                .rev()
                .take(max_excerpted_line_num)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<&str>>()
                .join("\n");

            Ok(format!("...\n{}", expr_str))
        }
        // Return の場合は先頭行と、最終行に近い行から順に取る
        HookTargetKind::Return => {
            let mut lines_iter = expr_str.lines();

            let first_line = lines_iter.next().unwrap_or("");

            let mut res = String::new();
            res.push_str(first_line);
            res.push('\n');

            let tail_lines = lines_iter
                .rev()
                .take(max_excerpted_line_num - 1)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<&str>>()
                .join("\n");

            res.push_str("...\n");
            res.push_str(&tail_lines);

            Ok(res)
        }
    }
}

struct TruncateStrFolder {
    max_len: usize,
    start_len: usize,
    end_len: usize,
}

impl Default for TruncateStrFolder {
    fn default() -> Self {
        TruncateStrFolder {
            max_len: 10,
            start_len: 4,
            end_len: 4,
        }
    }
}

impl TruncateStrFolder {
    fn new(max_len: usize) -> Self {
        if max_len < 2 {
            return TruncateStrFolder {
                max_len,
                start_len: 0,
                end_len: 0,
            };
        }

        let rest_len = max_len - 2;
        let start_len = rest_len / 2;
        let end_len = rest_len - start_len;

        TruncateStrFolder {
            max_len,
            start_len,
            end_len,
        }
    }
}

impl Fold for TruncateStrFolder {
    fn fold_lit_str(&mut self, i: syn::LitStr) -> syn::LitStr {
        let value = i.value();
        let mut truncated_value = Vec::new();

        // NOTE:
        // `lines` を使いたいが、最後に `\n` がある場合削除されてしまう
        // そのため、 `split('\n')` を使う
        // キャリッジリターンが含まれていたとしても、
        // 最後の方でも `\n` でつないでいるので大丈夫なはず
        for line in value.split('\n') {
            let truncated_line = if line.chars().count() > self.max_len {
                let start: String = line.chars().take(self.start_len).collect();
                let end: String = line
                    .chars()
                    .rev()
                    .take(self.end_len)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect();
                format!("{}..{}", start, end)
            } else {
                line.to_string()
            };
            truncated_value.push(truncated_line);
        }
        let i = syn::LitStr::new(&truncated_value.join("\n"), i.span());
        fold_lit_str(self, i)
    }
}

fn truncate_lit_str_inner(expr: Expr, max_len: Option<usize>) -> Expr {
    let mut folder = max_len.map_or(TruncateStrFolder::default(), TruncateStrFolder::new);
    folder.fold_expr(expr)
}

fn into_pretty_str_inner(
    input: TokenStream,
    current_line: &mut Option<usize>,
    current_column: &mut Option<usize>,
    mut is_top: bool,
) -> String {
    let mut res = String::new();

    fn add_tt(
        res: &mut String,
        tt_str: &str,
        tt_span: Span,
        current_line: &mut Option<usize>,
        current_column: &mut Option<usize>,
        is_top: bool,
    ) {
        let span = tt_span.unwrap();

        if is_top {
            let indent_num = span.column().saturating_sub(1);
            res.push_str(" ".repeat(indent_num).as_str());
        }

        if let Some(line) = current_line
            && span.line() > *line
        {
            res.push('\n');
            res.push_str(" ".repeat(span.column().saturating_sub(1)).as_str());
        } else if let Some(column) = current_column
            && span.column() > *column
        {
            res.push_str(" ".repeat(span.column().saturating_sub(*column)).as_str());
        }

        res.push_str(tt_str);

        *current_line = Some(span.line());
        *current_column = Some(span.end().column());
    }

    for tt in input {
        match tt {
            proc_macro2::TokenTree::Ident(_)
            | proc_macro2::TokenTree::Punct(_)
            | proc_macro2::TokenTree::Literal(_) => {
                let tt_str = tt.to_string();
                let tt_span = tt.span();

                add_tt(
                    &mut res,
                    &tt_str,
                    tt_span,
                    current_line,
                    current_column,
                    is_top,
                );
                is_top = false;
            }
            proc_macro2::TokenTree::Group(group) => {
                let start_delim = delim_start(group.delimiter());
                let start_delim_span = group.span_open();
                add_tt(
                    &mut res,
                    start_delim,
                    start_delim_span,
                    current_line,
                    current_column,
                    is_top,
                );
                is_top = false;

                let s = into_pretty_str_inner(group.stream(), current_line, current_column, false);
                res.push_str(&s);

                let end_delim = delim_end(group.delimiter());
                let end_delim_span = group.span_close();
                add_tt(
                    &mut res,
                    end_delim,
                    end_delim_span,
                    current_line,
                    current_column,
                    false,
                );
            }
        }
    }

    res
}

fn delim_start(d: Delimiter) -> &'static str {
    match d {
        Delimiter::Parenthesis => "(",
        Delimiter::Brace => "{",
        Delimiter::Bracket => "[",
        Delimiter::None => "",
    }
}

fn delim_end(d: Delimiter) -> &'static str {
    match d {
        Delimiter::Parenthesis => ")",
        Delimiter::Brace => "}",
        Delimiter::Bracket => "]",
        Delimiter::None => "",
    }
}

fn add_line_prefix(
    s: &str,
    start_line: usize,
    end_line: usize,
    top_padding: bool,
    bottom_padding: bool,
) -> String {
    let padding_num = end_line.to_string().chars().count();
    let mut res = String::new();
    let mut index = start_line;

    if top_padding {
        res.push_str(&format!("{}|\n", " ".repeat(padding_num)));
    }

    for line in s.lines() {
        res.push_str(&format!("{index: >padding_num$}| {line}\n"));
        index += 1;
    }

    if bottom_padding {
        res.push_str(&format!("{}|\n", " ".repeat(padding_num)));
    }

    res
}

#[cfg(test)]
mod tests {
    use syn::Expr;

    use super::{
        ExcerptSetting, PrettyStrSettings, TruncateLitStrSetting, into_excerpted_line_pretty_str,
        into_one_line_pretty_str, into_pretty_str, truncate_lit_str,
    };

    #[test]
    fn test_truncate_lit_str() {
        let ts = quote::quote! {
            func_call("0123456789abcdefghijklmnopqrstuvwxyz")
        };

        let res = truncate_lit_str(ts.clone(), None).unwrap();

        insta::assert_snapshot!(res.to_string(), @r#"func_call ("0123..wxyz")"#);

        let res = truncate_lit_str(ts, Some(36)).unwrap();

        insta::assert_snapshot!(res.to_string(), @r#"func_call ("0123456789abcdefghijklmnopqrstuvwxyz")"#);
    }

    #[test]
    fn test_truncate_lit_str_with_multiline() {
        let ts = quote::quote! {
            func_call("The Zen of Python, by Tim Peters

Beautiful is better than ugly.
Explicit is better than implicit.
Simple is better than complex.
Complex is better than complicated.
Flat is better than nested.
Sparse is better than dense.
Readability counts.
Special cases aren't special enough to break the rules.
Although practicality beats purity.
Errors should never pass silently.
Unless explicitly silenced.
In the face of ambiguity, refuse the temptation to guess.
There should be one-- and preferably only one --obvious way to do it.
Although that way may not be obvious at first unless you're Dutch.
Now is better than never.
Although never is often better than *right* now.
If the implementation is hard to explain, it's a bad idea.
If the implementation is easy to explain, it may be a good idea.
Namespaces are one honking great idea -- let's do more of those!", "
Next line
")
        };

        let res = truncate_lit_str(ts, None).unwrap();

        insta::assert_snapshot!(res.to_string(), @r#"func_call ("The ..ters\n\nBeau..gly.\nExpl..cit.\nSimp..lex.\nComp..ted.\nFlat..ted.\nSpar..nse.\nRead..nts.\nSpec..les.\nAlth..ity.\nErro..tly.\nUnle..ced.\nIn t..ess.\nTher.. it.\nAlth..tch.\nNow ..ver.\nAlth..now.\nIf t..dea.\nIf t..dea.\nName..ose!" , "\nNext line\n")"#);
    }

    // proc_macro::Span を利用する都合上
    // その他のテストについては統合テストで行う
}
