use reqwest::Client;
use std::env;

pub fn get_input(day: u8) -> String {
    let url = std::format!("https://adventofcode.com/2021/day/{}/input", day);
    let url = reqwest::Url::parse(&url).expect("Failed to parse url");

    let session = env::var("AOC_SESSION").expect("Please set the AOC_SESSION env variable");

    let client = Client::new();
    let mut res = client
        .get(url)
        .header("Cookie", std::format!("session={}", session))
        .send()
        .expect("Request failed");

    res.text().expect("Getting request body failed")
}
