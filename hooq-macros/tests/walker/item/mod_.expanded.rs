use hooq_macros::hooq;
mod tmp {
    pub fn func() -> Result<(), ()> {
        Err(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "mod"));
                };
            })
    }
}
