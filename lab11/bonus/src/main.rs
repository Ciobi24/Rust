use std::io::{self, Write, Result};

struct MyWriter<W: Write> {
    inner: W,
}

impl<W: Write> MyWriter<W> {
    fn new(inner: W) -> MyWriter<W> {
        MyWriter { inner }
    }
}

impl<W: Write> Write for MyWriter<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut duplicated_buf = Vec::with_capacity(buf.len() * 2);
        for &byte in buf {
            duplicated_buf.push(byte);
            duplicated_buf.push(byte);
        }

        self.inner.write_all(&duplicated_buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}

fn main() -> Result<()> {
    let mut writer = MyWriter::new(io::stdout());

    writer.write_all(b"abc")?;

    writer.flush()?;
    Ok(())
}
