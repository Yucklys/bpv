use regex::Regex;
use reqwest::Url;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const BILIBILI_PAGELIST: &'static str = "https://api.bilibili.com/x/player/pagelist";
const BILIBILI_COMMENT: &'static str = "http://comment.bilibili.com";

pub fn get_cid(url: &Url) -> String {
    let url_get_cid = Url::parse_with_params(BILIBILI_PAGELIST, &[("aid", &get_aid(url))]).unwrap();
    let content = reqwest::get(url_get_cid).unwrap().text().unwrap();
    let re = Regex::new(r"(\d{9})").unwrap();
    let cap = re.captures(&content).unwrap();
    cap.get(1).unwrap().as_str().to_string()
}

pub fn get_aid(url: &Url) -> String {
    let re = Regex::new(r"av(\d*)").unwrap();
    let cap = re.captures(url.as_str()).unwrap();
    cap.get(1).unwrap().as_str().to_string()
}

pub fn get_comment(url: &Url) -> String {
    let cid = get_cid(url);
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
        .args(&["-s", "1920x1080"])
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
