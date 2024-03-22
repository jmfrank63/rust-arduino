use std::process::Command;
use std::env;


fn main() {

    let target = env::var("TARGET").unwrap();
    let profile = env::var("PROFILE").unwrap();
    println!("cargo:rerun-if-changed=target/{}/{}", target, profile);
    let project_name = "rust-arduino"; // replace with your project name

    let elf_path = format!("target/{}/{}/{}.elf", target, profile, project_name);
    let hex_path = format!("target/{}/{}/{}.hex", target, profile, project_name);

    Command::new("avr-objcopy")
        .args(&["-O", "ihex", "-R", ".eeprom", &elf_path, &hex_path])
        .status()
        .unwrap();
}
