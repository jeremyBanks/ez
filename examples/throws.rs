use std::collections::HashMap;

#[ez::main]
fn main(args: Vec<String>, b: Vec<(String, String)>) {
    // let _a = alice();
    // let _a = try_alice()?;
    // let _f = try_alice();

    let _b = bob("1");
    let _b = try_bob("threeve")?;
    let _b = try_bob("3");

    // 32
    // Is there some dumb thing we can do with impl trait to make
    // explicit or implicit returns both work, without requiring the
    // manual use of Ok()?
}

fn foo() -> impl ez::errors::IntoResult<i32, eyre::Report> {
    if 1 % 2 == 3 {
        1
    } else {
        return 2
    }
}

#[ez::throws]
fn zzzz() -> impl ez::errors::IntoResult<i32, eyre::Report> {
    if 1 % 2 == 3 {
        1
    } else {
        return 2
    }
}

/// This is Alice.
#[ez::try_or_panics]
fn alice() -> i64 {
    try_bob("s")?
}

#[ez::try_or_panics(std::num::ParseIntError)]
/// This is Bob.
fn bob(n: &str) -> i64 {
    n.parse()?
}

trait Foo {
    #[ez::try_or_panics(std::num::ParseIntError)]
    fn foo(&self) -> i64;
}

struct Bar;

impl Foo for Bar {
    fn try_foo(&self) -> Result<i64, std::num::ParseIntError> {
        try_bob("threeve")
    }
}
