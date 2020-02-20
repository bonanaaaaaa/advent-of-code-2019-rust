#[path = "../reader.rs"]
mod reader;

use crate::day03::point::Point;
use crate::day03::line::Line;

pub fn run() {
  let contents = reader::read("src/day03/input2.txt".to_string());

  let paths: Vec<String> = contents.split("\n").map(|s| String::from(s)).collect();

  let mut origin = Point { x: 0, y: 0 };

  let mut line1: Vec<Line> = Vec::new();
  paths[0].split(",").for_each(|path| {
    let l = decode(path, &mut origin);
    line1.push(l);
  });

  origin = Point { x: 0, y: 0 };
  let mut line2: Vec<Line> = Vec::new();
  paths[1].split(",").for_each(|path| {
    let l = decode(path, &mut origin);
    line2.push(l);
  });

  let mut min_steps = std::i32::MAX;

  // println!("{:?}", line1);
  // println!("{:?}", line2);
  let mut i = 0;

  for l1 in &line1 {
    let mut j = 0;
    for l2 in &line2 {
      match l1.intersect(l2) {
        Some(p) => {
          // println!("Some: {:?} {:?}", l1, l2);
          // println!("{:?}", p);

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

  let l = Line::new(&old_point, p);

  l
}
