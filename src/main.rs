use ::easy_as_py::*;

use fstrings::{println_f as printf, format_args_f};

fn main() {
    dotenv::dotenv().ok();
    color_eyre::install().ok();

    let name = input("What is your name?");

    let age = input("How old are you?");
    let age = int(age);

    let age_chars = list(str(age));

    print!("Hello, {name} of {age} years.");

    dbg!(2 / Int(x) * 0.5);
}
