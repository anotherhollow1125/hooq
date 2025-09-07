use hooq::hooq;
#[allow(unused)]
use anyhow::Context as _;
pub fn a() -> anyhow::Result<()> {
    Err(
            ::anyhow::__private::must_use({
                let error = ::anyhow::__private::format_err(format_args!("error!"));
                error
            }),
        )
        .with_context(|| {
            let file = "hooq_anyhow.rs";
            let line = 5usize;
            let expr_str = "Err(anyhow :: anyhow! (\"error!\"))";
            let expr_str = if expr_str.len() > 20 {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("...{0}", & expr_str[expr_str.len() - 20..]),
                    )
                })
            } else {
                expr_str.to_string()
            };
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("[{0}:L{1}] {2}", file, line, expr_str),
                )
            })
        })
}
#[allow(unused)]
use anyhow::Context as _;
pub fn b() -> anyhow::Result<()> {
    a()
        .with_context(|| {
            let file = "hooq_anyhow.rs";
            let line = 10usize;
            let expr_str = "a()";
            let expr_str = if expr_str.len() > 20 {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("...{0}", & expr_str[expr_str.len() - 20..]),
                    )
                })
            } else {
                expr_str.to_string()
            };
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("[{0}:L{1}] {2}", file, line, expr_str),
                )
            })
        })?;
    Result::<(), anyhow::Error>::Ok(())
        .with_context(|| {
            let file = "hooq_anyhow.rs";
            let line = 12usize;
            let expr_str = "Result :: < (), anyhow :: Error > :: Ok(())";
            let expr_str = if expr_str.len() > 20 {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("...{0}", & expr_str[expr_str.len() - 20..]),
                    )
                })
            } else {
                expr_str.to_string()
            };
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("[{0}:L{1}] {2}", file, line, expr_str),
                )
            })
        })
}
#[allow(unused)]
use anyhow::Context as _;
pub fn c() -> anyhow::Result<()> {
    b()
        .with_context(|| {
            let file = "hooq_anyhow.rs";
            let line = 17usize;
            let expr_str = "b()";
            let expr_str = if expr_str.len() > 20 {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("...{0}", & expr_str[expr_str.len() - 20..]),
                    )
                })
            } else {
                expr_str.to_string()
            };
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("[{0}:L{1}] {2}", file, line, expr_str),
                )
            })
        })?;
    Result::<(), anyhow::Error>::Ok(())
        .with_context(|| {
            let file = "hooq_anyhow.rs";
            let line = 19usize;
            let expr_str = "Result :: < (), anyhow :: Error > :: Ok(())";
            let expr_str = if expr_str.len() > 20 {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("...{0}", & expr_str[expr_str.len() - 20..]),
                    )
                })
            } else {
                expr_str.to_string()
            };
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("[{0}:L{1}] {2}", file, line, expr_str),
                )
            })
        })
}
