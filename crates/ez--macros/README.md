`macro_rules` macros are automatically exported at the top-level of the crate
that they're defined in. `proc-macro`s have to be exported as a special type
of crate that can impair reuse. For that reason, all of our macros are defined
here, so they can be re-exported from `ez` where we actually want them to show
up.
