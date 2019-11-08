#[cfg(feature = "std")]
use std::mem::size_of_val;
#[cfg(feature = "std")]
use std::char::from_u32_unchecked;

use glyph::Glyph;

pub fn get_glyph(c: char) -> Option<&'static Glyph> {
    let code_point = c as usize;
    let mut offset: usize = 0;
    let mut result = None;
    for (start, end) in CODE_POINT_RANGES.iter() {
         if *start <= code_point && code_point < *end {
             result = Some(&GLYPH_TABLE[offset + code_point - start]);
             break;
         } else {
             offset += end - start;
         }
    }
    result
}

#[cfg(feature = "std")]
pub fn enumerate_glyphs() -> Box<Iterator<Item=(char, &'static Glyph)>> {
    let char_iterator = CODE_POINT_RANGES.iter()
        .flat_map(|(start, end)| *start..*end)
        .map(|code_point| unsafe { from_u32_unchecked(code_point as u32) });
    let glyph_iterator = GLYPH_TABLE.iter();
    Box::new(char_iterator.zip(glyph_iterator))
}

#[cfg(feature = "std")]
pub fn get_storage_size() -> usize {
    size_of_val(&CODE_POINT_RANGES) + size_of_val(&GLYPH_TABLE)
}


include!(concat!(env!("OUT_DIR"), "/glyph_table.rs"));

#[cfg(test)]
mod tests {
    use testutil;
    use super::*;

    #[test]
    fn glyph_a() {
        let glyph = get_glyph('a').unwrap();
        assert_eq!(glyph, &testutil::GLYPH_A);
    }

    #[test]
    fn glyph_ji() {
        let glyph = get_glyph('字').unwrap();
        assert_eq!(glyph, &testutil::GLYPH_JI);
    }

    #[test]
    fn enumeration() {
        let glyph_a = get_glyph('a').unwrap();
        let glyph_ji = get_glyph('字').unwrap();
        for (c, glyph) in enumerate_glyphs() {
            match c {
                'a' => assert_eq!(glyph, glyph_a),
                '字' => assert_eq!(glyph, glyph_ji),
                _ => {},
            }
        }
    }
}
