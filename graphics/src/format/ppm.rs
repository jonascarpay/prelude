use std::{
    fs::File,
    io::{BufWriter, Write},
};

use prelude::algebra::V2;

use crate::color::srgb::Srgb;

pub fn write_ppm(path: &str, size: V2<usize>, pixels: &[Srgb]) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut buf = BufWriter::new(file);
    write!(buf, "P6\n{} {}\n255\n", size.x, size.y)?;
    for p in pixels {
        buf.write_all(&p.pack())?;
    }
    buf.flush()?;
    Ok(())
}
