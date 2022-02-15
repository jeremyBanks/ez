use crate::{Read, Write};
use eyre::Context;
use ez::throws;

/// Read a single line from stdin.
pub fn read_line() -> String {
    stdin().read_line()
}

/// Print a string to stdout.
pub fn print(s: &str) {
    stdout().write_str(s)
}

/// Print a string to stderr.
pub fn eprint(s: &str) {
    stderr().write_str(s)
}

/// Get a handle to stdout.
pub fn stdout() -> Stdout {
    Stdout(std::io::stdout())
}

/// Get a handle to stderr.
pub fn stderr() -> Stderr {
    Stderr(std::io::stderr())
}

/// Get a handle to stdin.
pub fn stdin() -> Stdin {
    Stdin(std::io::stdin())
}

/// A handle to stdout.
#[derive(Debug)]
pub struct Stdout(std::io::Stdout);

/// A handle to stderr.
#[derive(Debug)]
pub struct Stderr(std::io::Stderr);

/// A handle to stdin.
#[derive(Debug)]
pub struct Stdin(std::io::Stdin);

impl_inherent_write!(Stdout);

impl_inherent_write!(Stderr);

impl_inherent_read!(Stdin);

impl Write for Stdout {
    #[throws]
    fn try_write_str(&mut self, s: &str) {
        use std::io::Write;

        self.0
            .write_all(s.as_bytes())
            .wrap_err("Failed to write to stdout")?;
    }
}

impl std::io::Write for Stdout {
    fn write(&mut self, b: &[u8]) -> Result<usize, std::io::Error> {
        self.0.write(b)
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        self.0.flush()
    }
}

impl Write for Stderr {
    #[throws]
    fn try_write_str(&mut self, s: &str) {
        use std::io::Write;

        self.0
            .write_all(s.as_bytes())
            .wrap_err("Failed to write to stdout")?;
    }
}

impl std::io::Write for Stderr {
    fn write(&mut self, b: &[u8]) -> Result<usize, std::io::Error> {
        self.0.write(b)
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        self.0.flush()
    }
}

impl Read for Stdin {
    #[throws]
    fn try_read_all(&mut self) -> String {
        use std::io::Read;

        let mut buf = String::new();
        self.0
            .read_to_string(&mut buf)
            .wrap_err("Failed to read from stdin")?;
        buf
    }

    #[throws]
    fn try_read_line(&mut self) -> String {
        let mut buf = String::new();
        self.0
            .read_line(&mut buf)
            .wrap_err("Failed to read from stdin")?;

        if !buf.is_empty() {
            buf.truncate(buf.len() - 1);
        }

        buf
    }
}

impl std::io::Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

read_into_iter!(Stdin);

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn stdout_write() {
        stdout().write_str("Hello!\n");
    }

    #[test]
    fn stderr_write() {
        stderr().write_str("Hello!\n");
    }

    #[test]
    fn stdout_writeln() {
        use std::io::Write;

        let mut out = stdout();
        writeln!(out, "World!").unwrap();
    }
}
