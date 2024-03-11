#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn start(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let step = framebuffer.info().bytes_per_pixel;
        for byte in framebuffer.buffer_mut().into_iter().step_by(step) {
            *byte = 0xFF;
        }
    }

    loop {}
}

bootloader_api::entry_point!(start);
