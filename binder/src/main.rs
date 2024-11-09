mod binder;

use anyhow::Result;
use std::io::Write;

use binder::Binder;

macro_rules! print {
    ($($arg:tt)*) => {
        std::io::stdout().write(format!($($arg)*).as_bytes()).unwrap();
        std::io::stdout().flush().unwrap();
    }
}

fn print_logo() {
    println!("$$$$$$$\\  $$$$$$\\ $$\\   $$\\ $$$$$$$\\  $$$$$$$$\\ $$$$$$$\\  ");
    println!("$$  __$$\\ \\_$$  _|$$$\\  $$ |$$  __$$\\ $$  _____|$$  __$$\\ ");
    println!("$$ |  $$ |  $$ |  $$$$\\ $$ |$$ |  $$ |$$ |      $$ |  $$ |");
    println!("$$$$$$$\\ |  $$ |  $$ $$\\$$ |$$ |  $$ |$$$$$\\    $$$$$$$  |");
    println!("$$  __$$\\   $$ |  $$ \\$$$$ |$$ |  $$ |$$  __|   $$  __$$< ");
    println!("$$ |  $$ |  $$ |  $$ |\\$$$ |$$ |  $$ |$$ |      $$ |  $$ |");
    println!("$$$$$$$  |$$$$$$\\ $$ | \\$$ |$$$$$$$  |$$$$$$$$\\ $$ |  $$ |");
    println!("\\_______/ \\______|\\__|  \\__|\\_______/ \\________|\\__|  \\__|");
    println!();
    println!("Usage:");
    println!("  -s <stub_path>    Set custom stub executable path (default: stub.exe)");
    println!("  -f <file1> <file2> ...    List of files to bind");
    println!("  -o <output_path>          Set output file path");
    println!("  Or run without arguments for interactive mode");
    println!();
    println!("Official repository: https://github.com/wHoIsDReAmer/any-file-binder")
}

fn input_files(args: &mut Vec<String>) { 
    // Add -s argument for stub file (default)
    args.push("-s stub.exe".into());

    // Add -f argument for input files
    args.push("-f".into());

    loop {
        print!("Enter file path (or 'q' to finish): ");

        let mut path = String::new();
        std::io::stdin().read_line(&mut path).expect("Failed to read line");
        path = path.trim().to_string();

        if path == "q" {
            break
        }

        args.push(path.replace("\"", ""));
    }

    print!("Enter output filename: ");

    let mut output = String::new();
    std::io::stdin().read_line(&mut output).expect("Failed to read line");

    output = output.trim().to_string();

    args.push("-o".into());
    args.push(output);
}

fn parse_arguments<'a>(args: &'a Vec<String>, binder: &mut Binder) -> Result<&'a str> {
    // Get -s argument value for set stub file
    let stub_index = args.iter().position(|arg| arg == "-s");
    let stub_file_path = match stub_index {
        Some(index) => args.get(index + 1),
        None => None,
    };
    
    binder.stub = match stub_file_path {
        Some(path) => std::fs::read(path)?,
        None => std::fs::read("stub.exe")?,
    };

    // Get -f argument value for set input files
    let files_index = args.iter()
        .position(|arg| arg == "-f")
        .ok_or(anyhow::anyhow!("No input files specified"))?;
    
    let mut files = Vec::new();
    // Collect all arguments until we hit another flag or end of args
    for arg in args.iter().skip(files_index + 1) {
        if arg.starts_with("-") {
            break;
        }
        files.push(arg.clone());
    }
    
    if files.is_empty() {
        return Err(anyhow::anyhow!("No input files provided after -f"));
    }
    binder.input_files = files;

    // Get -o argument value for set output file name
    let output_index = args.iter()
        .position(|arg| arg == "-o")
        .ok_or(anyhow::anyhow!("No output file name specified"))?;
    
    let output_file_name = args.get(output_index + 1)
        .ok_or(anyhow::anyhow!("Missing output filename"))?;

    Ok(output_file_name)
}

fn main() -> Result<()> {
    let mut binder = Binder::default();

    let mut args: Vec<String> = std::env::args().collect();
    // Delete first argument, first argument is path of executable
    args.remove(0);

    print_logo();

    // if no argument, CLI for users don't know what to input
    if args.len() == 0 {
        binder.stub = std::fs::read("stub.exe")?;
        input_files(&mut args);
    }

    let output_file_name = parse_arguments(&args, &mut binder)?;

    match binder.bind() {
        Ok(data) => std::fs::write(output_file_name, data)?,
        Err(err) => panic!("Error occured during binding files: {}", err),
    }

    Ok(())
}
