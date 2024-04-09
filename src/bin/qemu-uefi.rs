use std::process::{self, Command};

fn main() {
    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.args(["-serial", "stdio"]);
    cmd.args(["-drive", &format!("format=raw,file={}", env!("UEFI_IMAGE"))]);
    cmd.args(["-bios", ovmf_prebuilt::ovmf_pure_efi().to_str().unwrap()]);
    let status = cmd.status().expect("QEMU failed executing");
    process::exit(status.code().unwrap());
}
