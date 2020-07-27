
mod display;
mod command;

fn main() {

    if command::is_zfs_installed() {
        let mut screen = display::Screen::new();
        screen.run()

    } else {
        println!("ZFS is not installed")
    };
}
