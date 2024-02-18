use std::{
    io::Write,
    process::{Command, Stdio},
};

pub fn prettier(input: &str) -> String {
    let mut child = Command::new("npx")
        .arg("prettier")
        .arg("--stdin-filepath")
        .arg("index.ts")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start prettier");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    stdin
        .write_all(input.as_bytes())
        .expect("Failed to write to stdin");

    drop(stdin);

    let output = child
        .wait_with_output()
        .expect("Failed to wait for prettier");

    String::from_utf8(output.stdout).expect("Failed to read prettier output")
}
