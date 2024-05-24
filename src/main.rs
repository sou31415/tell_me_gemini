pub mod model;
pub mod secret;
use crate::model::{Args, Content, Part, ResponseModel, StdPostModel};
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
    let data = StdPostModel {
        contents: vec![Content {
            parts: vec![Part {
                text: "hello.".to_string(),
            }],
            role: "user".to_string(),
        }],
    };
    let res = client
        .post(url)
        .json(&data)
        .send()
        .await?
        .json::<ResponseModel>()
        .await?;
    println!("{:?}", res.candidates);
    Ok(())
}
