use std::collections::HashMap;
use rustc_hash::{FxBuildHasher, FxHashMap};
use crate::bilayout::Layout;
use crate::score::score;
use rand::prelude::*;
use crate::dicing;
use crate::populate::new_populated;

/// Optimizes using slices of the corpus with each iteration, instead of the full corpus,
/// providing a major speed gain. Each random iteration, two bigrams are randomly selected
/// to be swapped. All words featuring either of the two bigrams are combined as the corpus slice,
/// and the layout is scored twice, once before the swap and once after. If the after-swap score
/// is lower, the delta between the two scores is subtracted from the overall layout score and
/// the swap is kept.
///
/// Rough implementation of a basic random restart algorithm. A bit slow to run given current gen speed...

pub fn restart_optimize(processed_word_corpus: HashMap<String, u32, FxBuildHasher>, bigram_map: HashMap<String, u32, FxBuildHasher>, max_attempts: u32, restarts: u32) -> Layout {

    let mut attempts: u32 = 0;

    let mut layout= new_populated(bigram_map.clone());

    let diced_words: HashMap<String, HashMap<String, u32, FxBuildHasher>, FxBuildHasher> = dicing::slice_and_dice(&layout, &processed_word_corpus);
    let base_diced_words: HashMap<String, HashMap<String, u32, FxBuildHasher>, FxBuildHasher> = dicing::base_slice_and_dice(&layout, &processed_word_corpus);

    let mut interlayer: (usize, usize, usize, usize, String, String) = (0, 0, 0, 0, "".to_string(), "".to_string());
    let mut intralayer: (usize, usize, usize, String, String) = (0, 0, 0, "".to_string(), "".to_string());
    let mut base: (usize, usize, String, String) = (0, 0, "".to_string(), "".to_string());
    let mut current_score = 0.0;
    let mut swap_score = 0.0;
    let mut combined = FxHashMap::default();

    let mut scores = Vec::new();

    for _i in 0..restarts {

        let mut layout= new_populated(bigram_map.clone());

        let mut best_score = score(&mut layout, &processed_word_corpus);

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
            } else if rand < 0.3 {
                intralayer = layout.get_rand_intralayer();

                let slice1 = diced_words[&intralayer.3].clone();
                let slice2 = diced_words[&intralayer.4].clone();

                combined = dicing::combine(slice1, slice2);

                current_score = score(&mut layout, &combined);

                layout.swap_intralayer_known_bigrams(intralayer.0, intralayer.1, intralayer.2);

                swap_score = score(&mut layout, &combined);
            } else {
                base = layout.get_rand_base();

                let slice1 = base_diced_words[&base.2].clone();
                let slice2 = base_diced_words[&base.3].clone();

                combined = dicing::combine(slice1, slice2);

                current_score = score(&mut layout, &combined);

                layout.swap_known_bases(base.0, base.1);

                swap_score = score(&mut layout, &combined);
            }

            let delta = current_score - swap_score;

            if delta > 0.0 {
                best_score -= delta;
                println!("{}", best_score);
                attempts = 0;
            } else {
                if 0.9 > rand && rand > 0.3 { layout.swap_interlayer_known_bigrams(interlayer.0, interlayer.1, interlayer.2, interlayer.3); } else if rand < 0.3 { layout.swap_intralayer_known_bigrams(intralayer.0, intralayer.1, intralayer.2); } else { layout.swap_known_bases(base.0, base.1); }
                // println!("Couldn't find a better layout: {} attempts.", attempts);
                attempts += 1;
            }
        }

        scores.push((layout, best_score));

        attempts = 0;
    }

    scores.sort_by(|a, b| a.1.total_cmp(&b.1));

    scores[0].clone().0
}
