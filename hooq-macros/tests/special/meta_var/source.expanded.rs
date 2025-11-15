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
                        "[{0}:{1}:{2}] {3}\nsource_str:\n{4}\n", "source.rs", 52usize,
                        19usize, e,
                        "  51|                            magic\n  52|         .diacute()?\n    |"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nsource_str:\n{4}\n", "source.rs", 53usize,
                        19usize, e,
                        "  51|                            magic\n  52|         .diacute()?\n  53|         .diacute()?\n    |"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nsource_str:\n{4}\n", "source.rs", 54usize,
                        19usize, e,
                        "  51|                            magic\n  52|         .diacute()?\n  53|         .diacute()?\n  54|         .diacute()?\n    |"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nsource_str:\n{4}\n", "source.rs", 55usize,
                        19usize, e,
                        "  51|                            magic\n  52|         .diacute()?\n  53|         .diacute()?\n  54|         .diacute()?\n  55|         .diacute()?\n    |"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nsource_str:\n{4}\n", "source.rs", 56usize,
                        19usize, e,
                        "  51|                            magic\n...\n  53|         .diacute()?\n  54|         .diacute()?\n  55|         .diacute()?\n  56|         .diacute()?\n    |"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nsource_str:\n{4}\n", "source.rs", 57usize,
                        19usize, e,
                        "  51|                            magic\n...\n  54|         .diacute()?\n  55|         .diacute()?\n  56|         .diacute()?\n  57|         .diacute()?\n    |"
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
                        "[{0}:{1}:{2}] {3}\nsource_str_oneline:\n{4}\n", "source.rs",
                        79usize, 19usize, e,
                        "  78|                            magic\n  79|         .diacute()?\n    |"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nsource_str_oneline:\n{4}\n", "source.rs",
                        80usize, 19usize, e,
                        "  78|                            magic\n  79|         .diacute()?\n  80|         .diacute()?\n    |"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nsource_str_oneline:\n{4}\n", "source.rs",
                        81usize, 19usize, e,
                        "  78|                            magic\n  79|         .diacute()?\n  80|         .diacute()?\n  81|         .diacute()?\n    |"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nsource_str_oneline:\n{4}\n", "source.rs",
                        82usize, 19usize, e,
                        "  78|                            magic\n  79|         .diacute()?\n  80|         .diacute()?\n  81|         .diacute()?\n  82|         .diacute()?\n    |"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nsource_str_oneline:\n{4}\n", "source.rs",
                        83usize, 19usize, e,
                        "  78|                            magic\n...\n  80|         .diacute()?\n  81|         .diacute()?\n  82|         .diacute()?\n  83|         .diacute()?\n    |"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nsource_str_oneline:\n{4}\n", "source.rs",
                        84usize, 19usize, e,
                        "  78|                            magic\n...\n  81|         .diacute()?\n  82|         .diacute()?\n  83|         .diacute()?\n  84|         .diacute()?\n    |"
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
                        "[{0}:{1}:{2}] {3}\nsource_str_short_oneline:\n{4}\n",
                        "source.rs", 106usize, 19usize, e, "magic .diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nsource_str_short_oneline:\n{4}\n",
                        "source.rs", 107usize, 19usize, e,
                        "magic .diacute()? .diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nsource_str_short_oneline:\n{4}\n",
                        "source.rs", 108usize, 19usize, e,
                        "magic .diacute()? .diacute()? .diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nsource_str_short_oneline:\n{4}\n",
                        "source.rs", 109usize, 19usize, e,
                        "magic .diacute()? .diacute()? .diacute()? .diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nsource_str_short_oneline:\n{4}\n",
                        "source.rs", 110usize, 19usize, e,
                        "magic .diacute()? .diacute()? .diacute()? .diacute()? .diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nsource_str_short_oneline:\n{4}\n",
                        "source.rs", 111usize, 19usize, e,
                        "magic .diacute()? .diacute()? .diacute()? .diacute()? .diacute()? .diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nsource_str_short_oneline:\n{4}\n",
                        "source.rs", 112usize, 19usize, e,
                        "magic .diacute()? .diacute()? .diacute()? .diacute()? .diacute()? .diacute()? .diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nsource_str_short_oneline:\n{4}\n",
                        "source.rs", 113usize, 19usize, e,
                        "magic .diacute()? .diacute()? .diacute()? .diacute()? .diacute()? .diacute()? .diacute()? .diacute()?"
                    ),
                );
            };
        })?
        .diacute()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3}\nsource_str_short_oneline:\n{4}\n",
                        "source.rs", 114usize, 19usize, e,
                        "magic .diacute()? .diacute()? .diacute()? .diacute()? .diacute()? .diacute()? .diacute()? .diacute()? .diacute()?"
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
                            "[{0}:{1}:{2}] {3}\nsource_str_short_oneline:\n{4}\n",
                            "source.rs", 129usize, 9usize, e,
                            "return Err(\"An error occurred. 12345678901234567890\".to_string()) .inspect_err(|_| eprintln!(\"beep\")) .inspect_err(|_| eprintln!(\"beep\")) .inspect_err(|_| eprintln!(\"beep\"))"
                        ),
                    );
                };
            });
    }
    Ok(())
}
