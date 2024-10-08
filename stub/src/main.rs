// #![windows_subsystem = "windows"]

use std::fs;

mod core;

fn main() {
    let self_path = std::env::current_exe().unwrap();
    let self_binary = fs::read(self_path).unwrap();
    let stub = core::parse_stub(&self_binary).unwrap();

    println!("{:?}", stub);
}
