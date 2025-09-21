use hooq_macros::hooq;
fn func() -> Result<(), ()> {
    Err(())
        .inspect(|_| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "abs_path: {0}\n(abspath: {1})\n",
                        "<hooq_root>/tests/special/meta_var/abs_path.rs",
                        "<hooq_root>/tests/special/meta_var/abs_path.rs"
                    ),
                );
            };
        })
}
