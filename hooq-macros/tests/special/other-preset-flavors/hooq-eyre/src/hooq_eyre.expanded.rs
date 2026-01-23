use hooq::hooq;
#[allow(unused)]
use ::eyre::ContextCompat as _;
#[allow(unused)]
use ::eyre::WrapErr as _;
pub fn a() -> eyre::Result<()> {
    let _n = { Some(10) }
        .wrap_err_with(|| {
            let path = "<hooq_root>/src/hooq_eyre.rs";
            let line = 5usize;
            let col = 26usize;
            let expr = "   5>    { Some(10) }?\n    |";
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                )
            })
        })?;
    Err({
            let error = ::eyre::private::format_err(format_args!("error!"));
            error
        })
        .wrap_err_with(|| {
            let path = "<hooq_root>/src/hooq_eyre.rs";
            let line = 7usize;
            let col = 5usize;
            let expr = "   7>    Err(eyre::eyre!(\"error!\"))\n    |";
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                )
            })
        })
}
#[allow(unused)]
use ::eyre::ContextCompat as _;
#[allow(unused)]
use ::eyre::WrapErr as _;
pub fn b() -> eyre::Result<()> {
    a()
        .wrap_err_with(|| {
            let path = "<hooq_root>/src/hooq_eyre.rs";
            let line = 12usize;
            let col = 8usize;
            let expr = "  12>    a()?\n    |";
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                )
            })
        })?;
    Err({
            let error = ::eyre::private::format_err(format_args!("error!"));
            error
        })
        .wrap_err_with(|| {
            let path = "<hooq_root>/src/hooq_eyre.rs";
            let line = 14usize;
            let col = 5usize;
            let expr = "  14>    Err(eyre::eyre!(\"error!\"))\n    |";
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                )
            })
        })
}
#[allow(unused)]
use ::eyre::ContextCompat as _;
#[allow(unused)]
use ::eyre::WrapErr as _;
pub fn c() -> eyre::Result<()> {
    b()
        .wrap_err_with(|| {
            let path = "<hooq_root>/src/hooq_eyre.rs";
            let line = 19usize;
            let col = 8usize;
            let expr = "  19>    b()?\n    |";
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                )
            })
        })?;
    Err({
            let error = ::eyre::private::format_err(format_args!("error!"));
            error
        })
        .wrap_err_with(|| {
            let path = "<hooq_root>/src/hooq_eyre.rs";
            let line = 21usize;
            let col = 5usize;
            let expr = "  21>    Err(eyre::eyre!(\"error!\"))\n    |";
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                )
            })
        })
}
