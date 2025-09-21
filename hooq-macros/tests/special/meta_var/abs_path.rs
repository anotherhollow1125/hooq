use hooq_macros::hooq;

// スナップショットテストでは abs_path だけではなく
// path も絶対パスを返すので検証としての意味は薄い
// abs_path 自体がうまく機能しているかは all_info 等で別途テストの必要あり

#[hooq]
#[hooq::method(.inspect(|_| {
    eprintln!("abs_path: {}\n(abspath: {})", $abs_path, $abspath);
}))]
fn func() -> Result<(), ()> {
    Err(())
}

#[test]
fn test() {
    func().unwrap_err();
}
