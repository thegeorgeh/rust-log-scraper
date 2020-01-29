
use std::process::Command;

pub fn tail_grep(file_name: String, grep_string: String) -> String {

    let output = Command::new("sh").arg("-c").arg(format!("tail -100 {} | grep -i {} | perl -ne 'print \"|$_\"'", file_name, grep_string)).output().expect("");

    //let op = output.stdout;
    return String::from(String::from_utf8_lossy(&output.stdout));
}
