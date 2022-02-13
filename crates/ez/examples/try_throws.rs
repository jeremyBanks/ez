#[ez::try_throws]
fn something_that_fails() {
    throw!("oh no!");
}

fn main() {
    match try_something_that_fails() {
        Err(_err) => something_that_fails(),
        Ok(()) => unreachable!(),
    }
}
