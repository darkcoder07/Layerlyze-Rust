use std::collections::HashMap;
use std::fs::File;
use rustc_hash::FxBuildHasher;
use crate::bilayout::Layout;

pub fn new_populated(bigram_map: HashMap<String, u32, FxBuildHasher>) -> Layout {
    let mut layout = Layout::new();

    for (bigram, _) in bigram_map {
        layout.populate_with_bigram(bigram);
    }
    layout.populate_with_bigram("__".to_owned());

    layout
}