pub mod opt;
pub mod pkg;
use regex::Regex;

lazy_static::lazy_static! {
    static ref REGEX: Regex = Regex::new(r".*(https://v.douyin.com/(\w+)).*").unwrap();
}

pub fn get_video_id(url: &str) -> Result<String, reqwest::Error> {
    let binding = reqwest::blocking::get(url)?;
    let u = binding.url();
    Ok(u.path_segments().unwrap().last().unwrap().to_string())
}

pub fn get_url(url: &str) -> String {
    let caps = REGEX.captures(url).unwrap();
    caps[1].to_string()
}
