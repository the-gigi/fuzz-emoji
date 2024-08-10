extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use serde::Deserialize;

#[derive(Debug)]
pub(crate) struct FuzzEmoji {
    emoji_dict: HashMap<String, String>,
}

impl FuzzEmoji {
    fn new() -> FuzzEmoji {
        let mut emoji_dict = HashMap::new();
        let emojis = emoji::EMOJI_MAP;
        for (name, e) in emojis.iter() {
            let name = name.trim_matches(':').to_lowercase();
            emoji_dict.insert(name, e.to_string());
        }
        FuzzEmoji { emoji_dict }
    }

    async fn get_synonyms(&self, word: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let url = format!("https://api.datamuse.com/words?rel_syn={}", word);
        let resp = reqwest::get(&url).await?;
        if resp.status().is_success() {
            let body = resp.text().await?;
            let words: Vec<Word> = serde_json::from_str(&body)?;
            let synonyms = words.into_iter().map(|w| w.word).collect();
            Ok(synonyms)
        } else {
            Err(Box::new(CustomError(format!(
                "failed to fetch synonyms: {}",
                resp.status()
            ))))
        }
    }

    fn get_emoji(&self, description: &str) -> (String, String) {
        let description = description.to_lowercase();

        // Direct match
        if let Some(emoji_char) = self.emoji_dict.get(&description) {
            return (description.clone(), emoji_char.clone());
        }

        // Subset match
        for (name, emoji_char) in &self.emoji_dict {
            if name.contains(&description) {
                return (name.clone(), emoji_char.clone());
            }
        }

        if let Ok(synonyms) = self.get_synonyms(&description) {
            // Synonym match
            for syn in &synonyms {
                if let Some(emoji_char) = self.emoji_dict.get(syn) {
                    return (syn.clone(), emoji_char.clone());
                }
            }

            // Subset match
            for (name, emoji_char) in &self.emoji_dict {
                for syn in &synonyms {
                    if name.contains(syn) {
                        return (syn.clone(), emoji_char.clone());
                    }
                }
            }
        }

        ("".to_string(), "".to_string())
    }

    fn get_emojis(&self, descriptions: Vec<&str>) -> HashMap<String, String> {
        let mut result = HashMap::new();
        for d in descriptions {
            let (name, emoji_char) = self.get_emoji(d);
            result.insert(d.to_string(), format!("{}, {}", name, emoji_char));
        }
        result
    }
}

#[derive(Deserialize)]
struct Word {
    word: String,
}

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CustomError {}


/*
use emojis::{Emoji, lookup};

fn get_emoji(name: &str) -> Option<&'static str> {
    lookup(name).map(|emoji: &Emoji| emoji.as_str())
}

 */