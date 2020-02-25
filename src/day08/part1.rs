use crate::reader;
use std::collections::HashMap;

// const WIDTH: usize = 3;
// const TALL: usize = 2;
const WIDTH: usize = 25;
const TALL: usize = 6;

pub fn run() {
  let contents = reader::read("src/day08/input1.txt".to_string());
  // let contents = "123456782012";

  let mut min_zero_digits = std::i32::MAX;
  let mut one_digits = std::i32::MIN;
  let mut two_digits = std::i32::MIN;

  let mut img_data = &contents[..];

  loop {
    let (str_layer, rest) = img_data.split_at(WIDTH * TALL);
    img_data = rest;
    let m = count(str_layer);
    let zero_digits = m.get("0").unwrap();

    if min_zero_digits > *zero_digits {
      min_zero_digits = *zero_digits;
      one_digits = *m.get("1").unwrap();
      two_digits = *m.get("2").unwrap();
    }

    if rest == "" {
      break;
    }
  }

  println!("{}, {} {}", min_zero_digits, one_digits, two_digits);
  println!("{}", one_digits * two_digits);
}

fn count(img_str: &str) -> HashMap<&str, i32> {
  let mut map: HashMap<&str, i32> = HashMap::new();
  map.insert("0", 0);
  map.insert("1", 0);
  map.insert("2", 0);

  for s in img_str.chars() {
    match s {
      '0' => {
        map.insert("0", map.get("0").unwrap() + 1);
      }
      '1' => {
        map.insert("1", map.get("1").unwrap() + 1);
      }
      '2' => {
        map.insert("2", map.get("2").unwrap() + 1);
      }
      _ => continue,
    }
  }

  map
}
