use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use rustc_hash::FxBuildHasher;
use crate::bilayout::Layout;
use crate::populate::new_populated;

pub fn archive_layout(layout: Layout) {

    let serialized = serde_json::to_string(&layout).unwrap();
    let mut file = OpenOptions::new().write(true).open("files/optimized_layout.json").unwrap();
    file.set_len(0).expect("File clear failed.");
    file.write_all(serialized.as_ref()).expect("Exception: Couldn't write to file.");

    let serialized = serde_json::to_string(&layout.get_layer_list()).unwrap();
    let mut file = OpenOptions::new().write(true).open("files/optimized_layout_layers.json").unwrap();
    file.write_all(serialized.as_ref()).expect("Exception: Couldn't write to file.");

    let serialized = serde_json::to_string(&layout.get_base_layer()).unwrap();
    let mut file = OpenOptions::new().write(true).open("files/optimized_layout_base.json").unwrap();
    file.write_all(serialized.as_ref()).expect("Exception: Couldn't write to file.");
}

pub fn load_layout(file_path: &str) -> Layout {

    let mut data = String::new();
    let mut f = File::open(file_path).expect("Exception: Unable to open file.");
    f.read_to_string(&mut data).unwrap();
    let layout: Layout = serde_json::from_str(&data).unwrap();

    layout
}