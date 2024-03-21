#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod framebuffer;
mod interrupt;
mod logger;

use core::panic::PanicInfo;
use interrupt::init_idt;
use logger::init_logger;

use crate::framebuffer::Display;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{}", info);
    loop {}
}

fn start(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    init_idt();

    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        #[cfg(not(visual))]
        // Text mode
        let info = framebuffer.info();
        init_logger(framebuffer.buffer_mut(), info);
        log::info!("Hello kernel!");

        #[cfg(visual)]
        // Visual mode
        let display = Display::new(framebuffer);
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
