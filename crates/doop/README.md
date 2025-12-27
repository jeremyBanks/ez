# doop

**Pronounced like "dupe", spelled like "loop"**

A Rust macro for local code duplication using loop-style syntax. Doop generates multiple copies of code with different substitutions, making it easy to implement similar functionality for multiple types without manual copy-paste.

## Features

- ✨ Simple, intuitive loop-style syntax
- 🔄 Single variable or tuple destructuring
- 🎯 Works with any Rust items (impls, functions, traits, etc.)
- 🚀 No runtime overhead - purely compile-time code generation

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
doop = "0.1"
```

## Examples

### Basic Type Substitution

Generate trait implementations for multiple types:

```rust
use doop::doop;

trait Double {
    fn double(self) -> Self;
}

doop! {
    for Type in [u8, i16, u32, i64] {
        impl Double for Type {
            fn double(self) -> Type {
                self * 2
            }
        }
    }
}

// Now you can use:
assert_eq!(5u8.double(), 10u8);
assert_eq!(5i16.double(), 10i16);
```

### Tuple Destructuring

Use multiple variables for more complex substitutions:

```rust
use doop::doop;

trait GetValue {
    fn value() -> i32;
}

doop! {
    for (Type, val) in [(u8, 1), (i16, 2), (u32, 3)] {
        impl GetValue for Type {
            fn value() -> i32 {
                val
            }
        }
    }
}

assert_eq!(u8::value(), 1);
assert_eq!(i16::value(), 2);
assert_eq!(u32::value(), 3);
```

### Multiple Variables

```rust
trait Describe {
    fn bits() -> u32;
    fn name() -> &'static str;
}

doop! {
    for (T, bits, name) in [(u8, 8, "byte"), (u16, 16, "word"), (u32, 32, "dword")] {
        impl Describe for T {
            fn bits() -> u32 { bits }
            fn name() -> &'static str { name }
        }
    }
}

assert_eq!(u8::name(), "byte");
assert_eq!(u16::bits(), 16);
```

## How It Works

`doop!` is a procedural macro that:
1. Parses your loop-style syntax
2. For each value in the array, substitutes all occurrences of the variable(s)
3. Generates the expanded code at compile time

The substitution is purely textual at the token level - any identifier matching your variable name will be replaced.

## Comparison with Alternatives

- **`macro_rules!`** - More powerful but complex syntax, steeper learning curve
- **`paste!`** - Only handles identifier concatenation, not full code duplication
- **`duplicate`** - Similar idea but different syntax

`doop` aims to be the simplest and most intuitive option for the common case of "I want this code for multiple types".

## Limitations

- Variables must be simple identifiers (no patterns like `_` or `..`)
- Cannot concatenate identifiers (e.g., `test_Type` won't work - use `Type` standalone)
- All values in the array must have the same structure (single or tuple of same length)

## License

MIT OR Apache-2.0
