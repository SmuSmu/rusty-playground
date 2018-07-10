extern crate winreg;
use winreg::RegKey;
use winreg::enums::*;

fn main() {
    println!("uninstall keys in system (x64):");

    let reg_key_uninstall = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags(r#"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall"#, KEY_READ)
        .unwrap();
    for name in reg_key_uninstall.enum_keys().map(|x| x.unwrap()) {
        println!("{}", name);
    }

    println!("\n\nuninstall keys in system (x86):\n");

    let reg_key_uninstall = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags(r#"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall"#, KEY_READ)
        .unwrap();
    for name in reg_key_uninstall.enum_keys().map(|x| x.unwrap()) {
        println!("{}", name);
    }

}
