use bootloader_api::info::{FrameBuffer, PixelFormat};

#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn set_pixel(fb: &mut FrameBuffer, pos: &Position, color: &Color) {
    let info = fb.info();

    let pixel_offset = {
        let row_offset = info.stride * pos.y;
        row_offset + pos.x
    };
    let pixel_start = pixel_offset * info.bytes_per_pixel;
    let pixel_end = pixel_start + info.bytes_per_pixel;
    let pixel_bytes = &mut fb.buffer_mut()[pixel_start..pixel_end];

    match info.pixel_format {
        PixelFormat::Bgr => {
            pixel_bytes[0] = color.b;
            pixel_bytes[1] = color.g;
            pixel_bytes[2] = color.r;
        }
        PixelFormat::Rgb => {
            pixel_bytes[0] = color.r;
            pixel_bytes[1] = color.g;
            pixel_bytes[2] = color.b;
        }
        PixelFormat::U8 => {
            // Convert to grayscale using the Average method, since the Luminosity method requires the Mul trait.
            // https://www.baeldung.com/cs/convert-rgb-to-grayscale#2-average-method
            let grey = (color.r + color.g + color.b) / 3;
            pixel_bytes[0] = grey;
        }
        unknown => panic!("Unknown pixel format {unknown:?}"),
    }
}
