use proc_macro2::{Delimiter, Span, TokenStream};
use quote::ToTokens;
use syn::Expr;
use syn::fold::{Fold, fold_lit_str};
use syn::spanned::Spanned;

// --- primitives start ---

/// Truncate string literals in the given expression.
/// eg. "01234567891234" -> "0123..1234"
/// Truncation is done line by line.
pub fn truncate_lit_str(expr: Expr, max_len: Option<usize>) -> syn::Result<TokenStream> {
    let expr = truncate_lit_str_inner(expr, max_len);
    Ok(expr.into_token_stream())
}

pub struct IntoPrettyStrRes {
    body: String,
    padding: Padding,
}

impl std::fmt::Display for IntoPrettyStrRes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.padding.wrap(&self.body))
    }
}

pub struct Padding {
    top_padding: Option<String>,
    bottom_padding: Option<String>,
}

impl Padding {
    pub fn wrap(&self, expr_str: &str) -> String {
        format!(
            "{}{expr_str}{}",
            self.top_padding.as_deref().unwrap_or(""),
            self.bottom_padding.as_deref().unwrap_or(""),
        )
    }
}

/// Convert the given expression into a pretty-printed string representation.
pub fn into_pretty_str(
    expr: Expr,
    PrettyStrSettings {
        with_line,
        top_padding,
        bottom_padding,
    }: PrettyStrSettings,
) -> IntoPrettyStrRes {
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
        IntoPrettyStrRes {
            body: res,
            padding: Padding {
                top_padding: top_padding.then(|| "\n".to_string()),
                bottom_padding: bottom_padding.then(|| "\n".to_string()),
            },
        }
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
            max_excerpted_line_num: 5,
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
    expr: Expr,
    truncate_lit_str_setting: TruncateLitStrSetting,
) -> syn::Result<String> {
    let expr = truncate_lit_str_setting.apply(expr);
    let expr_str = into_pretty_str(
        expr,
        PrettyStrSettings {
            with_line: false,
            top_padding: false,
            bottom_padding: false,
        },
    )
    .body
    .lines()
    .map(|line| line.trim())
    .collect::<Vec<&str>>()
    .join(" ");

    Ok(expr_str)
}

pub fn into_excerpted_line_pretty_str(
    expr: Expr,
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
        let expr = truncate_lit_str_setting.apply(expr);
        let IntoPrettyStrRes {
            body: expr_str,
            padding,
        } = into_pretty_str(expr, pretty_str_settings);
        let expr_str = expr_str.lines().last().unwrap_or("").to_string();

        return Ok(padding.wrap(&expr_str));
    }

    // 以降は欲しい行が2行以上の場合

    let expr = truncate_lit_str_setting.apply(expr);
    let IntoPrettyStrRes {
        body: expr_str,
        padding,
    } = into_pretty_str(expr, pretty_str_settings);
    let line_count = expr_str.lines().count();

    // 対象の行が2行で収まる場合はすべて取る
    if line_count <= 2 {
        return Ok(padding.wrap(&expr_str));
    }

    // 以降は対象の行が3行以上の場合
    // 先頭行と、最終行に近い行から順に取る
    let mut lines_iter = expr_str.lines();

    let first_line = lines_iter.next().unwrap_or("");

    let mut res = Vec::new();
    res.push(first_line);

    let tail_lines = lines_iter
        .rev()
        .take(max_excerpted_line_num - 1)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>();

    if line_count > max_excerpted_line_num {
        res.push("...");
    }
    res.extend(tail_lines);

    let res = res.join("\n");

    Ok(padding.wrap(&res))
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

    struct AddTTPos {
        line: usize,
        column: usize,
        end_column: usize,
    }

    fn add_tt(
        res: &mut String,
        tt_str: &str,
        // NOTE:
        // Group のデリミタ部分について
        // span_open().unwrap() が意図通りの
        // 位置を指示さなかったため、
        // 行列を別個に受け取るようにした
        AddTTPos {
            line: tt_line,
            column: tt_column,
            end_column: tt_end_column,
        }: AddTTPos,

        current_line: &mut Option<usize>,
        current_column: &mut Option<usize>,
        is_top: bool,
    ) {
        if is_top {
            res.push_str("    ");
        }

        if let Some(line) = current_line
            && tt_line > *line
        {
            res.push_str("\n".repeat(tt_line.saturating_sub(*line)).as_str());
            res.push_str(" ".repeat(tt_column.saturating_sub(1)).as_str());
        } else if let Some(column) = current_column
            && tt_column > *column
        {
            res.push_str(" ".repeat(tt_column.saturating_sub(*column)).as_str());
        }

        res.push_str(tt_str);

        *current_line = Some(tt_line);
        *current_column = Some(tt_end_column);
    }

    for tt in input {
        match tt {
            proc_macro2::TokenTree::Ident(_)
            | proc_macro2::TokenTree::Punct(_)
            | proc_macro2::TokenTree::Literal(_) => {
                let tt_str = tt.to_string();
                let tt_span = tt.span().unwrap();

                add_tt(
                    &mut res,
                    &tt_str,
                    AddTTPos {
                        line: tt_span.start().line(),
                        column: tt_span.start().column(),
                        end_column: tt_span.end().column(),
                    },
                    current_line,
                    current_column,
                    is_top,
                );
                is_top = false;
            }
            proc_macro2::TokenTree::Group(group) => {
                let start_delim = delim_start(group.delimiter());
                let start_delim_span = group.span_open().unwrap();
                add_tt(
                    &mut res,
                    start_delim,
                    AddTTPos {
                        line: start_delim_span.start().line(),
                        column: start_delim_span.start().column(),
                        end_column: start_delim_span.start().column() + 1,
                    },
                    current_line,
                    current_column,
                    is_top,
                );
                is_top = false;

                let s = into_pretty_str_inner(group.stream(), current_line, current_column, false);
                res.push_str(&s);

                let end_delim = delim_end(group.delimiter());
                let end_delim_span = group.span_close().unwrap();
                add_tt(
                    &mut res,
                    end_delim,
                    AddTTPos {
                        line: end_delim_span.end().line(),
                        column: end_delim_span.end().column().saturating_sub(1),
                        end_column: end_delim_span.end().column(),
                    },
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
) -> IntoPrettyStrRes {
    let padding_num = end_line.to_string().chars().count().max(4);
    let mut res = Vec::new();
    let mut index = start_line;
    let mut punct = '>';

    for line in s.lines() {
        res.push(format!("{index: >padding_num$}{punct}{line}"));
        punct = '|';
        index += 1;
    }

    IntoPrettyStrRes {
        body: res.join("\n"),
        padding: Padding {
            top_padding: top_padding.then(|| format!("{}|\n", " ".repeat(padding_num))),
            bottom_padding: bottom_padding.then(|| format!("\n{}|", " ".repeat(padding_num))),
        },
    }
}

#[cfg(test)]
mod tests {
    use syn::Expr;

    use super::truncate_lit_str;

    #[test]
    fn test_truncate_lit_str() {
        let ts = quote::quote! {
            func_call("0123456789abcdefghijklmnopqrstuvwxyz")
        };

        let expr = syn::parse2::<Expr>(ts).unwrap();

        let res = truncate_lit_str(expr.clone(), None).unwrap();

        insta::assert_snapshot!(res.to_string(), @r#"func_call ("0123..wxyz")"#);

        let res = truncate_lit_str(expr, Some(36)).unwrap();
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

        let expr = syn::parse2::<Expr>(ts).unwrap();

        let res = truncate_lit_str(expr, None).unwrap();

        insta::assert_snapshot!(res.to_string(), @r#"func_call ("The ..ters\n\nBeau..gly.\nExpl..cit.\nSimp..lex.\nComp..ted.\nFlat..ted.\nSpar..nse.\nRead..nts.\nSpec..les.\nAlth..ity.\nErro..tly.\nUnle..ced.\nIn t..ess.\nTher.. it.\nAlth..tch.\nNow ..ver.\nAlth..now.\nIf t..dea.\nIf t..dea.\nName..ose!" , "\nNext line\n")"#);
    }

    // proc_macro::Span を利用する都合上
    // その他のテストについては統合テストで行う
}
