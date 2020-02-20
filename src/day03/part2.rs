use crate::day03::{line::Line, utils};
use crate::reader;

pub fn run() {
  println!("Day 3 Part 2");
  let contents = reader::read("src/day03/input2.txt".to_string());

  let paths: Vec<&str> = contents.split("\n").collect();

  let line1: Vec<Line> = utils::parse(paths[0]);
  let line2: Vec<Line> = utils::parse(paths[1]);

  let mut min_steps = std::i32::MAX;
  let mut i = 0;
  for l1 in &line1 {
    let mut j = 0;
    for l2 in &line2 {
      match l1.intersect(l2) {
        Some(p) => {
          let sum_steps = i + p.distance(&l1.start) + j + p.distance(&l2.start);

          if sum_steps < min_steps {
            min_steps = sum_steps;
          }
        }
        None => (),
      }
      j += l2.length;
    }
    i += l1.length;
  }
  println!("{}", min_steps);
}
