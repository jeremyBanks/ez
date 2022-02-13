This project isn't really in a state where it's ready to take contributions.

## Crate Structure

Why is this project split up into several crates? Because that's neccessary in
order to have full control of the public interface given the use of macros.

Macros created with `#[macro_export] macro_rules` are automatically exported at
the top level of the crate where they're defined. We define them in
`ez_internal` so that `ez` can choose where to re-export them publicly.

Procedural macros must be defined in their own crate, from which they must be
exported at the top level. We define our procedural macro's internal logic in
`ez_internal` using `proc_macro2` so that it's easier to reuse and test. We
actually export them as procedural macros through the `ez_proc_macro` crate, and
`ez` can choose where to re-export them publicly.

Examples are included in `./crates/ez/examples`, but when you run examples
locally they're actually run through `./crates/ez-examples/examples`, which is a
symlink. This is because if we run the examples in the crate they're defined in,
they can access that crates's dependencies directly, but we want to make sure
our examples are exercising our macros in a more accurate, hygienic context, so
we need to run them through a separate crate. However, the macros need to live
inside the `ez` crate because they're shipped with it so they can be included in
the generated documentation.

Some of our macros need to refer to third-party crates, such as `tokio`.
However, if the macros are being used in a project that doesn't depend on
`tokio` itself, `::tokio` won't be available in the global namespace for us to
reference. Procedural macros are limited in what they can reliably refer to. In
our case, we require the users to import our macros through the `ez` crate, so
that's the only thing we can be (relatively) certain is available. So anything
that's internally required by our macros, but isn't really supposed to be part
of our crate's public interface, is re-exported under the
hidden-from-documentation `ez::__` module. For example, a macro may refer to the
`tokio` crate as `::ez::__::tokio`.

## Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache License 2.0,
shall be dual licensed as described in [the `LICENSE` file](LICENSE), without
any additional terms or conditions.
