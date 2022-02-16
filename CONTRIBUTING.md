This project isn't ready to take contributions.

## Crate Structure

```mermaid
graph TD;
  subgraph public crates
    ez((ez))
    ez-int((ez-int))
    ezio((ezio))
  end

  subgraph internal crates
    ez-main
    ez-core
    ez-macro-rules
    ez-proc-macro
  end

  subgraph external crates
    rand(rand)
    eyre(eyre)
    color-eyre(color-eyre)
    num-traits("num-traits<br>&amp; num-derive")
    proc_macro2("proc_macro2<br>&amp; syn &amp; quote")
    tokio(tokio)
    tracing("tracing<br>&amp; tracing-subscriber<br>&amp; tracing-error")
    dotenv(dotenv)
  end

  ez ----> ez-main
  ez ----> ez-core
  ez --> ezio

  ez-int -..-> ez-core
  ez-int ----> num-traits

  ez-main -...-> tracing
  ez-main -....-> dotenv
  ez-main ---> tokio
  ez-main -..-> color-eyre
  ez-main --> ez-core

  ez-core --> eyre
  ez-core -.-> ez-macro-rules
  ez-core -.-> ez-proc-macro

  color-eyre --> eyre
  ez-proc-macro ---> proc_macro2
  ezio -..-> ez-core
  ezio --> ez-int
  ezio -..-> rand
```

## Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache License 2.0,
shall be dual licensed as described in [the `LICENSE` file](LICENSE), without
any additional terms or conditions.
