use crate::reader;

use crate::day03::line::Line;
use crate::day03::utils;

pub fn run() {
  println!("Day 3 Part 1");
  let contents = reader::read("src/day03/input1.txt".to_string());

  let paths: Vec<&str> = contents.split("\n").collect();

  let line1: Vec<Line> = utils::parse(paths[0]);
  let line2: Vec<Line> = utils::parse(paths[1]);

  let mut min_distance = std::i32::MAX;

  for l1 in &line1 {
    for l2 in &line2 {
      match l1.intersect(l2) {
        Some(p) => {
          let d = p.x.abs() + p.y.abs();
          if d < min_distance {
            min_distance = d;
          }
        }
        None => continue,
      }
    }
  }
  println!("{}", min_distance);
}