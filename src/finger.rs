use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use crate::bilayout::Layout;

/// Individual finger units that keep track of keypresses in their key column. I'm very unsure if
/// this is the best way to perform simulated typing analysis, but it works for now. Fingerspeed is
/// calculated by dividing the distance between keypresses by 2^(time between keypresses), so a
/// 2u s2fs, for example, would be 2/2^2 = 0.5. Aggregrate fingerspeed defines layout score. I may
/// change this scheme in the future to weight sfs differently. I'll probably need to collate fingers
/// into "hands" to keep track of trigram stats and scissors.

const LIST_TO_COORDINATES: [(i64, i64); 30] = [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0),
                       (8, 0), (9, 0), (0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1),
                       (6, 1), (7, 1), (8, 1), (9, 1), (0, 2), (1, 2), (2, 2), (3, 2),
                       (4, 2), (5, 2), (6, 2), (7, 2), (8, 2), (9, 2)];
 const SFR_DIST: f64 = 0.6;
 const TRAVEL_DIST: f64 = 0.0002;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Finger {
    name: String,
    history: [[usize; 2]; 24],
    speed: f64,
    iter: usize
}

 impl Finger {
     pub(crate) fn new(name: String) -> Finger {
         Finger {
             name,
             history: [[0, 0],[0, 0],[0, 0],[0, 0],[0, 0],[0, 0],[0, 0],[0, 0],[0, 0],
                 [0, 0],[0, 0],[0, 0],[0, 0],[0, 0],[0, 0],[0, 0],[0, 0],
                 [0, 0],[0, 0],[0, 0],[0, 0],[0, 0],[0, 0],[0, 0]],
             speed: 0.0,
             iter: 0

         }
     }

     pub fn press(&mut self, index: usize, time: usize, freq: u32, word: &String) {
         self.history[self.iter] = [index, time];
         if self.iter > 0 {
             let delta_time = time - self.history[self.iter - 1][1];
             let ireallyloverust = LIST_TO_COORDINATES[self.history[self.iter - 1][0]];
             let x_dist = LIST_TO_COORDINATES[index].0 - ireallyloverust.0;
             let y_dist = LIST_TO_COORDINATES[index].1 - ireallyloverust.1;
             let squared_dist = (x_dist).pow(2) + (y_dist).pow(2);
             let dist = (squared_dist as f64).sqrt();

             if delta_time < 20 { // turn this to 2 if you want only sfb analysis
                 if dist > 0.0 {
                     let speed_gain: f64 = dist / (10_u64.pow((delta_time - 1) as u32) as f64);
                     //if delta_time < 2 {println!("SFB Speed: {} Finger: {}  Word: {}", speed_gain, self.name, word);}
                     self.speed += speed_gain * freq as f64
                 } else {
                     let speed_gain: f64 = SFR_DIST / (10_u64.pow((delta_time - 1) as u32) as f64);
                     //if delta_time < 2 {println!("SFR Speed: {} Finger: {} Word: {}", speed_gain, self.name, word);}
                     self.speed += speed_gain * freq as f64
                 }
            }
         }
         else {
             if LIST_TO_COORDINATES[index].1 != 1 {
                 self.speed += TRAVEL_DIST * freq as f64
             }
         }
         self.iter += 1
     }

    pub fn clear_history(&mut self) {
        self.iter = 0;
     }

    pub fn reset_speed(&mut self) { self.speed = 0.0; }

    pub fn get_speed(&mut self) -> f64 {
         self.speed
     }
 }

impl fmt::Display for Finger {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.name)
    }
}
