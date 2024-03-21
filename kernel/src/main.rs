#![no_std]
#![no_main]
mod framebuffer;
mod logger;

use core::panic::PanicInfo;
use logger::init_logger;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn start(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        init_logger(framebuffer.buffer_mut(), info);
        log::info!("Hello kernel!");
    }

    loop {}
}

bootloader_api::entry_point!(start);
