
use std::io::*;
use std::process;

pub struct CommandResult {

    pub name: String,
    pub used: String,   
}

pub fn list_command(cmd: &str, arguments: &[&str]) -> Vec<CommandResult> {

    let mut command = process::Command::new(cmd);
    let output = command.args(arguments).output().expect("Failure running command: list_command").stdout;
    let command_output = String::from_utf8_lossy(&output).to_string();

    let lines: Vec<&str> = command_output.split('\n').collect();

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
            name,
            used,
        };
        
        result.push(command_result);
    }

    result
}

pub fn run_command(cmd: &str, arguments: &[&str]) -> String {

    let mut command = process::Command::new(cmd);
    let output = command.args(arguments).output().expect("Failure running command: run_command").stdout;

    String::from_utf8_lossy(&output).to_string()
}

pub fn is_zfs_installed() -> bool {

    let mut command = process::Command::new("which");
    let temp_output = command.arg("zfs").output().expect("Failure running command: is_zfs_installed");
    let output = String::from_utf8_lossy(&temp_output.stdout);
    
    output.contains("zfs")
}

pub fn list_pools() -> Vec<CommandResult> {
    
    let arguments = vec!["list", "-o", "name,size", "-H"];
    list_command("zpool", &arguments)
}

pub fn list_dataset() -> Vec<CommandResult> {

    let arguments = vec!["list", "-H", "-o", "name,used", "-t", "filesystem"];
    list_command("zfs", &arguments)
}

pub fn list_volumes() -> Vec<CommandResult> {
    
    let arguments = vec!["list", "-H", "-o", "name,used", "-t", "volume"];
    list_command("zfs", &arguments)
}

pub fn list_snapshots() -> Vec<CommandResult> {
    let arguments = vec!["list", "-H", "-o", "name,used", "-t", "snapshot"];
    list_command("zfs", &arguments)
}

pub fn volume_create(volume_name: String, volume_size: String) {

    let arguments = vec!["create", "-V", volume_size.as_str(), volume_name.as_str()];
    run_command("zfs", &arguments);
}

pub fn zfs_create(dataset_name: String) {

    let arguments = vec!["create", dataset_name.as_str()];
    run_command("zfs", &arguments);
}

pub fn zfs_rename(old_dataset_name: String, new_dataset_name: String) {

    let arguments = vec!["rename", old_dataset_name.as_str(), new_dataset_name.as_str()];
    run_command("zfs", &arguments);
}

pub fn zfs_clone(snapshot_name: String, new_dataset_name: String) {

    let arguments = vec!["clone", snapshot_name.as_str(), new_dataset_name.as_str()];
    run_command("zfs", &arguments);
}

pub fn zfs_snapshot(snapshot_name: String) {

    let arguments = vec!["snapshot", snapshot_name.as_str()];
    run_command("zfs", &arguments);
}

pub fn zfs_destroy(selected_elements: Vec<String>) {

    for element in selected_elements {

        let arguments = vec!["destroy", element.as_str()];
        run_command("zfs", &arguments);
    }
}

pub fn zfs_diff(snapshot_1: String, snapshot_2: String) -> String {

    let arguments = vec!["diff", snapshot_1.as_str(), snapshot_2.as_str()];
    run_command("zfs", &arguments)
}

pub fn zfs_send(snapshot_source: String, snapshot_stream: String) -> Result<(),> {

    let send_arguments = vec!["send", snapshot_source.as_str()];

    let mut send_command = process::Command::new("zfs")
        .args(send_arguments)
        .stdout(process::Stdio::piped())
        .spawn() ?;
        // .unwrap();

    let stream_cmd: Vec<&str> = snapshot_stream.split_whitespace().collect();
    let stream_arguments = stream_cmd.get(1..).unwrap();

    let mut recv_command = process::Command::new(stream_cmd.get(0).unwrap())
        .args(stream_arguments)
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn() ?;
        // .unwrap();

    if let Some(ref mut stdout) = send_command.stdout {
        if let Some(ref mut stdin) = recv_command.stdin {
            let mut buf: Vec<u8> = Vec::new();
            // stdout.read_to_end(&mut buf).unwrap();
            // stdin.write_all(&buf).unwrap();
            stdout.read_to_end(&mut buf) ?;
            stdin.write_all(&buf) ?;
        }
    }

    let _ = recv_command.wait_with_output().unwrap().stdout;

    Ok(())
}

pub fn zpool_get_all(dataset: String) -> String {

    let arguments = vec!["get", "all", dataset.as_str()];
    run_command("zpool", &arguments)
}

pub fn zfs_get_all(dataset: String) -> String {

    let arguments = vec!["get", "all", dataset.as_str()];
    run_command("zfs", &arguments)
}

pub fn zfs_rollback(selected_elements: Vec<String>) {

    for element in selected_elements {

        let arguments = vec!["rollback", "-rf", element.as_str()];
        run_command("zfs", &arguments);
    }
}

pub fn zpool_destroy(selected_elements: Vec<String>) {

    for element in selected_elements {

        let arguments = vec!["destroy", element.as_str()];
        run_command("zpool", &arguments);
    }
}

pub fn zpool_scrub(selected_elements: Vec<String>) {

    for element in selected_elements {

        let arguments = vec!["scrub", element.as_str()];
        run_command("zpool", &arguments);
    }
}
