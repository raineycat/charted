use std::char;

use binary_rw::{BinaryError, BinaryReader, BinaryWriter};

pub(crate) trait ReadNullTerminatedString<Error> {
    fn read_null_terminated_string(&mut self) -> Result<String, Error>;
}

pub(crate) trait WriteNullTerminatedString<Error> {
    fn write_null_terminated_string(&mut self, s: &str) -> Result<(), Error>;
}

impl ReadNullTerminatedString<BinaryError> for BinaryReader<'_> {
    fn read_null_terminated_string(&mut self) -> Result<String, BinaryError> {
        let mut str = String::new();
        loop {
            match self.read_u8()? {
                0 => break,
                c => str.push(c as char),
            }
        }
        Ok(str)
    }
}

impl WriteNullTerminatedString<BinaryError> for BinaryWriter<'_> {
    fn write_null_terminated_string(&mut self, s: &str) -> Result<(), BinaryError> {
        for c in s.chars() {
            self.write_u8(c as u8)?;
        }
        self.write_u8(0)?;
        Ok(())
    }
}
