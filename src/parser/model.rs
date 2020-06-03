use log::warn;
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub timestamp: String,
    pub revision_id: String,
    pub contents: Vec<String>,
    pub categories: Vec<String>,
    pub headings: Vec<String>,
    pub images: Vec<Image>,
    pub links: Vec<Text>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ImageType {
    Image,
    File,
}

impl FromStr for ImageType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "file" => Ok(ImageType::File),
            "image" => Ok(ImageType::Image),
            _ => {
                warn!("Unexpected Image Type: [{}]", s);
                Err(())
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub target: String,
    pub target_type: ImageType,
    pub text: Text,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Text {
    LinkText { text: String, link: Link },
    Text { text: String },
}

impl Text {
    pub fn clone_text(&self) -> String {
        return match self {
            Text::LinkText { text, .. } => String::from(text),
            Text::Text { text, .. } => String::from(text),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Link {
    ExternalLink { target: String },
    Link { target: String },
}
