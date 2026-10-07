use std::{fs::File, io::{BufRead, BufReader, Error}, path::Path};

pub fn get_word_iter(path: &Path) -> Result<Box<dyn Iterator<Item = String>>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);    
    let word_iter = reader.lines().flat_map(|line| {
        line.unwrap()
            .split_whitespace()
            .map(|w| w.to_string()) // Or keep as &str if lifetime-bound
            .collect::<Vec<String>>()
            .into_iter()
    });

    Ok(Box::new(word_iter))
}