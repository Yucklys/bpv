use crate::{get_bvid, VideoInfo};
use reqwest::Url;
use structopt::StructOpt;

fn parse_url(src: &str) -> Url {
    Url::parse(src).unwrap()
}

#[derive(Debug, StructOpt)]
#[structopt(name = "bpv", about = "Launch Bilibili video with Danmu on mpv.")]
pub struct Cli {
    #[structopt(name = "url", parse(from_str = parse_url))]
    url: Url,
    #[structopt(name = "part", short = "p")]
    part: Option<usize>,
}

impl Cli {
    pub fn get_video_info(&self) -> VideoInfo {
        VideoInfo {
            bvid: get_bvid(&self.url),
            p: self.part,
        }
    }
}
