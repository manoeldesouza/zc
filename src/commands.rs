
use std::io::*;
use std::process;


pub fn run(cmd: &str, arguments: &[&str]) -> String {

    let mut command = process::Command::new(cmd);
    let result = command
        .args(arguments)
        .output()
        .expect("Failure running command: commands::run()");

    let output = String::from_utf8_lossy(&result.stdout).to_string();
    let errors = String::from_utf8_lossy(&result.stderr).to_string();

    format!("{}{}", output, errors)
}


pub fn list(cmd: &str, arguments: &[&str]) -> Vec<String> {

    let output = run(cmd, arguments);
    
    output
        .lines()
        .map(|s: &str| s.to_string())
        .collect::<Vec<String>>()
}

pub fn piped(first_cmd: &str, first_args: Vec<&str>,
             second_cmd: &str, second_args: Vec<&str>) -> Vec<String> {

    let mut first_command = process::Command::new(first_cmd)
        .args(first_args)
        .stdout(process::Stdio::piped())
        .spawn()
        .unwrap();

    let mut second_command = process::Command::new(second_cmd)
        .args(second_args)
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(ref mut stdout) = first_command.stdout {
        if let Some(ref mut stdin) = second_command.stdin {
            let mut buf: Vec<u8> = Vec::new();
            stdout.read_to_end(&mut buf).unwrap();
            stdin.write_all(&buf).unwrap();
        }
    }   

    let output = second_command.wait_with_output().unwrap().stdout;

    String::from_utf8_lossy(&output)
        .to_string()
        .lines()
        .map(|s: &str| s.to_string())
        .collect::<Vec<String>>()
}