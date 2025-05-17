use crate::util;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
pub struct PathsConfig {
    pub save_path: PathBuf,
    pub json_path: PathBuf,
}

pub struct Anki {
    pub paths: PathsConfig,
    pub file_name: String,
    pub flashcards: HashMap<String, Vec<Flashcard>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Flashcard {
    pub question: String,
    pub answer: String,
}

impl PathsConfig {
    pub fn new(save_path: PathBuf, json_path: PathBuf) -> PathsConfig {
        PathsConfig {
            save_path,
            json_path,
        }
    }
}

impl Anki {
    pub fn new(paths: PathsConfig, file_name: String) -> Self {
        Self {
            paths,
            file_name,
            flashcards: HashMap::new(),
        }
    }

    pub fn save_json(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = fs::File::create(&self.paths.json_path)?;
        serde_json::to_writer_pretty(file, &self.flashcards)?;
        Ok(())
    }

    pub fn add_flashcard(&mut self, file: &str, card: Flashcard) {
        self.flashcards
            .entry(file.to_string())
            .or_insert_with(Vec::new)
            .push(card);
    }

    pub fn remove_flashcard(&mut self, file: &str, index: usize) -> Option<Flashcard> {
        self.flashcards.get_mut(file).and_then(|cards| {
            if index < cards.len() {
                Some(cards.remove(index))
            } else {
                None
            }
        })
    }

    pub fn clear(&mut self) {
        self.flashcards.clear();
    }

    pub fn load_all_json_to_flashcard_from_dir(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json_files = util::directory::read_directory(&self.paths.json_path, ".json")?;
        for json_file in json_files {
            let (file_name, flashcards) = Self::read_flashcards_from_file(&json_file)?;
            self.flashcards.insert(file_name, flashcards);
        }
        Ok(())
    }

    pub fn read_flashcards_from_file<P: AsRef<Path>>(
        path: P,
    ) -> Result<(String, Vec<Flashcard>), Box<dyn std::error::Error>> {
        let path_ref = path.as_ref();

        let file_name = path_ref
            .file_stem()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("unknown")
            .to_string();

        let file = File::open(path_ref)?;
        let reader = BufReader::new(file);

        let flashcards: Vec<Flashcard> = serde_json::from_reader(reader)?;

        Ok((file_name, flashcards))
    }
}
