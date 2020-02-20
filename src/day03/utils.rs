use crate::day03::{line::Line, point::Point};

pub fn parse(path: &str) -> Vec<Line> {
  let mut origin = Point { x: 0, y: 0 };
  let mut line: Vec<Line> = Vec::new();

  path.split(",").for_each(|path| {
    let l = decode(path, &mut origin);
    line.push(l);
  });

  line
}

fn decode(path: &str, p: &mut Point) -> Line {
  let tup = path.split_at(1);
  let direction = tup.0;
  let distance: i32 = tup.1.parse().unwrap();
  let old_point = Point { x: p.x, y: p.y };
  match direction {
    "U" => p.y += distance,
    "D" => p.y -= distance,
    "L" => p.x -= distance,
    "R" => p.x += distance,
    _ => (),
  };

  Line::new(&old_point, p)
}
