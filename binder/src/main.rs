use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    // type cli mode
    if args.len() == 0 {
        println!("Any file binder.exe");
        println!("");

        loop {
            std::io::stdout().write(b"input your file path (exit-q) : ");
            std::io::stdout().flush();

            let mut path = String::new();
            std::io::stdin().read_line(&mut path);
            path = path.trim().to_string();

            if path.trim() == "q" {
                break
            }

            args.push(path.replace("\"", ""));
        }

        println!("");
        std::io::stdout().write(b"output : ");
        std::io::stdout().flush();
        let mut output = String::new();
        std::io::stdin().read_line(&mut output);
        output = output.trim().to_string();

        args.push("-build".into());
        args.push(output);
    }

    let mut file = std::fs::read("stub.bd").unwrap();
    let mut build_index = Arc::new(Mutex::new(0usize));
    for path in args.iter().enumerate() {
        let index = path.0;
        let path = PathBuf::from(path.1);

        if path.starts_with("-build") {
            *build_index.lock().unwrap() = index + 1;
            break
        }

        let name_ext: Vec<u8> = base64::encode(path.file_name().unwrap().to_string_lossy().to_string().split(".").nth(path.file_name().unwrap().to_string_lossy().to_string().split(".").count() - 1).unwrap()).chars().into_iter().map(|e| e as u8).collect();
        let bind: Vec<u8> = base64::encode(std::fs::read(path).unwrap()).chars().into_iter().map(|e| e as u8).collect();
        let seperator: Vec<u8> = base64::encode("쟑맯됽뱫뷵").chars().into_iter().map(|e| e as u8).collect();

        for i in &seperator {
            file.push(*i);
        }

        for i in name_ext {
            file.push(i);
        }

        for i in &seperator {
            file.push(*i);
        }

        for i in bind {
            file.push(i);
        }
    }
    std::fs::write(&args[*build_index.lock().unwrap() as usize], file);
}
