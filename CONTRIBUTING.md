This project isn't ready to take contributions.

## Crate Structure

```mermaid
graph TD;
  subgraph public interfaces
    ez((ez))
    ez-int((ez-int))
    ezio((ezio))
  end

  subgraph internal dependencies
    ez-main
    ez-core
    ez-macro-rules
    ez-proc-macros
  end

  subgraph direct external dependencies
    rand(rand)
    eyre(eyre)
    color-eyre(color-eyre)
    noisy_float(noisy_float)
    num-traits("num-traits<br>&amp; num-derive")
    proc_macro2("proc_macro2<br>&amp; syn<br>&amp; quote")
    tokio(tokio)
    tracing("tracing<br>&amp; tracing-subscriber<br>&amp; tracing-error")
    dotenv(dotenv)
  end

  ez ----> ez-main
  ez ---> ezio
  ez ----> ez-core
  ez ---> ez-int
  ez ------> noisy_float

  ez-int --> ez-core
  ez-int ----> num-traits

  ez-main ----> dotenv
  ez-main ----> tracing
  ez-main ----> tokio
  ez-main ----> color-eyre
  ez-main --> ez-core

  ez-core --> eyre
  ez-core --> ez-macro-rules
  ez-core --> ez-proc-macros


  ez-proc-macros --> proc_macro2
  ezio --> ez-core
  noisy_float --> num-traits
  tokio --> tracing
  ezio --> rand
  color-eyre --> eyre
```

## Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache License 2.0,
shall be dual licensed as described in [the `LICENSE` file](LICENSE), without
any additional terms or conditions.
