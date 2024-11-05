#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! yansi = "0.5"
//! ```
extern crate yansi;
use std::process::Command;

macro_rules! run_command {
    ($cmd:expr) => {
        let mut command = command!($cmd);
        let mut child = command.spawn().unwrap();
        let status = child.wait().unwrap();
        if !status.success() {
            println!(
                "{} in {}",
                yansi::Paint::red("qualify terminates due to error"),
                yansi::Paint::yellow($cmd)
            );
            std::process::exit(-1);
        }
    };
}

macro_rules! command {
    ($cmd:expr) => {{
        print!("\n> {}\n", yansi::Paint::yellow($cmd));
        let mut chips = $cmd.split(' ');
        let mut command = Command::new(chips.next().unwrap());
        for chip in chips {
            command.arg(chip);
        }
        command
    }};
}

// fn run_script(s: &str) {
//     let mut path = std::path::PathBuf::from("./scripts");
//     path.push(s);
//     let command = format!(
//         "cargo script {}",
//         path.to_string_lossy().to_owned().to_string()
//     );
//     run_command!(&command);
// }

fn main() {
    // format
    run_command!("cargo fmt");

    // Build in important variants
    run_command!("cargo build");
    run_command!("cargo build --release");

    // Clippy in important variants
    run_command!("cargo clippy -- -D warnings");

    // Run tests in important variants
    run_command!("cargo test --release");
    run_command!("cargo test");

    // doc
    run_command!("cargo doc --all-features --no-deps --open");

    // check git status
    let mut cmd = command!("git status -s");
    let child = cmd.stdout(std::process::Stdio::piped()).spawn().unwrap();
    let output = child.wait_with_output().unwrap();
    if output.stdout.len() > 0 {
        print!("> {}", yansi::Paint::red("there are unsubmitted files"));
        std::process::exit(-1);
    }

    // say goodbye
    println!("\n> all done :-)  Looks like you're ready to \"cargo publish\"?");
}
