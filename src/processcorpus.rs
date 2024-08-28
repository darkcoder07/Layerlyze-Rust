use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::vec::IntoIter;
use rustc_hash::{FxBuildHasher, FxHashMap};
use regex::Regex;
use bincode::{config, Decode, Encode};
use crate::processcorpus;

/// Processes given corpora into a list of bigrams to generate layouts with, and lists of processed
/// corpora in the form of words to iterate over with simulated typing.

fn lines_from_corpus(file_path: &str) -> IntoIter<String> {
    let f = File::open(file_path).expect("Exception: Unable to open file.");
    let f = BufReader::new(f);
    let mut output_line:Vec<String> = vec![];

    for line in f.lines() {
        let chunk = line.unwrap();

        match chunk.as_str() {
            "" => {}
            other => output_line.push(chunk)
        }
    }
    output_line.into_iter()
}

fn processed_words_from_lines(lines: IntoIter<String>) -> HashMap<String, u32, FxBuildHasher> {
    let mut word_map = FxHashMap::default();
    let pattern = Regex::new("[^a-z,.';]").unwrap();  //this will replace space with space, which is slow, but it's fineeee

    //this code is garbage but it works :3
    for line in lines {
        let split_line = line.split(" ");
        for word in split_line {
            let lower = word.to_lowercase();
            let unshift = lower.replace("\"", "'").replace(":", ";");
            let unspecialed = pattern.replace_all(&*unshift, " ").to_string();
            let spaced = unspecialed + " ";
            *word_map.entry(spaced).or_insert(0) += 1;
        }

    }
    word_map
}

fn processed_bigrams_from_lines(lines: IntoIter<String>) -> HashMap<String, u32, FxBuildHasher> {
    let mut bigram_map = FxHashMap::default();
    let pattern = Regex::new("[^a-z,.';]").unwrap();  //this will replace space with space, which is slow, but it's fineeee
    for line in lines {
        let mut i = 0;
        let lower = line.to_lowercase();
        let unshift = lower.replace("\"", "'").replace(":", ";");
        let unspecialed = pattern.replace_all(&*unshift, " ").to_string();
        let spaced = unspecialed + " ";
        while i + 2 < spaced.chars().count() {  // ASCII only pls pls ty
            let slice = &spaced[i..i+2];
            if slice.contains(" ") {
                i += 1;
                continue;
            }
            *bigram_map.entry(slice.to_owned()).or_insert(0) += 1;
            i += 1;
        }
    }
    bigram_map
}

fn cull_words(mut map: HashMap<String, u32, FxBuildHasher>, mut bound: u32) -> HashMap<String, u32, FxBuildHasher> {
    for (key, value) in &mut map.clone() {
        if value < &mut bound {
            map.remove(key);
        }
    }
    map
}

fn map_to_file(file_path: &str, map: HashMap<String, u32, FxBuildHasher>) {
    let serialized = serde_json::to_string(&map).unwrap();
    let mut file = OpenOptions::new().write(true).open(file_path).unwrap();
    file.write_all(serialized.as_ref()).expect("Exception: Couldn't write to file.");

}

pub(crate) fn update_corpus_files(corpus_file_path: &str) {
    let bigrams = processed_bigrams_from_lines(lines_from_corpus(corpus_file_path));
    let words = processed_words_from_lines(lines_from_corpus(corpus_file_path));
    let diced_words = cull_words(words.clone(), 8);

    map_to_file("files/mr_culled_words.json", diced_words);
    map_to_file("files/mr_cleaned_bigrams.json", bigrams);
    map_to_file("files/mr_processed_words.json", words);
}