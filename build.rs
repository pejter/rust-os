use std::{env, path::PathBuf};

use bootloader::DiskImageBuilder;

fn main() {
    let kernel_path = env::var("CARGO_BIN_FILE_KERNEL").expect("kernel env var present");
    let package_name = env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME present");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR present"));

    let bios_path = out_dir.join(format!("{}-bios.img", package_name));
    let uefi_path = out_dir.join(format!("{}-uefi.img", package_name));

    let builder = DiskImageBuilder::new(kernel_path.into());

    builder.create_bios_image(&bios_path).expect("BIOS image");
    builder.create_uefi_image(&uefi_path).expect("UEFI image");

    println!("cargo:rustc-env=BIOS_IMAGE={}", bios_path.display());
    println!("cargo:rustc-env=UEFI_IMAGE={}", uefi_path.display());
}
