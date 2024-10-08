use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use rustc_hash::FxBuildHasher;

use crate::annealing_optimize::anneal_optimize;
use crate::darwins_optimization::asexual_optimization;
use crate::populate::new_populated;
use crate::save_and_load::{archive_layout, load_layout};
use crate::score::score;

mod finger;
mod bilayout;
mod processcorpus;
mod score;
mod populate;
mod chef_optimize;
mod annealing_optimize;
mod dicing;

mod save_and_load;
mod darwins_optimization;
mod random_restart;

//If you're trying to work with raw ASCII, however, pretty much all of the above advice goes away.
//Instead, you shouldn't be using str or String at all: you should be using &[u8] or Vec<u8>.
//Optimization goals...


/// This is just a weird amalgamation of my layout testing workflow. Not really optimized,
/// though that's not that important here.

fn main() {

    //processcorpus::update_corpus_files("files/monkeyracer.txt");
    //load_and_print();
    //typical_stuff();
    //load_and_score();
    //genetic()
}

 pub fn load_and_score() {



        let mut layout = load_layout("files/optimized_layout.json");
        let mut data = String::new();
        let mut f = File::open("files/mr_processed_words.json").expect("Exception: Unable to open file.");
        f.read_to_string(&mut data).unwrap();
        let word_map: HashMap<String, u32, FxBuildHasher> = serde_json::from_str(&data).unwrap();

        let score = score(&mut layout, &word_map);
        println!("{}", score);

    }

pub fn genetic() {

    let mut data = String::new();
    let mut f = File::open("files/mr_cleaned_bigrams.json").expect("Exception: Unable to open file.");
    f.read_to_string(&mut data).unwrap();
    let bigram_map: HashMap<String, u32, FxBuildHasher> = serde_json::from_str(&data).unwrap();

    let mut data = String::new();
    let mut f = File::open("files/mr_processed_words.json").expect("Exception: Unable to open file.");
    f.read_to_string(&mut data).unwrap();
    let word_map: HashMap<String, u32, FxBuildHasher> = serde_json::from_str(&data).unwrap();

    asexual_optimization(word_map, bigram_map,4000, 1500);
}

pub fn typical_stuff() {

    let mut data = String::new();
    let mut f = File::open("files/mr_cleaned_bigrams.json").expect("Exception: Unable to open file.");
    f.read_to_string(&mut data).unwrap();
    let bigram_map: HashMap<String, u32, FxBuildHasher> = serde_json::from_str(&data).unwrap();

    let layout = new_populated(bigram_map.clone());
    let mut data = String::new();
    let mut f = File::open("files/mr_processed_words.json").expect("Exception: Unable to open file.");
    f.read_to_string(&mut data).unwrap();
    let word_map: HashMap<String, u32, FxBuildHasher> = serde_json::from_str(&data).unwrap();

    // let diced_words = dicing::slice_and_dice(&layout, &word_map);
    // let base_diced_words = dicing::base_slice_and_dice(&layout, &word_map);

    let cool_layout = anneal_optimize(layout, word_map, 100000000,  4000000.0);
    println!("{:?}", cool_layout);
    println!("{:?}", cool_layout.get_base_layer());


    archive_layout(cool_layout);

}

pub fn load_and_print() {

    let mut layout = load_layout("files/optimized_layout.json");

    let mut file = OpenOptions::new().write(true).open("files/optimized_layout_repr.json").unwrap();
    file.set_len(0).expect("Halp");
        for layer in layout.get_layer_list() {

            for (index, bigram) in layer.iter().enumerate() {
                let serialized = serde_json::to_string(&bigram).unwrap();
                let mut file = OpenOptions::new().  append(true).open("files/optimized_layout_repr.json").unwrap();

                if index == 9 || index == 19 {
                    if let Err(e) = writeln!(file, "{}", " ".to_owned() + &serialized) {
                    eprintln!("Couldn't write to file: {}", e);}
                }
                else if let Err(e) = write!(file, "{}", " ".to_owned() + &serialized) {
                eprintln!("Couldn't write to file: {}", e);}
            }
            let mut file = OpenOptions::new().append(true).open("files/optimized_layout_repr.json").unwrap();

            if let Err(e) = writeln!(file, "{}", "".to_owned()) {
                eprintln!("Couldn't write to file: {}", e);
            }

            if let Err(e) = writeln!(file, "{}", "".to_owned()) {
                eprintln!("Couldn't write to file: {}", e);
            }

    }

    if let Err(e) = writeln!(file, "{}", "".to_owned()) {
                eprintln!("Couldn't write to file: {}", e);
    }

    if let Err(e) = writeln!(file, "{}", "BASE LAYER".to_owned()) {
                eprintln!("Couldn't write to file: {}", e);
    }

    for (index, base) in layout.get_base_layer().iter().enumerate() {
        let serialized = serde_json::to_string(&base).unwrap();
        let mut file = OpenOptions::new().append(true).open("files/optimized_layout_repr.json").unwrap();

        if index == 9 || index == 19 {
            if let Err(e) = writeln!(file, "{}", " ".to_owned() + &serialized) {
            eprintln!("Couldn't write to file: {}", e);}
        }
        else if let Err(e) = write!(file, "{}", " ".to_owned() + &serialized) {
        eprintln!("Couldn't write to file: {}", e);}
    }

}
