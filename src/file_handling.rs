use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

pub fn get_input(file_name: &str) -> String {
  let path = format!("assets/{}", file_name);
  let path = Path::new(&path);
  let mut file = File::open(&path).expect("Couldn't open file");
  let mut s = String::new();
  file.read_to_string(&mut s).expect("Couldn't read file");
  s
}
