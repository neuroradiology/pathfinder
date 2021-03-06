// Copyright 2017 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use byteorder::{BigEndian, ReadBytesExt};
use error::FontError;
use font::FontTable;
use outline::GlyphBounds;
use std::mem;
use util::Jump;

pub const TAG: u32 = ((b'h' as u32) << 24) |
                      ((b'e' as u32) << 16) |
                      ((b'a' as u32) << 8)  |
                       (b'd' as u32);

const MAGIC_NUMBER: u32 = 0x5f0f3cf5;

#[derive(Clone, Debug)]
pub struct HeadTable {
    pub units_per_em: u16,
    pub index_to_loc_format: i16,
    pub max_glyph_bounds: GlyphBounds,
}

impl HeadTable {
    pub fn new(table: FontTable) -> Result<HeadTable, FontError> {
        let mut reader = table.bytes;

        // Check the version.
        let major_version = try!(reader.read_u16::<BigEndian>().map_err(FontError::eof));
        let minor_version = try!(reader.read_u16::<BigEndian>().map_err(FontError::eof));
        if (major_version, minor_version) != (1, 0) {
            return Err(FontError::UnsupportedHeadVersion)
        }

        // Check the magic number.
        try!(reader.jump(mem::size_of::<u32>() * 2).map_err(FontError::eof));
        let magic_number = try!(reader.read_u32::<BigEndian>().map_err(FontError::eof));
        if magic_number != MAGIC_NUMBER {
            return Err(FontError::UnknownFormat)
        }

        // Read the units per em.
        try!(reader.jump(mem::size_of::<u16>()).map_err(FontError::eof));
        let units_per_em = try!(reader.read_u16::<BigEndian>().map_err(FontError::eof));

        // Read the maximum bounds.
        try!(reader.jump(mem::size_of::<i64>() * 2).map_err(FontError::eof));
        let x_min = try!(reader.read_i16::<BigEndian>().map_err(FontError::eof));
        let y_min = try!(reader.read_i16::<BigEndian>().map_err(FontError::eof));
        let x_max = try!(reader.read_i16::<BigEndian>().map_err(FontError::eof));
        let y_max = try!(reader.read_i16::<BigEndian>().map_err(FontError::eof));
        let max_glyph_bounds = GlyphBounds {
            left: x_min as i32,
            bottom: y_min as i32,
            right: x_max as i32,
            top: y_max as i32,
        };

        // Read the index-to-location format.
        try!(reader.jump(mem::size_of::<u16>() * 2 + mem::size_of::<i16>())
                   .map_err(FontError::eof));
        let index_to_loc_format = try!(reader.read_i16::<BigEndian>().map_err(FontError::eof));

        // Check the glyph data format.
        let glyph_data_format = try!(reader.read_i16::<BigEndian>().map_err(FontError::eof));
        if glyph_data_format != 0 {
            return Err(FontError::UnsupportedGlyphFormat)
        }

        Ok(HeadTable {
            units_per_em: units_per_em,
            index_to_loc_format: index_to_loc_format,
            max_glyph_bounds: max_glyph_bounds,
        })
    }
}
