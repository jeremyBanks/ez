A silly Python-inspired prelude for Rust, full of implicit magic that
infects whatever it touches. _Hic sunt dracones._ 🐉

```rust
#[no_implicit_prelude]
use ::easy_as_py::*;

fn main() {
  let name = input("what is your name?");
  let age = input("how old are you?");
  let age = float(age);
  let decades = int(age / 10);

  printf!("Wow, {name}, you've been around for {decades} decades!");

  let d = dict!{ 1: "hello", 2: "bravo" };
  let s = set!{ 1, 2, 3, 4 };
  let l = list![ 1, 2, 3, 4];

  for i, n in enumerate(range(1..10)) {

  }
}
```

```sh
(cargo +nightly fmt && echo && cargo run 2>&1) | less -reFX
```
