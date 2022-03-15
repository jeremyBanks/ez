use orerr::{throws, try_throws};

#[throws(struct)]
fn thing() {}
// #[derive(thiserror::Error)]
// struct ThingError;
// Result<(), ThingError>

#[throws(ThingError)]
fn alternate() {
    thing()
}
// Result<(), ThingError>

#[throws(OneError, AnotherError)]
pub fn banjo() {
    thing()
}
// #[derive(thiserror::Error)]
// pub enum BanjoError {
//    OneError(OneError),
//    AnotherError(AnotherError),
// }

#[throws(enum {
    OneError(OneError),
    #[error = "hello whatever"]
    AnotherError(AnotherError),
})]
pub fn bingo() {}
