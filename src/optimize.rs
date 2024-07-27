use std::collections::HashMap;
use rustc_hash::FxBuildHasher;
use crate::bilayout::Layout;
use crate::score::score;
use rand::prelude::*;

/// Legacy optimizer code

pub fn optimize(mut layout: Layout, processed_word_corpus: HashMap<String, u32, FxBuildHasher>, max_attempts: u32) -> Layout {

    let mut attempts: u32 = 0;

    let mut best_score = score(&mut layout, &processed_word_corpus);

    while attempts < max_attempts {
        let mut rng = thread_rng();
        let rand: f64 = rng.gen();

        let mut insurance1: (usize, usize, usize, usize) = (0, 0, 0, 0);
        let mut insurance2: (usize, usize, usize) = (0, 0, 0);
        let mut insurance3: (usize, usize) = (0, 0);

        if 0.75 > rand && rand > 0.3 { insurance1 = layout.swap_interlayer_bigrams(); }
        else if rand < 0.3 { insurance3 = layout.swap_bases(); }
        else { insurance2 = layout.swap_intralayer_bigrams(); }

        let new_score = score(&mut layout, &processed_word_corpus);

        if new_score < best_score {
            attempts = 0;
            best_score = new_score;
            println!("{}", best_score);
        }
        else {
            if 0.75 > rand && rand > 0.3 {
                let (a, b, c, d) = insurance1;
                layout.swap_interlayer_known_bigrams(a, b, c, d);
            }
            else if rand < 0.3 {
                let (a, b) = insurance3;
                layout.swap_known_bases(a, b); }
            else {
                let (a, b, c) = insurance2;
                layout.swap_intralayer_known_bigrams(a, b, c); }
            attempts += 1;
            // println!("Couldn't find a better layout: {} attempts.", attempts);
        }
    }

    layout
}