use hooq_macros::hooq;
mod tmp {
    pub fn func() -> Result<(), ()> {
        Ok(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "mod"));
                };
            })
    }
}
