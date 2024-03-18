#![no_std]
#![no_main]
mod framebuffer;

use core::panic::PanicInfo;
use framebuffer::{set_pixel, Color, Position};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn start(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        let green = Color { r: 0, g: 255, b: 0 };
        let blue = Color { r: 0, g: 0, b: 255 };
        let middle = Position {
            x: info.width / 2,
            y: info.height / 2,
        };

        // Draw a grid
        for x in 0..info.width {
            for y in 0..info.height {
                if x % 100 == 0 || y % 100 == 0 {
                    let pos = Position { x, y };
                    set_pixel(framebuffer, &pos, &green);
                }
            }
        }

        // Draw a 20:20 box in the middle
        for x in (middle.x - 10)..(middle.x + 10) {
            for y in (middle.y - 10)..(middle.y + 10) {
                let pos = Position { x, y };
                set_pixel(framebuffer, &pos, &blue);
            }
        }
    }

    loop {}
}

bootloader_api::entry_point!(start);
