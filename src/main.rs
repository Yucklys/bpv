mod cli;
mod module;

use cli::Cli;
use module::*;
use std::io::{prelude::*, stdout};
use std::process::Command;
use structopt::StructOpt;

fn main() {
    let cli = Cli::from_args();
    let url = cli.get_url();
    let comment_file = get_comment(url);
    println!("{}", comment_file);
    let output = Command::new("mpv")
        .arg(url.as_str())
        .arg(format!("--sub-file={}", &comment_file))
        .output()
        .expect("failed to start mpv");
    stdout().write_all(&output.stdout).unwrap();
}
