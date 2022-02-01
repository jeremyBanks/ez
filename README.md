A collection of easy-to-use, beginner-friendly utilities for Rust.

For simplicity, many functions in this collection may [`panic`] (crash) on
errors instead of returning a [`Result`], or be a bit slower than alternatives.
They may not be suitable for production use. They should be useful for
prototyping/scripting, or beginners who want something that "just works" for the
common case so they don't have to deal with full error handling yet.
