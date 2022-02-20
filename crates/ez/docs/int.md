This crate an `Int` and `Float` type that attempt to "just work" in a
weakly-typed style, automatically coercing to and from other primitive numeric
types where losslessly possible, but may panic at runtime if some attempted
conversion is not possible. These have some overhead and will be slower than
primitive integer operations, but they may be suitable for non-CPU-bound
prototyping or scripting. We also provide corresponding [`int()`] and
[`float()`] functions which will lossily convert or parse their argument with
similar semantics to Python's [`int()`][PY-INT] and [`float()`][PY-FLOAT]
functions. We rely on [::num-traits] for many method implementations, with our
own macros to generate additional coercions.

```skip
let x = [1, 2, 3];
let index = ez::int("3");
println!("{}", x[index]); // it works!
```

```compile_fail
let x = [1, 2, 3];
let index = 3u64;
println!("{}", x[index]);
//~^ the type `[{integer}]` cannot be indexed by `u64`
```

This was motivated by my experience working on [Advent of Code 2021][AOC-2021]
problems both in Rust and in Python. I found myself repeating a lot of
boilerplate in Rust for things that often aren't worth worrying about when
trying to write ephemeral code, such as for competitive programming. A few weeks
later [Nick Cameron announced `ezio`][EZIO], a crate with similar goals for file
IO, which was encouraging.

[`Int`] isn't an unbounded "big" integer like in Python. However, it is an
[`i128`], so it can fit a value from any of the widely-used integer sizes (if we
used `[i64]` we couldn't fit `[u64]`), and the result of almost any operation
operation on two 64-bit values without overflowing. Operations are checked: if
they overflow, we panic.

[`Float`] is an [`f64`], as that's the largest widely-used float size and the
largest type natively supported by Rust.

[AOC-2021]: https://adventofcode.com/2021
[EZIO]: https://www.ncameron.org/blog/ezio/
[PY-FLOAT]: https://docs.python.org/3/library/functions.html#float
[PY-INT]: https://docs.python.org/3/library/functions.html#int
