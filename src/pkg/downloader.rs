use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use reqwest::header::RANGE;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};

pub fn download_file(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let total_size = {
        let resp = client.head(url).send()?;
        if resp.status().is_success() {
            resp.headers()
                .get(reqwest::header::CONTENT_LENGTH)
                .and_then(|ct_len| ct_len.to_str().ok().and_then(|ct_len| ct_len.parse().ok()))
                .unwrap_or(0)
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get the file size",
            )));
        }
    };

    let mut request = client.get(url);
    let mut file = OpenOptions::new().create(true).write(true).open(path)?;
    let current_len = file.metadata()?.len();
    if current_len < total_size {
        request = request.header(RANGE, format!("bytes={}-", current_len));
    } else {
        return Ok(());
    }

    let mut response = request.send()?;
    file.seek(SeekFrom::Start(current_len))?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
    ).unwrap());

    let mut buffer = [0; 1024];
    while let Ok(n) = response.read(&mut buffer) {
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])?;
        pb.inc(n as u64);
    }

    pb.finish_with_message("download completed");

    Ok(())
}
