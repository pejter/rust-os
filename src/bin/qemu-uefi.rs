use std::process::{self, Command};

fn main() {
    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.arg("-drive")
        .arg(format!("format=raw,file={}", env!("UEFI_IMAGE")));
    cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
    let exit_code = cmd.status().expect("QEMU executes");
    process::exit(exit_code.code().unwrap());
}
