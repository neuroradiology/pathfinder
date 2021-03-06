/* Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/ */

extern crate euclid;
extern crate memmap;
extern crate pathfinder;

use memmap::{Mmap, Protection};
use pathfinder::charmap::CodepointRange;
use pathfinder::font::{Font, PointKind};
use std::char;
use std::env;

fn main() {
    let file = Mmap::open_path(env::args().nth(1).unwrap(), Protection::Read).unwrap();
    let mut buffer = vec![];
    unsafe {
        let font = Font::new(file.as_slice(), &mut buffer).unwrap();
        let codepoint_ranges = [CodepointRange::new('!' as u32, '~' as u32)];
        let glyph_mapping = font.glyph_mapping_for_codepoint_ranges(&codepoint_ranges).unwrap();
        for (glyph_index, (_, glyph_id)) in glyph_mapping.iter().enumerate() {
            let codepoint = '!' as u32 + glyph_index as u32;
            println!("Glyph {}: codepoint {} '{}':",
                     glyph_id,
                     codepoint,
                     char::from_u32(codepoint).unwrap_or('?'));

            let mut last_point_was_on_curve = false;
            font.for_each_point(glyph_id, |point| {
                let prefix = if point.index_in_contour == 0 {
                    "M "
                } else {
                    match point.kind {
                        PointKind::OnCurve if last_point_was_on_curve => "L ",
                        PointKind::OnCurve => " ",
                        PointKind::QuadControl => "Q ",
                        PointKind::FirstCubicControl => "C ",
                        PointKind::SecondCubicControl => " ",
                    }
                };

                print!("{}{},{}", prefix, point.position.x, point.position.y);

                last_point_was_on_curve = point.kind == PointKind::OnCurve;
                if last_point_was_on_curve {
                    println!("")
                }
            }).unwrap()
        }
    }
}

