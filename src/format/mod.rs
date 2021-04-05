//! Tools to encode and decode field data to binary representations.
//!
//! * [`fldx`]: the community `.fldx` format, with support for dynamic width
//!   and height values.
//! * [`fld`]: 100% OJ's own `.fld` format.

pub mod fldx;
pub mod fld;

use std::io::{Read, Write, Error, ErrorKind};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[cfg(feature = "base64")]
use base64::{Config, CharacterSet};

#[cfg(feature = "base64")]
const BASE64_CONFIG: Config = Config::new(CharacterSet::UrlSafe, true);

/// An error that indicates an invalid size of the input data.
#[derive(Debug)]
pub struct InvalidSize {
    pub expected: usize,
    pub got: usize
}

impl InvalidSize {  
    pub const fn new(expected: usize, got: usize) -> InvalidSize {
        InvalidSize { expected, got }
    }
}

impl Display for InvalidSize {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f, "invalid size of data, expected {} bytes, got {} bytes",
            self.expected, self.got
        )
    }
}

impl std::error::Error for InvalidSize { }

fn read_u16<T>(mut input: T) -> Result<u16, Error> 
where T: Read {
    let mut num_buf = [0u8; 2];
    
    if input.read(&mut num_buf)? < 2 {
        Err(Error::new(ErrorKind::UnexpectedEof, "unexpected end of file"))
    } else {
        Ok(u16::from_le_bytes(num_buf))
    }
}

fn write_u16<T>(mut output: T, data: u16) -> Result<(), Error> 
where T: Write {
    output.write(&data.to_le_bytes())?;
    Ok(())
}

