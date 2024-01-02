
#![allow(invalid_value)]

use crate::bindings::*;
use std::mem::MaybeUninit;

macro_rules! uninit {
    ($name: ident, $t: tt) => {
        let mut $name: $t = MaybeUninit::uninit().assume_init();
    };
}

macro_rules! check_err {
    ($err: expr) => {
        if $err != 0 { println!("FreeType is not happy: {}", $err); panic!() };
    };
}

// TEST
pub unsafe fn test() {
    uninit!(lib, FT_Library);
    uninit!(face, FT_Face);

    let error = FT_Init_FreeType(&mut lib);
    check_err!(error);

    let error = FT_New_Face(lib, b"./assets/IBMPlexMono.ttf\0".as_ptr() as *const i8, 0, &mut face);
    check_err!(error);

    // Try to get A
    let error = FT_Load_Char(face, 65, 0 /* Default?? */);
    check_err!(error);

    let error = FT_Render_Glyph((*face).glyph, 0 /* Default?? */);
    check_err!(error);

    let buffer = (*(*face).glyph).bitmap.buffer;
    let size_x = (*(*face).glyph).bitmap.width;
    let size_y = (*(*face).glyph).bitmap.rows;
    
    for y in 0..size_y {
        for x in 0..size_x {
            let ptr = buffer.add((x + y * size_x) as usize);

            if *ptr < (1 << 3) {
                print!(" ");
            }
            else if *ptr < (1 << 4) {
                print!("o");
            }
            else {
                print!("#");
            }

        }
        println!("");
    }
}

