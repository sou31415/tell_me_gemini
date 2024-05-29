use clap::Parser;
use serde::{Deserialize, Serialize};

#[allow(legacy_derive_helpers)]
#[command(
    author = "RuSwiftive",
    version = "0.1.0",
    about = "Hear what you want to hear with Gemini 1.0 Pro."
)]
#[allow(legacy_derive_helpers)]
#[derive(Debug, Parser)]
pub struct Args {
    /// Write what you wanna ask.
    #[arg(short, long)]
    pub prompt: String,
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
