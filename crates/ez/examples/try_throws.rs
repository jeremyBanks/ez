#[ez::try_throws]
#[allow(clippy::let_unit_value)] // false positive: macro transforms this into Result-returning fn
fn something_that_fails() {
    throw!("oh no!");
}

fn main() {
    match try_something_that_fails() {
        Err(_err) => something_that_fails(),
        Ok(()) => unreachable!(),
    }
}
