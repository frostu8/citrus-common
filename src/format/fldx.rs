//! Support for the `.fldx` format.
//!
//! The functions [`encode()`] and [`decode()`] encode and decode
//! fields in the `.fldx` format, a format deferring from the original `.fld`
//! format. **All integers, unless specified, use Little Endian.**
//!
//! All `.fldx` files start with a four byte header. This header is comprised
//! of two `ushort`s describing the width, and then the height in that order. 
//! Then, a flattened array in row-major order of the field data follows.
//!
//! Each panel in the field data is represented by two bytes. The first byte
//! details the panel's kind [as described in the `.fld` format][1]. The second
//! byte details the panel's directional information [as detailed in the `.fld`
//! format][1].
//!
//! [1]: ../fld/index.html

use super::*;

use crate::{Field, Panel, PanelKind};

use std::io::{Cursor, Read, Write, Error, ErrorKind};
use std::convert::TryFrom as _;

/// Encode a field to the `.fldx` format.
pub fn encode<T>(field: &Field, mut output: T) -> Result<(), Error>
where T: Write {
    // write the size data
    // write width
    write_u16(&mut output, field.width() as u16)?;
    // write height
    write_u16(&mut output, field.height() as u16)?;

    // write data
    for (x, y) in field.row_iter() {
        let panel = field.get(x, y);

        // we can do this because the panel's kind already reflects the OJ
        // format.
        output.write(&[panel.kind.into(), panel.exits_internal()])?;
    }

    Ok(())
}

/// Decode a field from the `.fldx` format.
pub fn decode<T>(mut input: T) -> Result<Field, Error>
where T: Read {
    // read the size data
    // read width
    let width = read_u16(&mut input)? as usize;
    // read height
    let height = read_u16(&mut input)? as usize;

    // read data
    let mut data = Vec::<Panel>::new();

    let mut panel_buf = [0u8; 2];
    
    while input.read(&mut panel_buf)? != 0 {
        let panel_kind = match PanelKind::try_from(panel_buf[0]) {
            Ok(kind) => kind,
            // throw
            Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
        };

        data.push(
            Panel::from_internal(panel_kind, panel_buf[1])
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

/// Encodes a field to a Base64 string.
#[cfg(feature = "base64")]
pub fn encode_base64(field: &Field) -> Result<String, Error> {
    let mut sw = EncoderStringWriter::new(BASE64_CONFIG);

    encode(field, &mut sw)
        .map(|_| sw.into_inner())
}

/// Decodes a field from a Base64 string.
#[cfg(feature = "base64")]
pub fn decode_base64(data: &str) -> Result<Field, Error> {
    let mut cursor = Cursor::new(data);
    let mut sr = DecoderReader::new(&mut cursor, BASE64_CONFIG);

    decode(&mut sr)
}
