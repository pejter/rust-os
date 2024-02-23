use std::process::{self, Command};

fn main() {
    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.args(["-drive", &format!("format=raw,file={}", env!("BIOS_IMAGE"))]);
    let exit_code = cmd.status().expect("QEMU executes");
    process::exit(exit_code.code().unwrap());
}
