use syn::fold::{Fold, fold_lit_str};
use syn::{Expr, File, parse_quote};

use crate::impls::inert_attr::context::HookTargetKind;

struct ShortStrFolder;

const MAX_STR_LEN: usize = 10;
const STD_START_LEN: usize = 4;
const STD_END_LEN: usize = 4;

impl Fold for ShortStrFolder {
    fn fold_lit_str(&mut self, i: syn::LitStr) -> syn::LitStr {
        let value = i.value();
        let value = value.replace('\n', " ");
        let short_value = if value.chars().count() > MAX_STR_LEN {
            let start: String = value.chars().take(STD_START_LEN).collect();
            let end: String = value
                .chars()
                .rev()
                .take(STD_END_LEN)
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
    let mut folder = ShortStrFolder;
    folder.fold_expr(expr)
}

/// return 用の文字列を加工する
/// - 1行・2行の時: 全体を返す
/// - 3行以上の時: 先頭行 + "..." + 最終行を返す
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

    for (i, line) in lines.iter().enumerate() {
        // インデント一つ分削除
        let line = line.strip_prefix("    ").unwrap_or(line);
        if i == 0 {
            res.push_str(line);
        } else if i == lines.len() - 1 {
            // line の前の部分のインデントを得る
            let indent = line.chars().take_while(|c| c == &' ').count();
            res.push('\n');
            res.push_str(&format!("{}{}", " ".repeat(indent), "..."));
            res.push('\n');

            res.push_str(line);
        }
    }

    res
}

/// `?` や tail_expr 用の文字列を加工する
/// - 1行・2行の時: 全体を返す
/// - 3行以上の時: "..." + 最終
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

    for (i, line) in lines.iter().enumerate() {
        // インデント一つ分削除
        let line = line.strip_prefix("    ").unwrap_or(line);
        if i == lines.len() - 1 {
            // line の前の部分のインデントを得る
            let indent = line.chars().take_while(|c| c == &' ').count();
            res.push_str(&format!("{}{}", " ".repeat(indent), "..."));
            res.push('\n');

            res.push_str(line);
        }
    }

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
pub fn describe_expr_short(expr: &Expr, hook_target: HookTargetKind) -> String {
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

    use super::describe_expr_short;
    use crate::impls::inert_attr::context::HookTargetKind;

    #[test]
    fn test_question_one_line() {
        let expr = parse_quote! {
            hoge("aaa")
        };
        let result = describe_expr_short(&expr, HookTargetKind::Question);
        assert_eq!(result, "hoge(\"aaa\")?");
    }

    #[test]
    fn test_return_one_line() {
        let expr = parse_quote! {
            hoge("aaa")
        };
        let result = describe_expr_short(&expr, HookTargetKind::Return);
        assert_eq!(result, "return hoge(\"aaa\");");
    }

    #[test]
    fn test_tail_expr_one_line() {
        let expr = parse_quote! {
            hoge("aaa")
        };
        let result = describe_expr_short(&expr, HookTargetKind::TailExpr);
        assert_eq!(result, "hoge(\"aaa\")");
    }

    #[test]
    fn test_question_double_line() {
        let expr = parse_quote! {
            hogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehoge()
                .foofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoo()
        };
        let result = describe_expr_short(&expr, HookTargetKind::Question);
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
        let result = describe_expr_short(&expr, HookTargetKind::Return);
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
        let result = describe_expr_short(&expr, HookTargetKind::TailExpr);
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
        let result = describe_expr_short(&expr, HookTargetKind::Question);
        assert_eq!(
            result,
            "    ...
    .baz()?"
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
        let result = describe_expr_short(&expr, HookTargetKind::Return);
        assert_eq!(
            result,
            "return hogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehogehoge()
    ...
    .baz();"
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
        let result = describe_expr_short(&expr, HookTargetKind::TailExpr);
        assert_eq!(
            result,
            "    ...
    .baz()"
        );
    }

    #[test]
    fn test_long_string_literal() {
        let expr = parse_quote! {
            hoge("12345678901")
        };
        let result = describe_expr_short(&expr, HookTargetKind::Question);
        assert_eq!(result, "hoge(\"1234..8901\")?");
        let result = describe_expr_short(&expr, HookTargetKind::Return);
        assert_eq!(result, "return hoge(\"1234..8901\");");
        let result = describe_expr_short(&expr, HookTargetKind::TailExpr);
        assert_eq!(result, "hoge(\"1234..8901\")");
    }

    #[test]
    fn test_r_string_literal() {
        let expr = parse_quote! {
            hoge(r#""This is a long string literal that exceeds the limit.""#)
        };
        let result = describe_expr_short(&expr, HookTargetKind::Question);
        assert_eq!(result, "hoge(\"\\\"Thi..it.\\\"\")?");
        let result = describe_expr_short(&expr, HookTargetKind::Return);
        assert_eq!(result, "return hoge(\"\\\"Thi..it.\\\"\");");
        let result = describe_expr_short(&expr, HookTargetKind::TailExpr);
        assert_eq!(result, "hoge(\"\\\"Thi..it.\\\"\")");
    }
}
