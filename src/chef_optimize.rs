use std::collections::HashMap;
use rustc_hash::{FxBuildHasher, FxHashMap};
use crate::bilayout::Layout;
use crate::score::score;
use rand::prelude::*;
use crate::dicing;


/// Optimizes using slices of the corpus with each iteration, instead of the full corpus,
/// providing a major speed gain. Each random iteration, two bigrams are randomly selected
/// to be swapped. All words featuring either of the two bigrams are combined as the corpus slice,
/// and the layout is scored twice, once before the swap and once after. If the after-swap score
/// is lower, the delta between the two scores is subtracted from the overall layout score and
/// the swap is kept.

pub fn chef_optimize(mut layout: Layout, processed_word_corpus: HashMap<String, u32, FxBuildHasher>, max_attempts: u32) -> Layout {

    let mut attempts: u32 = 0;

    let mut best_score = score(&mut layout, &processed_word_corpus);

    let diced_words: HashMap<String, HashMap<String, u32, FxBuildHasher>, FxBuildHasher> = dicing::slice_and_dice(&layout, &processed_word_corpus);
    let base_diced_words: HashMap<String, HashMap<String, u32, FxBuildHasher>, FxBuildHasher> = dicing::base_slice_and_dice(&layout, &processed_word_corpus);

    let mut interlayer: (usize, usize, usize, usize, String, String) = (0, 0, 0, 0, "".to_string(), "".to_string());
    let mut intralayer: (usize, usize, usize, String, String) = (0, 0, 0, "".to_string(), "".to_string());
    let mut base: (usize, usize, String, String) = (0, 0, "".to_string(), "".to_string());
    let mut current_score = 0.0;
    let mut swap_score = 0.0;
    let mut combined = FxHashMap::default();

    while attempts < max_attempts {
        let mut rng = thread_rng();
        let rand: f64 = rng.gen();

        if 0.9 > rand && rand > 0.3 {
            interlayer = layout.get_rand_interlayer();

            let slice1 = diced_words[&interlayer.4].clone();
            let slice2 = diced_words[&interlayer.5].clone();

            combined = dicing::combine(slice1, slice2);

            current_score = score(&mut layout, &combined);

            layout.swap_interlayer_known_bigrams(interlayer.0, interlayer.1, interlayer.2, interlayer.3);

            swap_score = score(&mut layout, &combined);

        }
        else if rand < 0.3 {
            intralayer = layout.get_rand_intralayer();

            let slice1 = diced_words[&intralayer.3].clone();
            let slice2 = diced_words[&intralayer.4].clone();

            combined = dicing::combine(slice1, slice2);

            current_score = score(&mut layout, &combined);

            layout.swap_intralayer_known_bigrams(intralayer.0, intralayer.1, intralayer.2);

            swap_score = score(&mut layout, &combined);

        }
        else {
            base = layout.get_rand_base();

            let slice1 = base_diced_words[&base.2].clone();
            let slice2 = base_diced_words[&base.3].clone();

            combined = dicing::combine(slice1, slice2);

            current_score = score(&mut layout, &combined);

            layout.swap_known_bases(base.0, base.1);

            swap_score = score(&mut layout, &combined);

        }

        if swap_score < current_score {
            attempts = 0;
            let delta = current_score - swap_score;
            best_score -= delta;
            println!("{}", best_score);
        }
        else {
            if 0.9 > rand && rand > 0.3 { layout.swap_interlayer_known_bigrams(interlayer.0, interlayer.1, interlayer.2, interlayer.3); }
            else if rand < 0.3 { layout.swap_intralayer_known_bigrams(intralayer.0, intralayer.1, intralayer.2); }
            else { layout.swap_known_bases(base.0, base.1);}
            attempts += 1;
            // println!("Couldn't find a better layout: {} attempts.", attempts);
        }
    }

    layout
}