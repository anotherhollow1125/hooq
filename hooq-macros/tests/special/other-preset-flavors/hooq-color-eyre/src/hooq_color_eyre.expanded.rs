use color_eyre::eyre::{Result, eyre};
use hooq::hooq;
#[allow(unused)]
use ::color_eyre::eyre::ContextCompat as _;
#[allow(unused)]
use ::color_eyre::eyre::WrapErr as _;
pub fn a() -> Result<()> {
    let _n = { Some(10) }
        .wrap_err_with(|| {
            let path = "<hooq_root>/src/hooq_color_eyre.rs";
            let line = 6usize;
            let col = 26usize;
            let expr = "   6>    { Some(10) }?\n    |";
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
            let path = "<hooq_root>/src/hooq_color_eyre.rs";
            let line = 8usize;
            let col = 5usize;
            let expr = "   8>    Err(eyre!(\"error!\"))\n    |";
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                )
            })
        })
}
#[allow(unused)]
use ::color_eyre::eyre::ContextCompat as _;
#[allow(unused)]
use ::color_eyre::eyre::WrapErr as _;
pub fn b() -> Result<()> {
    a()
        .wrap_err_with(|| {
            let path = "<hooq_root>/src/hooq_color_eyre.rs";
            let line = 13usize;
            let col = 8usize;
            let expr = "  13>    a()?\n    |";
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
            let path = "<hooq_root>/src/hooq_color_eyre.rs";
            let line = 15usize;
            let col = 5usize;
            let expr = "  15>    Err(eyre!(\"error!\"))\n    |";
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                )
            })
        })
}
#[allow(unused)]
use ::color_eyre::eyre::ContextCompat as _;
#[allow(unused)]
use ::color_eyre::eyre::WrapErr as _;
pub fn c() -> Result<()> {
    b()
        .wrap_err_with(|| {
            let path = "<hooq_root>/src/hooq_color_eyre.rs";
            let line = 20usize;
            let col = 8usize;
            let expr = "  20>    b()?\n    |";
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
            let path = "<hooq_root>/src/hooq_color_eyre.rs";
            let line = 22usize;
            let col = 5usize;
            let expr = "  22>    Err(eyre!(\"error!\"))\n    |";
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                )
            })
        })
}
