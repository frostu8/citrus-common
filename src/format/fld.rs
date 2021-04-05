//! Support for the official `.fld` format.
//!
//! Field files consist of a flattened 2D array of panels, where each panel 
//! consists of 8 bytes. 
//!
//! Each panel can be further broken down into two 4 byte integers, but the
//! only data that matters is the first byte of each 4 byte integer, and will
//! be referred to as such. This might have been a limitation of whatever 
//! encode/decode method Fruitbat Factory used.
//!
//! The first byte encodes the type of panel a panel is. A zero 
//! indicates that a tile is not present.
//!
//! The second byte encodes the panel's exit information as two sets of
//! bitflags. The first set of bitflags encodes the exits for a tile when 
//! **Backtrack** is enabled, and the other set of bitflags encodes the exits 
//! when Backtrack isn't enabled.
//!
//! Each set of bitflags encode the directions South, East, North, and West
//! from left to right as single bits.
//!
//! For more information, visit the [original definition of the `.fld` 
//! format][1]
//!
//! [1]: https://100orangejuice.fandom.com/wiki/User:Fr0stbytes/sandbox1

use super::*;

use crate::{Field, Panel, PanelKind};

use std::io::{Read, Write, Error, ErrorKind};
use std::convert::TryFrom as _;

/// A square field with the dimensions `15x15`.
///
/// Applies to Training Program,
pub const S15: (usize, usize) = (15, 15);

/// Encode a field to the `.fld` format.
///
/// If successful, returns a tuple of the field's dimensions.
pub fn encode<T>(field: &Field, mut output: T) -> Result<(usize, usize), Error>
where T: Write {
    // encode the field data
    for (x, y) in field.row_iter() {
        let panel = field.get(x, y);

        output.write(&[
            panel.kind.into(), 0, 0, 0,
            panel.exits_internal(), 0, 0, 0,
        ])?;
    }

    Ok((field.width(), field.height()))
}

/// Decode a field from the `.fld` format.
///
/// Requires a width and height, as the `.fld` format does not contain this
/// data. Uses a tuple, so constants can be defined and use for different field 
/// dimensions.
pub fn decode<T>(dims: (usize, usize), mut input: T) -> Result<Field, Error>
where T: Read {
    let (width, height) = dims;

    // read data
    let mut data = Vec::<Panel>::new();

    let mut panel_buf = [0u8; 8];
    
    while input.read(&mut panel_buf)? != 0 {
        let panel_kind = match PanelKind::try_from(panel_buf[0]) {
            Ok(kind) => kind,
            // throw
            Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
        };

        data.push(
            Panel::from_internal(panel_kind, panel_buf[4])
        );
    }
    
    // verify we can make a field from this
    if data.len() == width * height {
        Ok(Field::new_vec(data, width, height))
    } else {
        Err(Error::new(
            ErrorKind::InvalidData, 
            InvalidSize::new(width * height, data.len()),
        ))
    }
}

#[cfg(feature = "base64")]
use base64::{
    write::EncoderStringWriter,
    read::DecoderReader,
};
#[cfg(feature = "base64")]
use std::io::Cursor;

/// Encodes a field to a Base64 string.
#[cfg(feature = "base64")]
pub fn encode_base64(field: &Field) -> Result<String, Error> {
    let mut sw = EncoderStringWriter::new(BASE64_CONFIG);

    encode(field, &mut sw)
        .map(|_| sw.into_inner())
}

/// Decodes a field from a Base64 string.
///
/// Requires a width and height, as the `.fld` format does not contain this
/// data. Uses a tuple, so constants can be defined and use for different field 
/// dimensions.
#[cfg(feature = "base64")]
pub fn decode_base64(dims: (usize, usize), data: &str) -> Result<Field, Error> {
    let mut cursor = Cursor::new(data);
    let mut sr = DecoderReader::new(&mut cursor, BASE64_CONFIG);

    decode(dims, &mut sr)
}
