Returns from the enclosing function with an error value or error message.

If called with an error value, the macro expansion works like this:

```ignore
throw!(err);
```

```ignore
return Result::Err(err);
```

If called with a string literal as the first argument, the macro expansion works
like this:

```ignore
throw!("the value {:?} is out of range", &x.inner);
```

```ignore
return Result::Err(eyre::Report::msg(
    format!("the value {:?} is out of range", &x.inner)
));
```

This macro is automatically imported into the body of functions using
[`#[throws]`][crate::throws] and related macros, and is the intended way of
returning a new error value from such functions. However, it is also appropriate
for use elsewhere.
