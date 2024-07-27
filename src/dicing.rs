use std::collections::HashMap;
use rustc_hash::{FxBuildHasher, FxHashMap};
use crate::bilayout::Layout;

/// Separates a corpus into ~900 slices corresponding to each bigram in a layout. Each slice contains
/// the set of words that encounter that bigram while normally typing with the layout. (I.e, if "the"
/// is a word, "th" will include "the", but not "he", since normally "the" is typed with "th" + "e_").

pub fn slice_and_dice(initial_layout: &Layout, word_map: &HashMap<String, u32, FxBuildHasher>) -> HashMap<String, HashMap<String, u32, FxBuildHasher>, FxBuildHasher> {

    let mut corpus_by_bigrams = FxHashMap::default();
    for (bigram, _) in initial_layout.get_bigram_map() {
        let mut slice = FxHashMap::default();
        for (word, freq) in word_map {
            let mut t = 0;
            while t + 1 < word.len() {
                let chunk = &word[t..t+2];
                let vec: Vec<char> = chunk.chars().collect();
                if vec[0] == ' ' {
                    t += 1;
                    continue
                }
                else if vec[1] == ' ' {
                    t += 2; // I know I SHOULD handle the base layer here, but it's complicated...
                }
                else {
                    if chunk == bigram {
                        slice.insert(word.clone(), freq.clone());
                    }
                     t += 2;
                }
            }
        }
        corpus_by_bigrams.insert(bigram.clone(), slice);
    }
    corpus_by_bigrams
}

pub fn base_slice_and_dice(initial_layout: &Layout,  word_map: &HashMap<String, u32, FxBuildHasher>) -> HashMap<String, HashMap<String, u32, FxBuildHasher>, FxBuildHasher> {
    let mut corpus_by_bases = FxHashMap::default();
    for (base, _) in initial_layout.get_base_map() {
        let mut slice = FxHashMap::default();
        for (word, freq) in word_map {
            let mut t = 0;
            while t + 1 < word.len() {
                let chunk = &word[t..t+2];
                let vec: Vec<char> = chunk.chars().collect();
                if vec[0] == ' ' {
                    t += 1;
                    continue
                }
                else if vec[1] == ' ' {

                    if &chunk[..1] == base {
                        slice.insert(word.clone(), freq.clone());
                    }

                    t += 2;
                }
                else {
                    t += 2;
                }
            }
        }
        corpus_by_bases.insert(base.clone(), slice);
    }
    corpus_by_bases
}

pub fn combine(mut map1: HashMap<String, u32, FxBuildHasher>, map2: HashMap<String, u32, FxBuildHasher>) -> HashMap<String, u32, FxBuildHasher> {

    map1.extend(map2);
    map1
}