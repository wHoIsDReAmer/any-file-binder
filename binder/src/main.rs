mod utils;

use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

type Binder<'a> = utils::binder::Binder<'a>;

fn main() {
    let mut binder = Binder::default();
    binder.stub = std::fs::read("stub.exe").unwrap();

    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    // type cli mode
    if args.len() == 0 {
        println!("$$$$$$$\\  $$$$$$\\ $$\\   $$\\ $$$$$$$\\  $$$$$$$$\\ $$$$$$$\\  ");
        println!("$$  __$$\\ \\_$$  _|$$$\\  $$ |$$  __$$\\ $$  _____|$$  __$$\\ ");
        println!("$$ |  $$ |  $$ |  $$$$\\ $$ |$$ |  $$ |$$ |      $$ |  $$ |");
        println!("$$$$$$$\\ |  $$ |  $$ $$\\$$ |$$ |  $$ |$$$$$\\    $$$$$$$  |");
        println!("$$  __$$\\   $$ |  $$ \\$$$$ |$$ |  $$ |$$  __|   $$  __$$< ");
        println!("$$ |  $$ |  $$ |  $$ |\\$$$ |$$ |  $$ |$$ |      $$ |  $$ |");
        println!("$$$$$$$  |$$$$$$\\ $$ | \\$$ |$$$$$$$  |$$$$$$$$\\ $$ |  $$ |");
        println!("\\_______/ \\______|\\__|  \\__|\\_______/ \\________|\\__|  \\__|");
        println!();

        loop {
            std::io::stdout().write(b"input your file path (exit: q) : ");
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

        binder.output_file_name = output.clone();

        args.push("-build".into());
        args.push(output);
    }

    let mut build_index = 0usize;
    for path in args.iter().enumerate() {
        let index = path.0;
        let path = PathBuf::from(path.1);

        if path.starts_with("-build") {
            build_index = index + 1;
            break
        }

        binder.add_file(
            path.file_name().unwrap().to_string_lossy().to_string().split(".").nth(path.file_name().unwrap().to_string_lossy().to_string().split(".").count() - 1).unwrap().to_string(),
            std::fs::read(path).unwrap()
        );
    }

    if let Err(_) = binder.build() {
        println!("Some error occured during build file..");
    };
}
