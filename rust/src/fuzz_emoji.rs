extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate emojis;

use emojis::iter;
use std::collections::HashMap;
use std::error::Error;
use serde::Deserialize;

#[derive(Debug)]
pub(crate) struct FuzzEmoji {
    emoji_dict: HashMap<String, String>,
}

impl FuzzEmoji {
    pub(crate) fn new() -> FuzzEmoji {
        let mut emoji_dict = HashMap::new();
        for emoji in iter() {
            let name = emoji.shortcodes().next().unwrap_or_default().to_string();
            let glyph = emoji.as_str().to_string();
            emoji_dict.insert(name, glyph);
        }
        FuzzEmoji { emoji_dict }
    }

    fn get_synonyms(&self, word: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let url = format!("https://api.datamuse.com/words?rel_syn={}", word);
        let resp = reqwest::blocking::get(&url)?;

        if resp.status().is_success() {
            let body = resp.text()?;
            let words: Vec<Word> = serde_json::from_str(&body)?;
            let synonyms = words.into_iter().map(|w| w.word).collect();
            Ok(synonyms)
        } else {
            Err(format!("failed to fetch synonyms: {}", resp.status()).into())
        }
    }

    fn get_emoji(&self, description: &str) -> (String, String) {
        let description = description.to_lowercase();

        if let Some(emoji_char) = self.emoji_dict.get(&description) {
            return (description.clone(), emoji_char.clone());
        }

        for (name, emoji_char) in &self.emoji_dict {
            if name.contains(&description) {
                return (name.clone(), emoji_char.clone());
            }
        }

        let synonyms = match self.get_synonyms(&description) {
            Ok(synonyms) => synonyms,
            Err(_) => return ("".to_string(), "".to_string()),
        };

        for syn in &synonyms {
            if let Some(emoji_char) = self.emoji_dict.get(syn) {
                return (syn.clone(), emoji_char.clone());
            }
        }

        for (name, emoji_char) in &self.emoji_dict {
            for syn in &synonyms {
                if name.contains(syn) {
                    return (syn.clone(), emoji_char.clone());
                }
            }
        }

        ("".to_string(), "".to_string())
    }

    pub(crate) fn get_emojis(&self, descriptions: Vec<&str>) -> HashMap<String, String> {
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