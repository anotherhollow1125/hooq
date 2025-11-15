use hooq::hooq;
#[allow(unused)]
use ::anyhow::Context as _;
pub fn a() -> anyhow::Result<()> {
    Err(
            ::anyhow::__private::must_use({
                let error = ::anyhow::__private::format_err(format_args!("error!"));
                error
            }),
        )
        .with_context(|| {
            let path = "<hooq_root>/src/hooq_anyhow.rs";
            let line = 5usize;
            let col = 5usize;
            let expr = "   5>    Err(anyhow::anyhow!(\"error!\"))\n    |";
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                )
            })
        })
}
#[allow(unused)]
use ::anyhow::Context as _;
pub fn b() -> anyhow::Result<()> {
    a()
        .with_context(|| {
            let path = "<hooq_root>/src/hooq_anyhow.rs";
            let line = 10usize;
            let col = 8usize;
            let expr = "  10>    a()?\n    |";
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                )
            })
        })?;
    Err(
            ::anyhow::__private::must_use({
                let error = ::anyhow::__private::format_err(format_args!("error!"));
                error
            }),
        )
        .with_context(|| {
            let path = "<hooq_root>/src/hooq_anyhow.rs";
            let line = 12usize;
            let col = 5usize;
            let expr = "  12>    Err(anyhow::anyhow!(\"error!\"))\n    |";
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                )
            })
        })
}
#[allow(unused)]
use ::anyhow::Context as _;
pub fn c() -> anyhow::Result<()> {
    b()
        .with_context(|| {
            let path = "<hooq_root>/src/hooq_anyhow.rs";
            let line = 17usize;
            let col = 8usize;
            let expr = "  17>    b()?\n    |";
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                )
            })
        })?;
    Err(
            ::anyhow::__private::must_use({
                let error = ::anyhow::__private::format_err(format_args!("error!"));
                error
            }),
        )
        .with_context(|| {
            let path = "<hooq_root>/src/hooq_anyhow.rs";
            let line = 19usize;
            let col = 5usize;
            let expr = "  19>    Err(anyhow::anyhow!(\"error!\"))\n    |";
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                )
            })
        })
}
