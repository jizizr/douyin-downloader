use reqwest::blocking::Client;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct Data {
    pub aweme_detail: Aweme,
}
#[derive(Deserialize, Debug)]
pub struct Aweme {
    pub desc: String,
    pub video: Video,
}

#[derive(Deserialize, Debug)]
pub struct Video {
    pub play_addr: PlayAddr,
}
#[derive(Deserialize, Debug)]
pub struct PlayAddr {
    pub url_list: Vec<String>,
}
pub fn get_info(vid: String) -> Data {
    let url = format!(
        "https://www.douyin.com/aweme/v1/web/aweme/detail/?device_platform=webapp&aid=6383&channel=channel_pc_web&aweme_id={}&pc_client_type=1&version_code=190500&version_name=19.5.0&cookie_enabled=true&screen_width=1707&screen_height=960&browser_language=zh-CN&browser_platform=Win32&browser_name=Chrome&browser_version=121.0.0.0&browser_online=true&engine_name=Blink&engine_version=121.0.0.0&os_name=Windows&os_version=10&cpu_core_num=24&device_memory=8&platform=PC&downlink=0.4&effective_type=2g&round_trip_time=2200&webid={}",
        vid, vid
    );
    //client with cookies
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "cookie",
        reqwest::header::HeaderValue::from_static("COOKIE"),
    );
    let client = Client::builder().default_headers(headers).build().unwrap();
    let binding = client.get(&url).send().unwrap();
    let data: Data = binding.json().unwrap();
    data
}
