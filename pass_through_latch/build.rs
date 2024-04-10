use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_name = Path::new(&manifest_dir)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    println!("cargo:rerun-if-changed=target/{}/{}", target, profile);

    let elf_path = format!("../target/{}/{}/{}.elf", target, profile, project_name);
    let hex_path = format!("../target/{}/{}/{}.hex", target, profile, project_name);

    Command::new("avr-objcopy")
        .args(&["-O", "ihex", "-R", ".eeprom", &elf_path, &hex_path])
        .status()
        .unwrap();
}
