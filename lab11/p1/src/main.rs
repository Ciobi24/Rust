use std::fs::File;
use std::io::{ Write, Result};

struct MyWriter {
    file: File,
}

impl MyWriter {
    fn new(file: File) -> MyWriter {
        MyWriter { file }
    }
}

impl Write for MyWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut duplicated_buf = Vec::with_capacity(buf.len() * 2);
        for &byte in buf {
            duplicated_buf.push(byte);
            duplicated_buf.push(byte);
        }

        self.file.write_all(&duplicated_buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        self.file.flush()
    }
}

fn main() -> Result<()> {
    let mut writer = MyWriter::new(File::create("a.txt")?);
    writer.write_all(b"abc")?;
    writer.flush()?; 

    Ok(())
}
