pub mod model;
pub mod secret;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use crate::model::{AppendToFile, Args, Language, ResponseModel, StdPostModel};
use clap::Parser;
use reqwest::Client;
use secret::TOKEN;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let arg = Args::parse();
    let url: String =
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key="
            .to_string()
            + TOKEN;
    if !arg.history.clone() {
        let client = Client::new();
        if let Some(mut custom) = arg.custom.clone() {
            let s = custom.clone();
            match arg.translate.clone() {
                Some(Language::En) => custom = "「".to_string() + s.as_str() + "」を英語に翻訳して",
                Some(Language::Jp) => {
                    custom = "「".to_string() + s.as_str() + "」を日本語に翻訳して"
                }
                Some(Language::Fr) => {
                    custom = "「".to_string() + s.as_str() + "」をフランス語に翻訳して"
                }
                _ => print!(""),
            }
            let data = StdPostModel::new(custom.clone());
            let res = client
                .post(url)
                .json(&data)
                .send()
                .await?
                .json::<ResponseModel>()
                .await?;
            let body = AppendToFile::new(custom, res.candidates[0].content.parts[0].text.clone());
            let mut file_ref = OpenOptions::new()
                .append(true)
                .open("/Users/sotarofurukawa/.config/askai/value.txt")
                .expect("Couldn't open file.");
            file_ref
                .write_all(format!("{}", body).as_bytes())
                .expect("Couldn't write file.");
            println!("{}", res.candidates[0].content.parts[0].text);
        }
    } else {
        let mut file = File::open("/Users/sotarofurukawa/.config/askai/value.txt").unwrap();
        let mut body = String::new();
        file.read_to_string(&mut body).unwrap();
        println!("{}", body);
    }
    Ok(())
}
