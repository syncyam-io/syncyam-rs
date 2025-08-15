use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let git_args = ["rev-parse", "--short", "HEAD"];
    let output = Command::new("git").args(git_args).output().unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    Ok(())
}
