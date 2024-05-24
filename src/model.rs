use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[command(author,version,about,long_about = None)]
pub struct Args {
    #[arg(short, long)]
    prompt: String,
    #[arg(short,long,default_value=None)]
    role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StdPostModel {
    pub contents: Vec<Content>,
}
impl StdPostModel {
    pub fn new(value: String) -> Self {
        Self {
            contents: vec![Content {
                parts: vec![Part { text: value }],
                role: "user".to_string(),
            }],
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub content: Content,
    finishReason: String,
    index: usize,
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

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct citationSource {
    startIndex: usize,
    endIndex: usize,
    uri: String,
    license: String,
}

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    promptTokenCount: usize,
    candidatesTokenCount: usize,
    totalTokenCount: usize,
}
