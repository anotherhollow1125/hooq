use syn::fold::{Fold, fold_lit_str};
use syn::{Expr, File, parse_quote};

use crate::impls::inert_attr::context::HookTargetKind;

struct ShortStrFolder {
    max_str_len: usize,
    start_len: usize,
    end_len: usize,
}

impl ShortStrFolder {
    fn new(max_str_len: usize) -> Result<Self, String> {
        if max_str_len < 2 {
            return Err("max_str_len must be at least 2".to_string());
        }

        let rest_str_len = max_str_len - 2;
        let start_len = rest_str_len / 2;
        let end_len = rest_str_len / 2;

        Ok(Self {
            max_str_len,
            start_len,
            end_len,
        })
    }
}

impl Default for ShortStrFolder {
    fn default() -> Self {
        Self::new(10).unwrap()
    }
}

impl Fold for ShortStrFolder {
    fn fold_lit_str(&mut self, i: syn::LitStr) -> syn::LitStr {
        let value = i.value();
        let value = value.replace('\n', " ");
        let short_value = if value.chars().count() > self.max_str_len {
            let start: String = value.chars().take(self.start_len).collect();
            let end: String = value
                .chars()
                .rev()
                .take(self.end_len)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
            format!("{}..{}", start, end)
        } else {
            value
        };
        let i = syn::LitStr::new(&short_value, i.span());
        fold_lit_str(self, i)
    }
}

fn short_str(expr: Expr) -> Expr {
    let mut folder = ShortStrFolder::default();
    folder.fold_expr(expr)
}

fn dedent(s: &str) -> &str {
    s.strip_prefix("    ").unwrap_or(s)
}

fn adjust_indent_num(last_line: &str, indent: usize) -> usize {
    if last_line.trim_start().starts_with(['}', ')', ']']) {
        if indent > 0 { indent * 2 } else { 4 }
    } else {
        indent
    }
}

/// return 用の文字列を加工する
/// - 1行・2行の時: 全体を返す
/// - 3行以上の時: 先頭行 + ※ + 最終行を返す
///   - 最後の行が }, ), ] で始まる場合、 ※ = "    ..."
///   - 上記以外の場合、 ※ = "..."
fn get_strs_for_return(s: String) -> String {
    let lines: Vec<&str> = s.lines().collect();

    if lines.len() <= 2 {
        return lines
            .into_iter()
            // インデント一つ分削除
            .map(|line| line.strip_prefix("    ").unwrap_or(line))
            .collect::<Vec<&str>>()
            .join("\n");
    }

    let mut res = String::new();

    let mut lines_iter = lines.iter();

    let Some(first_line) = lines_iter.next() else {
        unreachable!() // because lines.len() > 2
    };
    res.push_str(dedent(first_line));

    let Some(last_line) = lines_iter.last() else {
        unreachable!() // because lines.len() > 2
    };

    let last_line = dedent(last_line);
    let indent = last_line.chars().take_while(|c| c == &' ').count();

    // "..." の前のインデント数を決定
    let indent = adjust_indent_num(last_line, indent);

    res.push('\n');
    res.push_str(&format!("{}{}", " ".repeat(indent), "..."));
    res.push('\n');

    res.push_str(last_line);

    res
}

/// `?` や tail_expr 用の文字列を加工する
/// - 1行・2行の時: 全体を返す
/// - 3行以上の時: ※ + 最終行を返す
///   - 最後の行が }, ), ] で始まる場合、 ※ = "    ..."
///   - 上記以外の場合、 ※ = "..."
fn get_strs_for_question_or_tail_expr(s: String) -> String {
    let lines: Vec<&str> = s.lines().collect();

    if lines.len() <= 2 {
        return lines
            .into_iter()
            // インデント一つ分削除
            .map(|line| line.strip_prefix("    ").unwrap_or(line))
            .collect::<Vec<&str>>()
            .join("\n");
    }

    let mut res = String::new();

    let Some(last_line) = lines.last() else {
        unreachable!() // because lines.len() > 2
    };

    let last_line = dedent(last_line);

    // line の前の部分のインデントを得る
    let indent = last_line.chars().take_while(|c| c == &' ').count();

    // "..." の前のインデント数を決定
    let indent = adjust_indent_num(last_line, indent);

    res.push_str(&format!("{}{}", " ".repeat(indent), "..."));
    res.push('\n');

    res.push_str(last_line);

    res
}

/// Expr をデバッグ出力用に加工する
/// - 文字列:
///   - "...", r#"..."# などの場合があるので、リテラル状態で加工を行う
///   - 改行文字 \n は消す
///   - 10文字以上で、両端4文字に挟まれ間に入っている文字が3文字以上の時、10文字にする
///   - "aaaabbbaaaa" -> "aaaa..aaaa"
/// - フォーマットして文字列化したのち、基本的には最後の行を取得する
///   - 例: `hoge("aaa")?` -> `hoge("aaa")`
///   - 1, 2行の場合: そのまま返す
///   - 3行以上の場合: 先頭行 + "..." + 最終行を返す (return 用)
///   - 3行以上の場合: "..." + 最終行を返す (? や tail_expr 用)
pub fn expr_shorten(expr: &Expr, hook_target: HookTargetKind) -> String {
    // メモ: マクロで提供することにする
    // HookTargetKind は文字列として $target_kind などを新たに設けて得ることにする
    // hooq のクレート名は get_hooq_crate_ident で得ることにする

    let expr = short_str(expr.clone());

    let file: File = match hook_target {
        HookTargetKind::Question => parse_quote! {
            fn _f() {
                #expr ?
            }
        },
        HookTargetKind::Return => parse_quote! {
            fn _f() {
                return #expr ;
            }
        },
        HookTargetKind::TailExpr => parse_quote! {
            fn _f() {
                #expr
            }
        },
    };

    let formatted = prettyplease::unparse(&file);

    let formatted = formatted
        .trim()
        .strip_prefix("fn _f() {")
        .and_then(|s| s.strip_suffix('}'))
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| formatted.to_string());

    match hook_target {
        HookTargetKind::Return => get_strs_for_return(formatted),
        HookTargetKind::TailExpr | HookTargetKind::Question => {
            get_strs_for_question_or_tail_expr(formatted)
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::expr_shorten;
    use crate::impls::inert_attr::context::HookTargetKind;

    #[test]
    fn test_question_one_line() {
        let expr = parse_quote! {
            hoge("aaa")
        };
        let result = expr_shorten(&expr, HookTargetKind::Question);
        assert_eq!(result, "hoge(\"aaa\")?");
    }

    #[test]
    fn test_return_one_line() {
        let expr = parse_quote! {
            hoge("aaa")
        };
        let result = expr_shorten(&expr, HookTargetKind::Return);
        assert_eq!(result, "return hoge(\"aaa\");");
    }

    #[test]
    fn test_tail_expr_one_line() {
        let expr = parse_quote! {
            hoge("aaa")
        };
        let result = expr_shorten(&expr, HookTargetKind::TailExpr);
        assert_eq!(result, "hoge(\"aaa\")");
    }

    #[test]
    fn test_question_double_line() {
        let expr = parse_quote! {
            hogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehoge()
                .foofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoo()
        };
        let result = expr_shorten(&expr, HookTargetKind::Question);
        assert_eq!(
            result,
            "hogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehoge()
    .foofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoo()?"
        );
    }

    #[test]
    fn test_return_double_line() {
        let expr = parse_quote! {
            hogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehoge()
                .foofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoo()
        };
        let result = expr_shorten(&expr, HookTargetKind::Return);
        assert_eq!(
            result,
            "return hogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehoge()
    .foofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoo();"
        );
    }

    #[test]
    fn test_tail_expr_double_line() {
        let expr = parse_quote! {
            hogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehoge()
                .foofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoo()
        };
        let result = expr_shorten(&expr, HookTargetKind::TailExpr);
        assert_eq!(
            result,
            "hogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehoge()
    .foofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoo()"
        );
    }

    #[test]
    fn test_question_multi_line() {
        let expr = parse_quote! {
            hogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehoge()
                .foo()
                .bar()
                .baz()
        };
        let result = expr_shorten(&expr, HookTargetKind::Question);
        assert_eq!(
            result,
            "    ...
    .baz()?"
        );
    }

    #[test]
    fn test_question_multi_line_with_brackets() {
        let expr = parse_quote! {
            async {
                hoge().await?;
                hoge().await?;
                hoge().await?;
                hoge().await?;
                hoge().await?;
                fuga().await?
            }
        };
        let result = expr_shorten(&expr, HookTargetKind::Question);
        assert_eq!(
            result,
            "    ...
}?"
        );
    }

    #[test]
    fn test_return_multi_line() {
        let expr = parse_quote! {
            hogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehoge()
                .foo()
                .bar()
                .baz()
        };
        let result = expr_shorten(&expr, HookTargetKind::Return);
        assert_eq!(
            result,
            "return hogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehoge()
    ...
    .baz();"
        );
    }

    #[test]
    fn test_return_multi_line_with_brackets() {
        let expr = parse_quote! {
            [
                hoge(),
                hoge(),
                hoge(),
                hoge(),
                hoge(),
                hoge(),
                hoge(),
                hoge(),
                hoge(),
                hoge(),
                hoge(),
                hoge(),
                hoge(),
                hoge(),
                hoge(),
                fuga(),
            ]
        };
        let result = expr_shorten(&expr, HookTargetKind::Return);
        assert_eq!(
            result,
            "return [
    ...
];"
        );
    }

    #[test]
    fn test_tail_expr_multi_line() {
        let expr = parse_quote! {
            hogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehoge()
                .foo()
                .bar()
                .baz()
        };
        let result = expr_shorten(&expr, HookTargetKind::TailExpr);
        assert_eq!(
            result,
            "    ...
    .baz()"
        );
    }

    #[test]
    fn test_tail_expr_multi_line_with_brackets() {
        let expr = parse_quote! {
            (
                bar(),
                bar(),
                bar(),
                bar(),
                bar(),
                bar(),
                bar(),
                bar(),
                bar(),
                bar(),
                bar(),
                bar(),
                bar(),
                bar(),
                bar(),
                baz(),
            )
        };
        let result = expr_shorten(&expr, HookTargetKind::TailExpr);
        assert_eq!(
            result,
            "    ...
)"
        );
    }

    #[test]
    fn test_long_string_literal() {
        let expr = parse_quote! {
            hoge("12345678901")
        };
        let result = expr_shorten(&expr, HookTargetKind::Question);
        assert_eq!(result, "hoge(\"1234..8901\")?");
        let result = expr_shorten(&expr, HookTargetKind::Return);
        assert_eq!(result, "return hoge(\"1234..8901\");");
        let result = expr_shorten(&expr, HookTargetKind::TailExpr);
        assert_eq!(result, "hoge(\"1234..8901\")");
    }

    #[test]
    fn test_r_string_literal() {
        let expr = parse_quote! {
            hoge(r#""This is a long string literal that exceeds the limit.""#)
        };
        let result = expr_shorten(&expr, HookTargetKind::Question);
        assert_eq!(result, "hoge(\"\\\"Thi..it.\\\"\")?");
        let result = expr_shorten(&expr, HookTargetKind::Return);
        assert_eq!(result, "return hoge(\"\\\"Thi..it.\\\"\");");
        let result = expr_shorten(&expr, HookTargetKind::TailExpr);
        assert_eq!(result, "hoge(\"\\\"Thi..it.\\\"\")");
    }
}
