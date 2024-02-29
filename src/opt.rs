use clap::Parser;
lazy_static::lazy_static! {
    // 当前路径
    static ref PATH:String = std::env::current_dir().unwrap().to_str().unwrap().to_string();
}
#[derive(Parser)]
#[clap(version = "0.1", author = "Allen", about = "Douyin video downloader")]
pub struct Douyin {
    #[clap(required = true)]
    pub url: Vec<String>,
    #[clap(
        short,
        long,
        default_value = PATH.as_str(),
    )]
    pub path: String,
}
