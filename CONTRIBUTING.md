This project isn't ready to take contributions.

## Crate Structure

```mermaid
graph TD;
  subgraph public[" "]
    ez
    ez-int
    ezio
  end

  ezio -.-> ez-int
  ez ---> ez-core
  ez -.-> ez-int
  ez -.-> ezio
  ez -...-> ez-batteries
  ez -...-> ez-main
  ezio --> ez-core
  ez-core --> ez-macro-rules
  ez-core --> ez-proc-macro
  ez-int --> ez-core
  ez-main --> ez-core
```

## Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache License 2.0,
shall be dual licensed as described in [the `LICENSE` file](LICENSE), without
any additional terms or conditions.
