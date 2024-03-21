use core::convert::Infallible;

use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{Rgb888, RgbColor},
    Pixel,
};
pub struct Display<'f> {
    fb: &'f mut FrameBuffer,
}

impl<'f> Display<'f> {
    pub fn new(fb: &'f mut FrameBuffer) -> Self {
        Display { fb }
    }

    fn draw_pixel(&mut self, Pixel(pos, color): Pixel<Rgb888>) {
        let info = self.fb.info();
        let x = usize::try_from(pos.x).unwrap();
        let y = usize::try_from(pos.y).unwrap();

        if (0..info.width).contains(&x) && (0..info.height).contains(&y) {
            set_pixel(
                self.fb,
                &Position { x, y },
                &Color {
                    r: color.r(),
                    g: color.g(),
                    b: color.b(),
                },
            )
        }
    }
}

impl DrawTarget for Display<'_> {
    type Color = Rgb888;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            self.draw_pixel(pixel)
        }

        Ok(())
    }
}

impl OriginDimensions for Display<'_> {
    fn size(&self) -> embedded_graphics::prelude::Size {
        let info = self.fb.info();
        Size::new(
            info.width.try_into().unwrap(),
            info.height.try_into().unwrap(),
        )
    }
}

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
