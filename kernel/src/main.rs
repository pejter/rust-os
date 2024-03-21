#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod framebuffer;
mod interrupt;
mod logger;

use core::panic::PanicInfo;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    mono_font::{self, MonoTextStyle},
    pixelcolor::{Rgb888, RgbColor},
    text::Text,
    Drawable,
};
use logger::init_logger;

use crate::framebuffer::Display;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{}", info);
    loop {}
}

fn start(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    interrupt::init();

    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        // Draw some text
        let mut display = Display::new(framebuffer.buffer_mut(), info);
        display.clear(Rgb888::BLACK).expect("Clear screen");

        let style = MonoTextStyle::new(&mono_font::ascii::FONT_10X20, Rgb888::GREEN);
        let text = Text::new(
            "Hello kernel!",
            Point::new(
                i32::try_from(info.width / 2).unwrap(),
                i32::try_from(info.height / 2).unwrap(),
            ),
            style,
        );
        text.draw(&mut display).expect("Draw text");

        // Init text mode
        init_logger(framebuffer.buffer_mut(), info);
        log::info!("Hello kernel!");
    }

    // Breakpoint
    x86_64::instructions::interrupts::int3();

    // Page fault
    unsafe {
        *(0xdeadbeef as *mut u8) = 10;
    }

    loop {}
}

bootloader_api::entry_point!(start);
