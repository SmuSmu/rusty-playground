extern crate winreg;

use winreg::enums::*;

fn bytes_to_hex (input: &winreg::RegValue) -> String
{
    let mut output: String = "".to_string();
    for x in &input.bytes {
        output = format!("{} {}", output, format!("{:02x}", x));
    }
    return output.trim().to_string();
}

fn main() {
    let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);
    // ProductName
    let subkey = hkcu.open_subkey_with_flags(r#"System"#, KEY_READ)
                    .expect("Failed to open subkey");
    let test: winreg::RegValue = subkey.get_raw_value("test")
                    .expect("Failed to read test");
    
    println!("TestKey : {:#?}", test.bytes);

    println!("[{}]", bytes_to_hex(&test));

}