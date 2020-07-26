
use std::process;

pub struct CommandResult {

    pub name: String,
    pub used: String,   
}


pub fn zfs_pools() -> Vec<CommandResult> {
    
    vec![
        CommandResult {
            name: "Carolina Wildner Simm".to_string(),
            used: "36".to_string()
        }
    ]
}

pub fn zfs_dataset() -> Vec<CommandResult> {
    let arguments = vec!["list", "-o", "name,used", "-H"];
    run_command("zfs", &arguments)
}

pub fn zfs_volumes() -> Vec<CommandResult> {
    
    vec![
        CommandResult {
            name: "Manoel de Souza e Silva Neto".to_string(),
            used: "40".to_string()
        }
    ]
}

pub fn zfs_snapshots() -> Vec<CommandResult> {
    let arguments = vec!["list", "-H", "-o", "name,used", "-t", "snapshot"];
    run_command("zfs", &arguments)
}

pub fn run_command(cmd: &str, arguments: &Vec<&str>) -> Vec<CommandResult> {

    let mut command = process::Command::new(cmd);
    let output = command.args(arguments).output().expect("Failure running command").stdout;
    let command_output = String::from_utf8_lossy(&output).to_string();

    let lines: Vec<&str> = command_output.split("\n").collect();

    let mut result = Vec::new();

    for line in lines.iter() {

        let mut split = line.split_whitespace();

        let name = match split.next() {
            Some(n) => n.to_string(),
            None => String::new(),
        };

        let used = match split.next() {
            Some(n) => n.to_string(),
            None => String::new(),
        };

        let command_result = CommandResult {
            name: name,
            used: used,
        };
        
        result.push(command_result);
    }

    result
}