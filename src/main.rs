use std::{env, fs};

fn main() {
    let cwd = env::current_exe().unwrap();
    let bios_target = cwd.with_file_name("bios.img");
    let uefi_target = cwd.with_file_name("uefi.img");

    fs::copy(env!("UEFI_IMAGE"), &uefi_target).unwrap();
    fs::copy(env!("BIOS_IMAGE"), &bios_target).unwrap();

    println!("UEFI disk image at {}", uefi_target.display());
    println!("BIOS disk image at {}", bios_target.display());
}
