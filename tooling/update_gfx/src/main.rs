//Read in the png and output the palletted data as a text array
extern crate png;

use std::fs::File;
use std::io::prelude::*;

const IMAGE_FILENAME: &'static str = "../../assets/gfx.png";
// for testing
// const IMAGE_FILENAME: &'static str = "assets/pallete.png";

fn main() -> Result<(), Box<std::error::Error>> {
    let decoder = png::Decoder::new(File::open(IMAGE_FILENAME)?);
    let (info, mut reader) = decoder.read_info()?;
    println!(
        "{:?}",
        (
            info.width,
            info.height,
            info.color_type,
            info.bit_depth,
            info.line_size
        )
    );
    // Allocate the output buffer.
    let mut buf = vec![0; info.buffer_size()];
    // Read the next frame. Currently this function should only called once.
    // The default options
    reader.next_frame(&mut buf)?;

    let mut file = File::create("out.txt")?;

    use png::ColorType::*;
    let pixel_width = match info.color_type {
        RGB => 3,
        RGBA => 4,
        _ => unimplemented!(
            "This program cannot handle {:?} images (yet.)",
            info.color_type
        ),
    };

    let mut output = Vec::with_capacity(buf.len() / pixel_width);

    for colour in buf.chunks(pixel_width) {
        let index = match (colour[0], colour[1], colour[2]) {
            (51, 82, 225) => 0,
            (48, 176, 110) => 1,
            (222, 73, 73) => 2,
            (255, 185, 55) => 3,
            (83, 51, 84) => 4,
            (90, 125, 139) => 5,
            (238, 238, 238) => 6,
            (34, 34, 34) => 7,
            _ => 255,
        };

        output.push(index);
    }

    file.write_all(&format!("{:?}", output).as_bytes())?;

    Ok(())
}
