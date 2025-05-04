#![allow(deprecated)]

use hooq::hooq;

#[hooq]
#[hooq::method(.map_err(|e| {
    eprintln!("[{}@{}] {:?}", $line, $file, e);
    e
}))]
fn hogehoge() -> Result<(), String> {
    (|| {
        Err({
            #[hooq::skip]
            fuga()?;
            fuga()?;
            "hoge".to_string()
        })
    })()
}

#[hooq]
#[hooq::method(.map_err(|e| {
    eprintln!("[] {}@{} {:?}", $line, file!(), e);
    e
}))]
fn hoge() -> Result<(), String> {
    hogehoge()?;
    Ok(())
}

#[hooq]
#[hooq::method(.map_err(|e| {
    eprintln!("[] {}@{} {:?}", $line, file!(), e);
    e
}))]
fn fuga() -> Result<(), String> {
    Ok(())
}

#[hooq]
#[hooq::method(.map_err(|e| {
    eprintln!("[] {}@{} {:?}", $line, file!(), e);
    e
}))]
fn bar() -> Result<(), String> {
    #[allow(clippy::needless_return)]
    return hoge();
}

#[hooq]
#[hooq::method(.map_err(|e| {
    eprintln!("[] {}@{} {:?}", $line, file!(), e);
    e
}))]
fn func() -> Result<(), String> {
    hoge()?;

    #[hooq::skip]
    hoge()?;

    bar()?;

    Ok(())
}

fn main() {
    func().unwrap();
}
