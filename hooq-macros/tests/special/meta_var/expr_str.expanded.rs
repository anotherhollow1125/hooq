use hooq_macros::hooq;
struct Magic {
    word: &'static str,
    reinforcement_word: &'static str,
    reinforcement_count: usize,
    max_reinforcement: usize,
}
impl Magic {
    fn diacute(self) -> Result<Self, String> {
        if self.reinforcement_count >= self.max_reinforcement {
            return Err(
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "Cannot reinforce \'{0}\' more than {1} times", self.word,
                            self.max_reinforcement
                        ),
                    )
                }),
            );
        }
        Ok(Self {
            word: self.word,
            reinforcement_word: self.reinforcement_word,
            reinforcement_count: self.reinforcement_count + 1,
            max_reinforcement: self.max_reinforcement,
        })
    }
    fn cast_spell(self) -> String {
        let mut result = String::new();
        for _ in 0..self.reinforcement_count {
            result.push_str(self.reinforcement_word);
        }
        result.push_str(self.word);
        result
    }
}
fn test_expr_str() -> Result<(), String> {
    let magic = Magic {
        word: "ファイヤー",
        reinforcement_word: "ファ",
        reinforcement_count: 0,
        max_reinforcement: 6,
    };
    let reinforced_magic = magic
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str:\n{4}\n", "expr_str.rs", 52usize,
                        19usize, e, "magic.diacute()"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str:\n{4}\n", "expr_str.rs", 53usize,
                        19usize, e, "magic.diacute() ? .diacute()"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str:\n{4}\n", "expr_str.rs", 54usize,
                        19usize, e, "magic.diacute() ? .diacute() ? .diacute()"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str:\n{4}\n", "expr_str.rs", 55usize,
                        19usize, e,
                        "magic.diacute() ? .diacute() ? .diacute() ? .diacute()"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str:\n{4}\n", "expr_str.rs", 56usize,
                        19usize, e,
                        "magic.diacute() ? .diacute() ? .diacute() ? .diacute() ? .diacute()"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str:\n{4}\n", "expr_str.rs", 57usize,
                        19usize, e,
                        "magic.diacute() ? .diacute() ? .diacute() ? .diacute() ? .diacute() ?\n.diacute()"
                    ),
                );
            };
        })?;
    {
        ::std::io::_print(
            format_args!("Cast spell: {0}\n", reinforced_magic.cast_spell()),
        );
    };
    Ok(())
}
fn test_expr_str_short() -> Result<(), String> {
    let magic = Magic {
        word: "ファイヤー",
        reinforcement_word: "ファ",
        reinforcement_count: 0,
        max_reinforcement: 6,
    };
    let reinforced_magic = magic
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str_short:\n{4}\n", "expr_str.rs",
                        79usize, 19usize, e, "magic.diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str_short:\n{4}\n", "expr_str.rs",
                        80usize, 19usize, e, "    ...\n    .diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str_short:\n{4}\n", "expr_str.rs",
                        81usize, 19usize, e, "    ...\n    .diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str_short:\n{4}\n", "expr_str.rs",
                        82usize, 19usize, e, "    ...\n    .diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str_short:\n{4}\n", "expr_str.rs",
                        83usize, 19usize, e, "    ...\n    .diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str_short:\n{4}\n", "expr_str.rs",
                        84usize, 19usize, e, "    ...\n    .diacute()?"
                    ),
                );
            };
        })?;
    {
        ::std::io::_print(
            format_args!("Cast spell: {0}\n", reinforced_magic.cast_spell()),
        );
    };
    Ok(())
}
fn test_expr_str_short_oneline() -> Result<(), String> {
    let magic = Magic {
        word: "ファイヤー",
        reinforcement_word: "ファ",
        reinforcement_count: 0,
        max_reinforcement: 6,
    };
    let reinforced_magic = magic
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str_short_oneline:\n{4}\n",
                        "expr_str.rs", 106usize, 19usize, e, "magic.diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str_short_oneline:\n{4}\n",
                        "expr_str.rs", 107usize, 19usize, e, "... .diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str_short_oneline:\n{4}\n",
                        "expr_str.rs", 108usize, 19usize, e, "... .diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str_short_oneline:\n{4}\n",
                        "expr_str.rs", 109usize, 19usize, e, "... .diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str_short_oneline:\n{4}\n",
                        "expr_str.rs", 110usize, 19usize, e, "... .diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str_short_oneline:\n{4}\n",
                        "expr_str.rs", 111usize, 19usize, e, "... .diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str_short_oneline:\n{4}\n",
                        "expr_str.rs", 112usize, 19usize, e, "... .diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str_short_oneline:\n{4}\n",
                        "expr_str.rs", 113usize, 19usize, e, "... .diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nexpr_str_short_oneline:\n{4}\n",
                        "expr_str.rs", 114usize, 19usize, e, "... .diacute()?"
                    ),
                );
            };
        })?;
    {
        ::std::io::_print(
            format_args!("Cast spell: {0}\n", reinforced_magic.cast_spell()),
        );
    };
    Ok(())
}
fn func(flag: bool) -> Result<(), String> {
    if flag {
        return Err("An error occurred. 12345678901234567890".to_string())
            .inspect_err(|_| {
                ::std::io::_eprint(format_args!("beep\n"));
            })
            .inspect_err(|_| {
                ::std::io::_eprint(format_args!("beep\n"));
            })
            .inspect_err(|_| {
                ::std::io::_eprint(format_args!("beep\n"));
            })
            .inspect_err(|e| {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3}\nexpr_str_short_oneline:\n{4}\n",
                            "expr_str.rs", 129usize, 9usize, e,
                            "return Err(\"An e..7890\".to_string()) ... .inspect_err(|_| eprintln!(\"beep\"));"
                        ),
                    );
                };
            });
    }
    Ok(())
}
