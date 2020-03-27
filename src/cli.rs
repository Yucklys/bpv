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
}

impl Cli {
    pub fn get_url(&self) -> &Url {
        &self.url
    }
}
