
use rustc_hash::FxHashMap;
use rand::Rng;
use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use crate::finger::Finger;

/// Stores bigram layouts with arrays for a fingermap, bigrams, and base layer, with hashmaps
/// to quickly locate the indexes of bigrams or bases without having to iterate through arrays.
/// Some of this code is really scuffed, and I had to use unsafe rust for...reasons.

const EMPTY: &str = "__";
const LAYER_SIZE: usize = 30;  // we are assuming that the number of layers is equal to the layer size
const DEFAULT_BASE: [&str; 30] = ["b", "h", "y", "u", "g", "x", "f", "o", "l", "j",
                                  "a", "s", "t", "e", "i", ".", "n", "d", "r", ",",
                                  "k", "v", "m", "p", "q", "z", "c", "'", "w", ";"];
// we don't make a default bigram list because that would take too much work tbh

const DEFAULT_FINGER_LIST: [&str; 30] = ["lp", "lr", "lm", "li", "li", "ri", "ri", "rm", "rr", "rp",
                                         "lp", "lr", "lm", "li", "li", "ri", "ri", "rm", "rr", "rp",
                                         "lp", "lr", "lm", "li", "li", "ri", "ri", "rm", "rr", "rp"];

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Layout {
    layer_list: [[String; 30]; 30],
    base_layer: [String; 30],
    bigram_map: FxHashMap<String, (usize, usize)>,
    base_map: FxHashMap<String, usize>,
    finger_list: [String; 30],
    rp: Finger,
    rr: Finger,
    rm: Finger,
    ri: Finger,
    li: Finger,
    lm: Finger,
    lr: Finger,
    lp: Finger,
    t: Finger
}

impl Layout {
    pub fn new() -> Layout {
        Layout {
            base_layer: DEFAULT_BASE.map(String::from),
            layer_list: core::array::from_fn(|_i| core::array::from_fn(|_i| EMPTY.to_owned()).clone()),  // I LOVE RUST
            bigram_map: FxHashMap::default(),
            base_map: build_base_map(),
            finger_list: DEFAULT_FINGER_LIST.map(String::from),
            rp: Finger::new(String::from("rp")),
            rr: Finger::new(String::from("rr")),
            rm: Finger::new(String::from("rm")),
            ri: Finger::new(String::from("ri")),
            li: Finger::new(String::from("li")),
            lm: Finger::new(String::from("lm")),
            lr: Finger::new(String::from("lr")),
            lp: Finger::new(String::from("lp")),
            t: Finger::new(String::from("t"))
        }
    }
    pub fn new_specific( layer_list: [[String; 30]; 30], base_layer: [String; 30],
                         bigram_map: FxHashMap<String, (usize, usize)>, base_map: FxHashMap<String,
                         usize>, finger_list: [String; 30], rp: Finger, rr: Finger, rm: Finger, ri: Finger,
                         li: Finger, lm: Finger, lr: Finger, lp: Finger, t: Finger) -> Layout {
        Layout {
            layer_list,
            base_layer,
            bigram_map,
            base_map,
            finger_list,
            rp,
            rr,
            rm,
            ri,
            li,
            lm,
            lr,
            lp,
            t
        }
    }

    pub fn swap_interlayer_bigrams(&mut self) -> (usize, usize, usize, usize){
        let l1 = Self::rand_index();
        let l2 = Self::rand_index();
        let i1 = Self::rand_index();
        let i2 = Self::rand_index();

        let bi1 = self.layer_list[l1][i1].clone();
        let bi2 = self.layer_list[l2][i2].clone();

        unsafe {  std::ptr::swap(self.bigram_map.get_mut(&bi2).unwrap(), self.bigram_map.get_mut(&bi1).unwrap()); }

        //rip desshaw
        self.layer_list[l1][i1] = bi2;
        self.layer_list[l2][i2] = bi1;
        (l1, l2, i1, i2)
    }

    pub fn swap_interlayer_known_bigrams (&mut self, l1: usize, l2: usize, i1: usize, i2: usize) {

        let bi1 = self.layer_list[l1][i1].clone();
        let bi2 = self.layer_list[l2][i2].clone();
        unsafe { std::ptr::swap(self.bigram_map.get_mut(&bi2).unwrap(), self.bigram_map.get_mut(&bi1).unwrap()); }

        self.layer_list[l1][i1] = bi2;
        self.layer_list[l2][i2] = bi1;
    }

    pub fn swap_intralayer_bigrams(&mut self) -> (usize, usize, usize) {
        let layer_index = Self::rand_index();
        let i1 = Self:: rand_index();
        let i2 = Self:: rand_index();
        let bi1 = &self.layer_list[layer_index][i1];
        let bi2 = &self.layer_list[layer_index][i2];
        unsafe { std::ptr::swap(self.bigram_map.get_mut(bi2).unwrap(), self.bigram_map.get_mut(bi1).unwrap()); }

        self.layer_list[layer_index].swap(i1, i2);
        (layer_index, i1, i2)
    }

    pub fn swap_intralayer_known_bigrams(&mut self, layer_index: usize, i1: usize, i2: usize) {

        let bi1 = &self.layer_list[layer_index][i1];
        let bi2 = &self.layer_list[layer_index][i2];
        unsafe { std::ptr::swap(self.bigram_map.get_mut(bi2).unwrap(), self.bigram_map.get_mut(bi1).unwrap()); }

        self.layer_list[layer_index].swap(i1, i2);
    }

    pub fn swap_bases(&mut self) -> (usize, usize) {
        let i1 = Self::rand_index();
        let i2 = Self::rand_index();
        let b1 = &self.base_layer[i1];
        let b2 = &self.base_layer[i2];
        //dictionary before layer because of the references...
        //if this is too slow, consider swapping just the keys, not the values?
        //UNSAFE ATTACK
        unsafe { std::ptr::swap(self.base_map.get_mut(b2).unwrap(), self.base_map.get_mut(b1).unwrap()); }

        self.base_layer.swap(i1, i2);
        (i1, i2)
    }

    pub fn swap_known_bases(&mut self, i1: usize, i2: usize) {
        let b1 = &self.base_layer[i1];
        let b2 = &self.base_layer[i2];
        //dictionary before layer because of the references...
        //if this is too slow, consider swapping just the keys, not the values? or not re-do things ahahhryihtryh
        //UNSAFE ATTACK
        unsafe { std::ptr::swap(self.base_map.get_mut(b2).unwrap(), self.base_map.get_mut(b1).unwrap()); }


        self.base_layer.swap(i1, i2);

    }

    pub fn populate_with_bigram(&mut self, bigram:String) {
        for _i in 0..10000{
            let layer_index = Self::rand_index();
            let layer = &mut self.layer_list[layer_index];
            let bigram_index = Self::rand_index();
            if layer[bigram_index] == *EMPTY {
                layer[bigram_index].clone_from(&bigram);
                self.bigram_map.insert(bigram.clone(), (layer_index, bigram_index));
                return
            }
        }
        println!("Exception: Bigram add failure! Were all slots in the layout already taken?")
    }

    pub fn find_finger(&mut self, index: usize) -> &mut Finger {
        match self.finger_list[index].as_str() {
            "rp" => &mut self.rp,
            "rr" => &mut self.rr,
            "rm" => &mut self.rm,
            "ri" => &mut self.ri,
            "li" => &mut self.li,
            "lm" => &mut self.lm,
            "lr" => &mut self.lr,
            "lp" => &mut self.lp,
            "t" => &mut self.t,
            _ => panic!()
        }
    }

    pub fn rand_index() -> usize {
        let index = rand::thread_rng().gen_range(0..LAYER_SIZE);
        index
    }

    pub fn get_bigram_map(&self) -> &FxHashMap<String, (usize, usize)> { &self.bigram_map }

    pub fn get_base_map(&self) -> &FxHashMap<String, usize> {
        &self.base_map
    }

    pub fn clear_all_fingers(&mut self) {
        self.t.clear_history();
        self.lp.clear_history();
        self.lr.clear_history();
        self.lm.clear_history();
        self.li.clear_history();
        self.ri.clear_history();
        self.rm.clear_history();
        self.rr.clear_history();
        self.rp.clear_history();
    }

    pub fn reset_speed_all_fingers(&mut self) {
        self.t.reset_speed();
        self.lp.reset_speed();
        self.rp.reset_speed();
        self.rm.reset_speed();
        self.lm.reset_speed();
        self.rr.reset_speed();
        self.lr.reset_speed();
        self.ri.reset_speed();
        self.li.reset_speed();
    }

    pub fn get_speed_all_fingers(&mut self) -> f64 {
        let mut speed = 0.0;
        speed += self.li.get_speed();
        speed += self.ri.get_speed();
        speed += self.rr.get_speed();
        speed += self.lr.get_speed();
        speed += self.lm.get_speed();
        speed += self.rm.get_speed();
        speed += self.rp.get_speed();
        speed += self.lp.get_speed();
        speed += self.t.get_speed();
        speed
    }

    pub fn get_rand_interlayer(&self) -> (usize, usize, usize, usize, String, String) {
        let l1 = Self::rand_index();
        let l2 = Self::rand_index();
        let i1 = Self::rand_index();
        let i2 = Self::rand_index();

        let bi1 = self.layer_list[l1][i1].clone();
        let bi2 = self.layer_list[l2][i2].clone();

        (l1, l2, i1, i2, bi1, bi2)
    }

    pub fn get_rand_bigram(&self) -> (usize, usize, String) {
        let l1 = Self::rand_index();
        let i1 = Self::rand_index();

        let bi1 = self.layer_list[l1][i1].clone();

        (l1, i1, bi1)
    }

    pub fn set_layer_index(&mut self, l: usize, i: usize, bi: String) {

         self.layer_list[l][i].clone_from(&bi);
         self.bigram_map.remove(&bi);
         self.bigram_map.insert(bi, (l, i));

    }

    pub fn get_rand_intralayer(&self) -> (usize, usize, usize, String, String) {

        let layer_index = Self::rand_index();
        let i1 = Self:: rand_index();
        let i2 = Self:: rand_index();
        let bi1 = self.layer_list[layer_index][i1].clone();
        let bi2 = self.layer_list[layer_index][i2].clone();

        (layer_index, i1, i2, bi1, bi2)
    }

    pub fn get_rand_base(&self) -> (usize, usize, String, String) {
        let i1 = Self::rand_index();
        let i2 = Self::rand_index();
        let b1 = self.base_layer[i1].clone();
        let b2 = self.base_layer[i2].clone();

        (i1, i2, b1, b2)
    }

    pub fn get_one_rand_base(&self) -> (usize, String) {
        let i1 = Self::rand_index();
        let b1 = self.base_layer[i1].clone();

        (i1, b1)
    }

    pub fn set_base_index(&mut self, index: usize, base: String) {

         self.base_layer[index].clone_from(&base);
         self.base_map.remove(&base);
         self.base_map.insert(base, index);

    }

    pub fn get_base_layer(&self) -> &[String; 30] { &self.base_layer }

    pub fn get_layer_list(&self) -> &[[String; 30]; 30] { &self.layer_list }

}
impl fmt::Display for Layout {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.layer_list)
    }
}

fn build_base_map() -> FxHashMap<String, usize> {
    let mut map = FxHashMap::default();
    map.insert(String::from("b"), 0);
    map.insert(String::from("h"), 1);
    map.insert(String::from("y"), 2);
    map.insert(String::from("u"), 3);
    map.insert(String::from("g"), 4);
    map.insert(String::from("x"), 5);
    map.insert(String::from("f"), 6);
    map.insert(String::from("o"), 7);
    map.insert(String::from("l"), 8);
    map.insert(String::from("j"), 9);
    map.insert(String::from("a"), 10);
    map.insert(String::from("s"), 11);
    map.insert(String::from("t"), 12);
    map.insert(String::from("e"), 13);
    map.insert(String::from("i"), 14);
    map.insert(String::from("."), 15);
    map.insert(String::from("n"), 16);
    map.insert(String::from("d"), 17);
    map.insert(String::from("r"), 18);
    map.insert(String::from(","), 19);
    map.insert(String::from("k"), 20);
    map.insert(String::from("v"), 21);
    map.insert(String::from("m"), 22);
    map.insert(String::from("p"), 23);
    map.insert(String::from("q"), 24);
    map.insert(String::from("z"), 25);
    map.insert(String::from("c"), 26);
    map.insert(String::from("w"), 27);
    map.insert(String::from("'"), 28);
    map.insert(String::from(";"), 29);
    map
}