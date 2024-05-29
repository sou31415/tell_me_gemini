pub mod model;
pub mod secret;
use crate::model::{Args, ResponseModel, StdPostModel};
use clap::Parser;
use reqwest::Client;
use secret::TOKEN;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let arg = Args::parse();
    let url: String =
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key="
            .to_string()
            + TOKEN;
    let client = Client::new();
    let data = StdPostModel::new(arg.prompt.clone());
    let res = client
        .post(url)
        .json(&data)
        .send()
        .await?
        .json::<ResponseModel>()
        .await?;
    println!("{}", res.candidates[0].content.parts[0].text);
    Ok(())
}