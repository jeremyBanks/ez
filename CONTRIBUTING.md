This project isn't ready to take contributions.

## Crate Structure

```mermaid
graph TD;
  subgraph public
    ez
    ez-int
    ezio
  end

  subgraph internal
    ez-core["ez-core<br>(used as <i>ez</i> for macro compatibility)"]
    ez-macro-rules
    ez-proc-macros
  end

  subgraph external
    eyre["eyre<br>&amp; color-eyre"]
    noisy_float
    num-traits["num-traits<br>&amp; num-derive"]
    proc_macro2["proc_macro2<br>&amp; syn<br>&amp; quote"]
    tokio
    tracing["tracing<br>&amp; tracing-subscriber<br>&amp; tracing-error"]
    dotenv
  end

  ez --> ezio
  ez --> ez-core
  ez --> ez-int
  ez --> noisy_float
  ez --> tokio
  tokio --> tracing
  ez --> tracing
  ez-core --> ez-macro-rules
  ez-core --> ez-proc-macros
  ez-core ----> eyre
  ez-int --> ez-core
  ez-int --> num-traits
  ez-proc-macros --> proc_macro2
  ezio --> ez-core
  noisy_float --> num-traits
  ez --> dotenv
```

## Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache License 2.0,
shall be dual licensed as described in [the `LICENSE` file](LICENSE), without
any additional terms or conditions.
