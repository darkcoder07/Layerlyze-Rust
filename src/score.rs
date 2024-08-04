use std::collections::HashMap;
use rustc_hash::FxBuildHasher;
use crate::bilayout::Layout;


/// Runs through the given corpus and performs simulated typing on each word by sending presses to the
/// respective fingers used while typing each bigram of the current word.

//we should optimize this whole thing by assuming ASCII and changing to vectors of bytes. I swear
//might be an issue that we need to mutably borrow the layout to score it, I'm not sure
pub(crate) fn score(layout: &mut Layout, processed_word_map: &HashMap<String, u32, FxBuildHasher>) -> f64 {
    for (word, freq) in processed_word_map {  //ASCII only until I get good ahhhh
        let mut t = 0;
        while t + 1 < word.len() {  //ASCIIIII

            // I know I shouldn't need to make both a string slice AND a vector here, there really should be a faster way to do all this, don't know the right
            // library though. Rust makes strings weird since I have to deal with UTF-8 directly and stuff...
            let chunk = &word[t..t+2];
            let vec: Vec<char> = chunk.chars().collect();
            if vec[0] == ' ' {
                t += 1;
                continue
            }
            else if vec[1] == ' ' {
                let base_index = layout.get_base_map().get(&chunk[..1]).unwrap().to_owned();
                layout.find_finger(base_index).press(base_index, t, *freq, word);
                t += 2;
            }
            else {
                let chunk_index = layout.get_bigram_map().get(chunk).unwrap().to_owned();
                layout.find_finger(chunk_index.0).press(chunk_index.0, t, *freq, word);
                t = t + 1;
                layout.find_finger(chunk_index.1).press(chunk_index.1, t, *freq, word);
                t = t + 1;
            }
        }
        layout.clear_all_fingers();

    }
    let total_speed = layout.get_speed_all_fingers();
    layout.reset_speed_all_fingers();

    total_speed
}

