use rustc_version::{Channel, version_meta};

fn main() {
    let version_meta = version_meta().expect("Failed to fetch rustc version metadata");
    if version_meta.channel == Channel::Nightly {
        println!(r#"cargo:rustc-cfg=feature="nightly""#);
    }
}
