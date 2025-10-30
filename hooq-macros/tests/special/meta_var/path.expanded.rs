use hooq_macros::hooq;
fn func() -> Result<(), ()> {
    Err(())
        .inspect(|_| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "path: {0}\n",
                        "<hooq_root>/tests/special/meta_var/path.rs"
                    ),
                );
            };
        })
}
