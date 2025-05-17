use crate::anki::json_to_flash_card::{Anki, Flashcard, PathsConfig};
use crate::util;
use genanki_rs::{Deck, Field, Model, Note, Template};
use rand::Rng;
use std::{error::Error, fs::File, io::BufReader, path::Path};

pub fn create_anki(anki: Anki) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();
    let my_model = Model::new(
        rng.random_range(100000000..=9999999999),
        &anki.file_name,
        vec![Field::new("Question"), Field::new("Anwser")],
        vec![Template::new("QA_Card")
            .qfmt("{{Question}}")
            .afmt(r#"{{FrontSide}}<hr id="anwser">{{Anwser}}"#)],
    );
    let mut my_deck = Deck::new(
        rng.random_range(100000000..=9999999999),
        &format!("{}::{}", anki.file_name.as_str(), anki.file_name.as_str()),
        "Deck",
    );
    for (file_name, flashcards_vec) in anki.flashcards.iter() {
        for flashcard in flashcards_vec {
            let my_note = Note::new(
                my_model.clone(),
                vec![&flashcard.question.clone(), &flashcard.answer.clone()],
            )?;
            my_deck.add_note(my_note);
        }
    }
    match my_deck.write_to_file(
        &anki
            .paths
            .save_path
            .into_os_string()
            .to_str()
            .unwrap_or_default(),
    ) {
        Ok(()) => println!("OK"),
        Err(e) => println!("{e:?}"),
    }
    Ok(())
}
