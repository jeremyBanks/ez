#![warn(dead_code)]

use ez::try_or_panics;

#[try_or_panics]
fn main() {
    // let _a = alice();
    // let _a = try_alice()?;
    // let _f = try_alice();

    // let _b = bob("1");
    // let _b = try_bob("threeve")?;
    // let _b = try_bob("3");
}

/// This is Alice.
#[try_or_panics]
fn alice() -> i64 {
    try_bob("s")?
}

#[try_or_panics(std::num::ParseIntError)]
/// This is Bob.
fn bob(n: &str) -> i64 {
    n.parse()?
}

trait Foo {
    // we're not detecting the missing block correctly?
    #[try_or_panics(std::num::ParseIntError)]
    fn foo(&self) -> i64;
}

struct Bar;

impl Foo for Bar {
    fn try_foo(&self) -> Result<i64, std::num::ParseIntError> {
        try_bob("threeve")
    }
}
