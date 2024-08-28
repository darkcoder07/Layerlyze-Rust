use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};

/// Individual finger units that keep track of keypresses in their key column.
const LIST_TO_COORDINATES: [(i64, i64); 30] = [
    (0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0),
    (0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1), (8, 1), (9, 1), 
    (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2), (6, 2), (7, 2), (8, 2), (9, 2)
];
const SFR_DIST: f64 = 0.6;
const TRAVEL_DIST: f64 = 0.0002;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Finger {
    name: String,
    history: [[usize; 2]; 24],
    speed: f64,
    iter: usize,
}

impl Finger {
    pub(crate) fn new(name: String) -> Finger {
        Finger {
            name,
            history: [[0, 0]; 24],
            speed: 0.0,
            iter: 0,
        }
    }

    pub fn press(&mut self, index: usize, time: usize, freq: u32, word: &str) {
        self.history[self.iter] = [index, time];
        if self.iter > 0 {
            let delta_time = time - self.history[self.iter - 1][1];
            let prev_coords = LIST_TO_COORDINATES[self.history[self.iter - 1][0]];
            let curr_coords = LIST_TO_COORDINATES[index];
            let x_dist = curr_coords.0 - prev_coords.0;
            let y_dist = curr_coords.1 - prev_coords.1;
            let dist = ((x_dist.pow(2) + y_dist.pow(2)) as f64).sqrt();

            if delta_time < 20 {
                if dist > 0.0 {
                    let speed_gain = dist / (10_u64.pow((delta_time - 1) as u32) as f64);
                    self.speed += speed_gain * freq as f64;
                } else {
                    let speed_gain = SFR_DIST / (10_u64.pow((delta_time - 1) as u32) as f64);
                    self.speed += speed_gain * freq as f64;
                }
            }
        } else if LIST_TO_COORDINATES[index].1 != 1 {
            self.speed += TRAVEL_DIST * freq as f64;
        }
        self.iter += 1;
    }

    pub fn clear_history(&mut self) {
        self.iter = 0;
        self.history = [[0, 0]; 24];
    }

    pub fn reset_speed(&mut self) {
        self.speed = 0.0;
    }

    pub fn get_speed(&self) -> f64 {
        self.speed
    }
}

impl fmt::Display for Finger {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}