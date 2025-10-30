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
            return Err(format!(
                "Cannot reinforce '{}' more than {} times",
                self.word, self.max_reinforcement
            ));
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

#[hooq]
#[hooq::method(.inspect_err(|e| {
    eprintln!("[{}:{}:{}] {}
expr_str:
{}", $file, $line, $column, e, $expr_str);
}))]
fn test_expr_str() -> Result<(), String> {
    let magic = Magic {
        word: "ファイヤー",
        reinforcement_word: "ファ",
        reinforcement_count: 0,
        max_reinforcement: 6,
    };

    let reinforced_magic = magic
        .diacute()?
        .diacute()?
        .diacute()?
        .diacute()?
        .diacute()?
        .diacute()?;

    println!("Cast spell: {}", reinforced_magic.cast_spell());

    Ok(())
}

#[hooq]
#[hooq::method(.inspect_err(|e| {
    eprintln!("[{}:{}:{}] {}
expr_str_short:
{}", $file, $line, $column, e, $expr_str_short);
}))]
fn test_expr_str_short() -> Result<(), String> {
    let magic = Magic {
        word: "ファイヤー",
        reinforcement_word: "ファ",
        reinforcement_count: 0,
        max_reinforcement: 6,
    };

    let reinforced_magic = magic
        .diacute()?
        .diacute()?
        .diacute()?
        .diacute()?
        .diacute()?
        .diacute()?;

    println!("Cast spell: {}", reinforced_magic.cast_spell());

    Ok(())
}

#[hooq]
#[hooq::method(.inspect_err(|e| {
    eprintln!("[{}:{}:{}] {}
expr_str_short_oneline:
{}", $file, $line, $column, e, $expr_str_short_oneline);
}))]
fn test_expr_str_short_oneline() -> Result<(), String> {
    let magic = Magic {
        word: "ファイヤー",
        reinforcement_word: "ファ",
        reinforcement_count: 0,
        max_reinforcement: 6,
    };

    let reinforced_magic = magic
        .diacute()?
        .diacute()?
        .diacute()?
        .diacute()?
        .diacute()?
        .diacute()?
        .diacute()?
        .diacute()?
        .diacute()?; // This should be error

    println!("Cast spell: {}", reinforced_magic.cast_spell());

    Ok(())
}

#[hooq]
#[hooq::method(.inspect_err(|e| {
    eprintln!("[{}:{}:{}] {}
expr_str_short_oneline:
{}", $file, $line, $column, e, $expr_str_short_oneline);
}))]
fn func(flag: bool) -> Result<(), String> {
    if flag {
        return Err("An error occurred. 12345678901234567890".to_string())
            .inspect_err(|_| eprintln!("beep"))
            .inspect_err(|_| eprintln!("beep"))
            .inspect_err(|_| eprintln!("beep"));
    }

    Ok(())
}

#[test]
fn test() {
    test_expr_str().unwrap();
    test_expr_str_short().unwrap();
}

#[test]
#[should_panic]
fn test_panic_1() {
    test_expr_str_short_oneline().unwrap();
}

#[test]
#[should_panic]
fn test_panic_2() {
    func(true).unwrap();
}
