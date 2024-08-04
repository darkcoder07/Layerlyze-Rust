use std::collections::HashMap;
use rand::distributions::{Bernoulli, Uniform};
use rustc_hash::{FxBuildHasher, FxHashMap};
use crate::bilayout::Layout;
use crate::score::score;
use rand::prelude::*;
use crate::dicing;
use crate::populate::new_populated;

const CUTOFF: f64 = 0.25; // smaller cutoff means fitter layouts are kept and more reproduction or mutation occurs

/// WIP/Likely worse than just using simulated annealing. Not really practical right now
/// anyways given current generation speed.
///
/// Genetic algorithm implementation to optimize with simple mutation operations on
/// the population or crossover operations in the future, mimicking sexual reproduction.

fn layout_population_gen(population: usize, bigram_map: HashMap<String, u32, FxBuildHasher>, word_map: &HashMap<String, u32, FxBuildHasher>) -> Vec<(Layout, f64)> {
    let mut layout_population = Vec::new();
    for _i in 0..population {
        let mut new_layout = new_populated(bigram_map.clone());
        let score = score(&mut new_layout, word_map);
        layout_population.push((new_layout, score));
    }
    layout_population
}

fn layout_crossover(l1: Layout, l2: Layout) -> (Layout, Layout) {

    let mut offspring: (Layout, Layout);

    if l1.get_layer_list() == l2.get_layer_list() && l1.get_base_layer() == l2.get_base_layer() { return (l1, l2) }




    (l1, l2)

}

fn layout_mutate(mut layout: Layout, mut former_score: f64, mut times: u32, diced_words: &HashMap<String, HashMap<String, u32, FxBuildHasher>, FxBuildHasher>, base_diced_words: &HashMap<String, HashMap<String, u32, FxBuildHasher>, FxBuildHasher>) -> (Layout, f64) {
    let mut interlayer: (usize, usize, usize, usize, String, String) = (0, 0, 0, 0, "".to_string(), "".to_string());
    let mut intralayer: (usize, usize, usize, String, String) = (0, 0, 0, "".to_string(), "".to_string());
    let mut base: (usize, usize, String, String) = (0, 0, "".to_string(), "".to_string());
    let mut current_score = 0.0;
    let mut swap_score = 0.0;
    let mut combined = FxHashMap::default();
    let mut new_score = former_score;

    let mut rng = thread_rng();

    for _i in 0..times {
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


        new_score -= delta;
    }
    (layout, new_score)
}

fn layout_safe_mutate(mut layout: Layout, mut former_score: f64, times: u32, try_limit: u32, diced_words: &HashMap<String, HashMap<String, u32, FxBuildHasher>, FxBuildHasher>, base_diced_words: &HashMap<String, HashMap<String, u32, FxBuildHasher>, FxBuildHasher>) -> (Layout, f64) {
    let mut interlayer: (usize, usize, usize, usize, String, String) = (0, 0, 0, 0, "".to_string(), "".to_string());
    let mut intralayer: (usize, usize, usize, String, String) = (0, 0, 0, "".to_string(), "".to_string());
    let mut base: (usize, usize, String, String) = (0, 0, "".to_string(), "".to_string());
    let mut current_score = 0.0;
    let mut swap_score = 0.0;
    let mut combined = FxHashMap::default();
    let mut new_score = former_score;
    let mut good_swaps = 0;
    let mut attempts = 0;
    let mut rng = thread_rng();

    loop {

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
            new_score -= delta;
            good_swaps += 1;
            attempts = 0;
        }
        else {
                if 0.9 > rand && rand > 0.3 { layout.swap_interlayer_known_bigrams(interlayer.0, interlayer.1, interlayer.2, interlayer.3); }
                else if rand < 0.3 { layout.swap_intralayer_known_bigrams(intralayer.0, intralayer.1, intralayer.2); }
                else { layout.swap_known_bases(base.0, base.1);}
                attempts += 1;
        }

        if attempts == try_limit || good_swaps == times { break }
    }
    (layout, new_score)
}



pub fn asexual_optimization(processed_word_corpus: HashMap<String, u32, FxBuildHasher>, bigram_map: HashMap<String, u32, FxBuildHasher>, generations: u32, gen_size: usize) -> Layout  {
    let mut generation = layout_population_gen(gen_size, bigram_map, &processed_word_corpus);
    let mut rng = thread_rng();
    let mut current_best: (Layout, f64) = generation[0].clone();
    let diced_words: HashMap<String, HashMap<String, u32, FxBuildHasher>, FxBuildHasher> = dicing::slice_and_dice(&current_best.0, &processed_word_corpus);
    let base_diced_words: HashMap<String, HashMap<String, u32, FxBuildHasher>, FxBuildHasher> = dicing::base_slice_and_dice(&current_best.0, &processed_word_corpus);

    for _i in 0..generations {
        generation.sort_by(|a, b| a.1.total_cmp(&b.1));
        current_best = generation[0].clone();
        println!("{}", current_best.1);

        generation.truncate((gen_size as f64 *CUTOFF).round() as usize);

        let truncated_size = generation.len();
        let makeup = gen_size - truncated_size;

        for _i in 0..makeup {
            let mut a: usize = 0;
            if a == truncated_size { a = 0; }
            let rand = rng.gen_range(1..10);
            if rand > 3 && rand < 5 {
                let new_layout = layout_mutate(generation[a].0.clone(), generation[a].1, rng.gen_range(1..5), &diced_words, &base_diced_words);
                generation.push(new_layout);
            }
            else if rng.gen_range(1..10) >= 5 {
                let new_layout = layout_safe_mutate(generation[a].0.clone(), generation[a].1, rng.gen_range(1..2), 10, &diced_words, &base_diced_words);
                generation.push(new_layout);
            }
            else {
                let new_layout: (Layout, f64) = generation[a].clone();
                generation.push(new_layout);
            }
            a += 1;
        }
    }
    current_best.0

}
