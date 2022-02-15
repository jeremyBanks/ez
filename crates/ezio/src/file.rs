use crate::{Read, Write};
use eyre::WrapErr;
use ez::{throws, try_throws};
use std::path::Path;

/// Create an object to read from a file.
///
/// The `path` argument specifies the path to the file to read from. It may be
/// either a `Path`, a string, or any other type that can be converted (using
/// `AsRef`) to a `Path`. The file must already exist.
///
/// Panics if there is no file at `path`, or the file cannot be opened.
#[try_throws]
pub fn reader(path: impl AsRef<Path>) -> Reader {
    Reader(std::io::BufReader::new(
        std::fs::File::open(path).wrap_err("Couldn't open file")?,
    ))
}

/// Create an object to write to a file.
///
/// The `path` argument specifies the path to the file to write to. It may be
/// either a `Path`, a string, or any other type that can be converted (using
/// `AsRef`) to a `Path`. If the file does not exist, it will be created.
///
/// Panics if the file cannot be opened or created.
#[try_throws]
pub fn writer(path: impl AsRef<Path>) -> Writer {
    Writer(std::fs::File::create(path).wrap_err("Couldn't create file")?)
}

/// Read and return a whole file.
///
/// The `path` argument specifies the path to the file to read from. It may be
/// either a `Path`, a string, or any other type that can be converted (using
/// `AsRef`) to a `Path`. The file must already exist.
///
/// Panics if there is no file at `path`, or the file cannot be opened.
#[try_throws]
pub fn read(path: impl AsRef<Path>) -> String {
    reader(path).try_read_all()?
}

/// Write a string to a file.
///
/// The `path` argument specifies the path to the file to write to. It may be
/// either a `Path`, a string, or any other type that can be converted (using
/// `AsRef`) to a `Path`. If the file does not exist, it will be created.
///
/// Panics if the file cannot be opened or created.
#[try_throws]
pub fn write_str(path: impl AsRef<Path>, s: &str) {
    writer(path).try_write_str(s)?
}

/// An object for writing to a file.
pub struct Writer(std::fs::File);

/// An object for reading from a file.
pub struct Reader(std::io::BufReader<std::fs::File>);

impl_inherent_read!(Reader);

impl_inherent_write!(Writer);

impl Write for Writer {
    #[throws]
    fn try_write_str(&mut self, s: &str) {
        use std::io::Write;

        self.0
            .write_all(s.as_bytes())
            .wrap_err("Failed to write to stdout")?;
    }
}

impl std::io::Write for Writer {
    fn write(&mut self, b: &[u8]) -> Result<usize, std::io::Error> {
        self.0.write(b)
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        self.0.flush()
    }
}

impl Read for Reader {
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
        use std::io::BufRead;

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

impl std::io::Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

read_into_iter!(Reader);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn write_read() {
        write_str("test.out", "line 1\nline 2 \nline 3\n");
        assert_eq!(read("test.out"), "line 1\nline 2 \nline 3\n");

        let mut reader = reader("test.out");
        assert_eq!(reader.read_line(), "line 1");
        assert_eq!(reader.read_line(), "line 2 ");
        assert_eq!(reader.read_line(), "line 3");
    }
}
