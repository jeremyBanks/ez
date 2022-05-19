use doop::doop;

fn main() {
    doop! {
        for N in [1, 2, 3] {
            let n = N;
            println!("{n}");
            println!("{N}");
        }
    }
}
