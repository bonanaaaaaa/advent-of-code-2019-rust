#[path = "../reader.rs"]
mod reader;

use crate::day03::point::Point;
use crate::day03::line::Line;

pub fn run() {
  let contents = reader::read("src/day03/input1.txt".to_string());

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

  let mut min_distance = std::i32::MAX;

  // println!("{:?}", line1);
  // println!("{:?}", line2);

  for l1 in &line1 {
    for l2 in &line2 {
      match l1.intersect(l2) {
        Some(p) => {
          // println!("Some: {:?} {:?}", l1, l2);
          // println!("{:?}", p);

          let d = p.x.abs() + p.y.abs();
          if d < min_distance {
            min_distance = d;
          }
        }
        None => {
          // println!("None: {:?} {:?}", l1, l2);
          continue
        },
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

  let l = Line::new(&old_point, p);

  l
}
