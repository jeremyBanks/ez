A collection of easy-to-use, beginner-friendly utilities for Rust.

For simplicity, many functions in this collection may [`panic!`] on error
instead of returning a [`Result`], or be a bit slower than alternatives.
They may not be suitable for production use. They should be useful for
prototyping/scripting, or beginners who want something that "just works"
for the common case so they don't have to deal with full error handling
yet.

This was motivated by my experience working on [Advent of Code 2021][AOC-2021]
problems both in Rust and in Python. I found myself repeating a lot of
boilerplate in Rust for things that often aren't worth worrying about when
trying to write ephemeral code, such as for competitive programming. A few weeks
later [Nick Cameron announced `ezio`][EZIO], a crate with similar goals for IO
in particular, which was encouraging.

[AOC-2021]: https://adventofcode.com/2021
[EZIO]: https://www.ncameron.org/blog/ezio/
