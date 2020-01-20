
// http://siciarz.net/24-days-rust-winreg/

extern crate winreg;

use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ}; 

fn regreadvalue (regpath: &str, regvalue: &str) {
    let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);

    let subkey = hklm.open_subkey_with_flags(regpath, KEY_READ)
                    .expect("Failed to open subkey");
    let thevalue: String = subkey.get_value(regvalue)
                    .expect("Failed to read product name");
    
    println!("{}\\{}={}", regpath, regvalue, thevalue);
}

fn main() {
    let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    
    // ProductName
    let subkey = hklm.open_subkey_with_flags(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, KEY_READ)
                    .expect("Failed to open subkey");
    let product_name: String = subkey.get_value("ProductName")
                    .expect("Failed to read product name");

    // MachineGuid
    let subkey = hklm.open_subkey_with_flags(r#"SOFTWARE\Microsoft\Cryptography"#, KEY_READ)
                    .expect("Failed to open subkey");
    let machine_guid: String = subkey.get_value("MachineGuid")
                    .expect("Failed to read product name");
    
    println!("Windows ProductName : {}", product_name);
    println!("Windows MachineGuid : {}", machine_guid);

    regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "ProductName");


    println!("Playing with Errors");
    if hklm.open_subkey_with_flags(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, KEY_READ).is_ok()
        {
            println!("No Error");
        }
    else
        {
            println!("Error");
        }
}