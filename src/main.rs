
use std::env;

mod display;
mod dialog;
mod command;
mod content;


const NAME: &str = "zc - ZFS Commander";
const COPYRIGHT: &str = "Copyright (c) 2020, Manoel de Souza <manoel.desouza@outlook.com.br>";
const VERSION: &str = "0.9.5";
const RELEASE: &str = "31-Jul-2020";

const HELP: &str = r#"
                  |======= Navigation Keys ===========|
                  |                                   |
                  |   LEFT/RIGHT: Change pane         |
                  |   TAB: Change current pane mode   |
                  |                                   |
                  =====================================

================================ Function Keys ============================
|  Key  |       Pool      |    Dataset    |    Snapshot   |     Volume    |
|:-----:|:---------------:|:-------------:|:-------------:|:-------------:|
|  F1   |       Help      |      Help     |      Help     |      Help     |
|  F2   |         -       |        -      |    zfs diff   |        -      |
|  F3   |         -       |        -      |    zfs send   |        -      |
|  F4   |         -       |        -      |        -      |        -      |
|  F5   |         -       |  zfs snapshot |   zfs clone   |  zfs snapshot |
|  F6   |         -       |   zfs rename  |   zfs rename  |   zfs rename  |
|  F7   |   zpool scrub   |   zfs create  |  zfs rollback |   zfs create  |
|  F8   |  zpool destroy  |  zfs destroy  |  zfs destroy  |  zfs destroy  |
|  F9   |         -       |  zfs get all  |  zfs get all  |  zfs get all  |
|  F10  |       Exit      |      Exit     |      Exit     |      Exit     |
===========================================================================
"#;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("-v")) {

        println!("{}", NAME);
        println!("{}", COPYRIGHT);
        println!("All rights reserved.");
        println!("  Version: {}", VERSION);
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
