
use std::env;

mod display;
mod dialog;
mod command;
mod content;


const VERSION: &str = "0.9.0";
const RELEASE: &str = "30-Jul-2020";

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("-v")) {

        println!("ZFS Commander - Copyright (c) 2020, Manoel de Souza <manoel.desouza@outlook.com.br>");
        println!("All rights reserved.");
        println!("       Version: {}", VERSION);
        println!("  Release Date: {}", RELEASE);
        return;
    }

    if command::is_zfs_installed() {
        let mut screen = display::Screen::new();
        screen.run()

    } else {
        println!("ZFS is not installed")
    };
}
