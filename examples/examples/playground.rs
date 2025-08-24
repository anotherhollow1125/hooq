#![allow(deprecated)]

use clap::Parser;
use hooq::hooq;

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
enum ReturnLocation {
    A,
    B,
    C,
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, value_enum, default_value_t = ReturnLocation::A)]
    location: ReturnLocation,
}

#[hooq]
#[hooq::method(.map(|v| {
    println!("{} @ {}", $expr, $line);
    v
}))]
fn hoge(location: ReturnLocation) -> Result<(), ()> {
    use ReturnLocation::*;

    if let A = location {
        return {
            println!("A.");

            Ok(())
        };
    }

    if let B = location {
        #[hooq::skip]
        return {
            println!("B.");

            Ok(())
        };
    }

    if let C = location {
        return {
            println!("C.");

            Ok(())
        };
    }

    Ok(())
}

fn main() -> Result<(), ()> {
    let Args { location } = Args::parse();

    hoge(location)
}
