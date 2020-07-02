mod cli;
mod module;

use cli::Cli;
use module::*;
use std::io::{prelude::*, stdout};
use std::process::Command;
use structopt::StructOpt;

fn main() {
    let cli = Cli::from_args();
    let video_info = cli.get_video_info();
    let bvid = video_info.bvid.clone();
    let video_part = get_video_part(&video_info);
    let cid = video_part.cid;
    let comment_file = get_comment(video_info);
    let play_url = get_play_url(cid, bvid);
    let output = Command::new("mpv")
        .arg(play_url)
        .arg(format!("--sub-file={}", &comment_file))
        .arg("--ytdl=no")
        .arg("--referrer=https://www.bilibili.com")
        .output()
        .expect("failed to start mpv");
    stdout().write_all(&output.stdout).unwrap();
}
