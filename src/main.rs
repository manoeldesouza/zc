
use std::env;

mod display;
mod dialog;
mod command;
mod content;

const NAME: &str = "zc - ZFS Commander";
const COPYRIGHT: &str = "Copyright (c) 2020, Manoel de Souza <manoel.desouza@outlook.com.br>";
const VERSION: &str = "0.9.7";
const RELEASE: &str = "31-Jul-2020";
const HELP: &str = r#"
    Navigation Keys:

    UP/DOWN/PG_UP/PG_DOWN: Navigate within current pane
    LEFT/RIGHT:            Change pane
    TAB:                   Change current pane mode


    Function Keys:

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
    |  F9   |  zpool get all  |  zfs get all  |  zfs get all  |  zfs get all  |
    |  F10  |       Exit      |      Exit     |      Exit     |      Exit     |
"#;

const LICENSE: &str = r#"
    BSD 4-Clause License
    
    Copyright (c) 2020, Manoel de Souza <manoel.desouza@outlook.com.br>
    All rights reserved.
    
    Redistribution and use in source and binary forms, with or without
    modification, are permitted provided that the following conditions are met:
    
    1. Redistributions of source code must retain the above copyright notice, this
       list of conditions and the following disclaimer.
    
    2. Redistributions in binary form must reproduce the above copyright notice,
       this list of conditions and the following disclaimer in the documentation
       and/or other materials provided with the distribution.
    
    3. All advertising materials mentioning features or use of this software must
       display the following acknowledgement:
         This product includes software developed by Manoel de Souza.
    
    4. Neither the name of the copyright holder nor the names of its
       contributors may be used to endorse or promote products derived from
       this software without specific prior written permission.
    
    THIS SOFTWARE IS PROVIDED BY COPYRIGHT HOLDER "AS IS" AND ANY EXPRESS OR
    IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
    MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO
    EVENT SHALL COPYRIGHT HOLDER BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
    SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
    PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
    OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
    WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
    OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
    ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
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
