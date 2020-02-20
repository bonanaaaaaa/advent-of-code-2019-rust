#[path = "../reader.rs"]
mod reader;

use crate::day03::line::Line;
use crate::day03::point::Point;
use crate::day03::utils;

pub fn run() {
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

fn decode(path: &str, p: &mut Point) -> Line {
  let tup = path.split_at(1);
  let direction = tup.0;
  let distance: i32 = tup.1.parse().unwrap();
  let old_point = Point {
    x: p.x.clone(),
    y: p.y.clone(),
  };
  match direction {
    "U" => p.y += distance,
    "D" => p.y -= distance,
    "L" => p.x -= distance,
    "R" => p.x += distance,
    _ => (),
  };

  Line::new(&old_point, p)
}
