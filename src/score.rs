use std::collections::HashMap;
use rustc_hash::FxBuildHasher;
use crate::bilayout::Layout;
use std::str;

pub(crate) fn score(layout: &mut Layout, processed_word_map: &HashMap<String, u32, FxBuildHasher>) -> f64 {
    for (word, freq) in processed_word_map {
        let bytes = word.as_bytes();
        let mut t = 0;
        while t + 1 < bytes.len() {
            let chunk = &bytes[t..t+2];
            unsafe {
                if chunk[0] == b' ' {
                    t += 1;
                    continue;
                } else if chunk[1] == b' ' {
                    let base_index = layout.get_base_map().get(str::from_utf8_unchecked(&[chunk[0]])).unwrap().to_owned();
                    layout.find_finger(base_index).press(base_index, t, *freq, word);
                    t += 2;
                } else {
                    let chunk_index = layout.get_bigram_map().get(str::from_utf8_unchecked(&[chunk[0], chunk[1]])).unwrap().to_owned();
                    layout.find_finger(chunk_index.0).press(chunk_index.0, t, *freq, word);
                    t += 1;
                    layout.find_finger(chunk_index.1).press(chunk_index.1, t, *freq, word);
                    t += 1;
                }
            }
        }
        layout.clear_all_fingers();
    }
    let total_speed = layout.get_speed_all_fingers();
    layout.reset_speed_all_fingers();

    total_speed
}