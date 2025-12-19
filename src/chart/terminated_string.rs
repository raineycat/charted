use std::char;

use binary_rw::{BinaryError, BinaryReader};

pub(crate) trait ReadTerminatedString<Error> {
    fn read_null_terminated_string(&mut self) -> Result<String, Error>;
}

impl ReadTerminatedString<BinaryError> for BinaryReader<'_> {
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
