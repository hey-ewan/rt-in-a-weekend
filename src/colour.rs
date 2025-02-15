use crate::vec::Vec3;
use std::io::{self, Write};

pub type Colour = Vec3;

// Function to write colour
pub fn write_colour(out: &mut impl Write, pixel_colour: &Colour) -> io::Result<()> {
    let r = (pixel_colour.x() * 255.999) as i32;
    let g = (pixel_colour.y() * 255.999) as i32;
    let b = (pixel_colour.z() * 255.999) as i32;

    writeln!(out, "{} {} {}\n", r, g, b)
}
