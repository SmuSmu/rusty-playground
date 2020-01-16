extern crate winreg;

use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};

fn bytes_to_hex (input: &winreg::RegValue) -> String
{
    let mut output: String = "".to_string();
    for x in &input.bytes {
        output = format!("{} {}", output, format!("{:02X}", x));
    }
    return output;
}

fn main() {
    let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    
    // ProductName
    let subkey = hklm.open_subkey_with_flags(r#"SOFTWARE\Microsoft\Windows Defender"#, KEY_READ)
                    .expect("Failed to open subkey");
    let install_time: winreg::RegValue = subkey.get_raw_value("InstallTime")
                    .expect("Failed to read InstallTime");
    
    println!("InstallTime : {:#?}", install_time.bytes);

    println!("[{}]", bytes_to_hex(&install_time));

}