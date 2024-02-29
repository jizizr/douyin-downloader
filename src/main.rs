use std::path::Path;

use clap::Parser;
use douyin_parser::{
    get_url, get_video_id, opt,
    pkg::{self, downloader::download_file},
};
fn main() {
    let args = opt::Douyin::parse();
    let video_id = get_video_id(&get_url(&args.url.join(""))).unwrap();
    let data = pkg::parser::get_info(video_id);
    let binding = Path::new(&args.path).join(&format!("{}.mp4", data.aweme_detail.desc));
    let path = binding.to_str().unwrap();
    println!("Downloading the video to {}", path);
    let _ = download_file(&data.aweme_detail.video.play_addr.url_list[0], &path);
}
