use regex::Regex;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const BILIBILI_PAGELIST: &'static str = "https://api.bilibili.com/x/player/pagelist";
const BILIBILI_COMMENT: &'static str = "http://comment.bilibili.com";
const BILIBILI_PLAY_URL: &'static str = "https://api.bilibili.com/x/player/playurl";

pub struct VideoInfo {
    pub bvid: String,
    pub p: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct Playlist {
    pub data: Vec<VideoPart>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VideoPart {
    pub cid: usize,
    pub page: usize,
    pub part: String,
    pub dimension: Dimension,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Dimension {
    width: usize,
    height: usize,
}

impl Dimension {
    pub fn get_dimension(&self) -> String {
        format!("{width}x{height}", width = self.width, height = self.height)
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlayUrl {
    pub data: VideoData,
}

impl PlayUrl {
    pub fn get_play_url(&self) -> String {
        self.data.durl.last().unwrap().url.clone()
    }
}

#[derive(Serialize, Deserialize)]
pub struct VideoData {
    pub durl: Vec<VideoUrl>,
}

#[derive(Serialize, Deserialize)]
pub struct VideoUrl {
    pub url: String,
}

pub fn get_video_part(video_info: &VideoInfo) -> VideoPart {
    let bvid = video_info.bvid.clone();
    let url_get_cid = Url::parse_with_params(BILIBILI_PAGELIST, &[("bvid", bvid)]).unwrap();
    let content = reqwest::get(url_get_cid).unwrap().text().unwrap();
    let playlist: Playlist = serde_json::from_str(&content).unwrap();
    let page = video_info.p.unwrap_or(1);
    playlist
        .data
        .iter()
        .find(|p| p.page == page)
        .unwrap()
        .clone()
}

pub fn get_bvid(url: &Url) -> String {
    let re = Regex::new(r"BV(.*)").unwrap();
    let cap = re.captures(url.as_str()).unwrap();
    cap.get(1).unwrap().as_str().to_string()
}

pub fn get_comment(video_info: VideoInfo) -> String {
    let video_part = get_video_part(&video_info);
    let cid = video_part.cid;
    let url_get_comment = Url::parse(BILIBILI_COMMENT)
        .unwrap()
        .join(&format!("{}.xml", cid))
        .unwrap();

    let bpv_tmp_dir = PathBuf::from("/tmp/bpv");
    if !bpv_tmp_dir.exists() {
        fs::create_dir(&bpv_tmp_dir).unwrap()
    }
    let comment_xml_path = bpv_tmp_dir.join(format!("{}.xml", cid));
    Command::new("curl")
        .arg(url_get_comment.as_str())
        .arg("--output")
        .arg(comment_xml_path.to_str().unwrap())
        .arg("--compressed")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    let comment_ass_file = bpv_tmp_dir.join(format!("{}.ass", cid));
    Command::new("danmaku2ass")
        .arg(comment_xml_path.to_str().unwrap())
        .args(&["-s", &video_part.dimension.get_dimension()])
        .args(&["-ds", "10"])
        .args(&["-dm", "15"])
        .args(&["-fs", "35"])
        .args(&["-o", comment_ass_file.to_str().unwrap()])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    comment_ass_file.to_str().unwrap().to_string()
}

pub fn get_play_url(cid: usize, bvid: String) -> String {
    let url_get_play_url = Url::parse_with_params(
        BILIBILI_PLAY_URL,
        &[("cid", &cid.to_string()), ("bvid", &bvid)],
    )
    .unwrap();
    let content = reqwest::get(url_get_play_url).unwrap().text().unwrap();
    let play_url: PlayUrl = serde_json::from_str(&content).unwrap();
    let url = play_url.get_play_url();
    url.replace("\\", "")
}
