use chrono::Local;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Parser)]
#[command(
    author = "RuSwiftive",
    version = "0.1.0",
    about = "Hear what you want to hear with Gemini 1.0 Pro."
)]
pub struct Args {
    /// Write what you wanna ask.
    #[arg(short, long)]
    pub custom: Option<String>,
    #[arg(long)]
    pub history: bool,
    #[arg(short, long)]
    pub translate: Option<Language>,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Language {
    En,
    Jp,
    Fr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part {
    pub text: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AppendToFile {
    pub custom: String,
    pub answer: String,
    pub time: String,
}
impl AppendToFile {
    pub fn new(custom: String, answer: String) -> Self {
        let now = Local::now();
        Self {
            custom,
            answer,
            time: format!("{:?}", now),
        }
    }
}
impl fmt::Display for AppendToFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!(
            "\nTime : {}\nQuestion : {}\nAnswer : {}\n",
            self.time, self.custom, self.answer
        );
        write!(f, "{}", s)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct StdPostModel {
    pub contents: Vec<Content>,
    pub systemInstruction: Option<Content>,
}

impl StdPostModel {
    pub fn new(value: String) -> Self {
        let response = {
            Self {
                contents: vec![Content {
                    parts: vec![Part { text: value }],
                    role: "user".to_string(),
                }],
                systemInstruction: None,
            }
        };
        return response;
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub content: Content,
    finishReason: String,
    index: usize,
    #[allow(non_snake_case)]
    safetyRatings: Vec<SafetyRating>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyRating {
    category: String,
    probability: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseModel {
    pub candidates: Vec<Candidate>,
    pub usageMetadata: Metadata,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct citationSource {
    startIndex: usize,
    endIndex: usize,
    uri: String,
    license: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    promptTokenCount: usize,
    candidatesTokenCount: usize,
    totalTokenCount: usize,
}
