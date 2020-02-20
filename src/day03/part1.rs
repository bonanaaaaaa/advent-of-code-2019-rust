#[path = "../reader.rs"]
mod reader;

#[derive(Debug)]
struct Point {
  x: i32,
  y: i32,
}

#[derive(PartialEq, Debug)]
enum Alignment {
  Horizontal,
  Vertical,
}

#[derive(Debug)]
struct Line {
  start: Point,
  end: Point,
  alignment: Alignment,
}

impl Line {
  pub fn new(a: &Point, b: &Point) -> Line {
    Line {
      start: Point {
        x: if a.x < b.x { a.x } else { b.x },
        y: if a.y < b.y { a.y } else { b.y },
      },
      end: Point {
        x: if a.x > b.x { a.x } else { b.x },
        y: if a.y > b.y { a.y } else { b.y },
      },
      alignment: if a.y == b.y {
        Alignment::Horizontal
      } else {
        Alignment::Vertical
      },
    }
  }

  pub fn intersect(&self, other: &Line) -> Option<Point> {
    if self.alignment == Alignment::Horizontal && other.alignment == Alignment::Vertical {
      if self.start.x < other.start.x
        && other.start.x < self.end.x
        && other.start.y < self.start.y
        && self.start.y < other.end.y
      {
        return Some(Point {
          x: other.start.x,
          y: self.start.y,
        });
      } else {
        return None;
      }
    } else if self.alignment == Alignment::Vertical && other.alignment == Alignment::Horizontal {
      if self.start.y < other.start.y
        && other.start.y < self.end.y
        && other.start.x < self.start.x
        && self.start.x < other.start.x
      {
        return Some(Point {
          x: self.start.x,
          y: other.start.y,
        });
      } else {
        return None;
      }
    } else {
      return None;
    }
  }
}

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

  let mut min_distance = 9999999;

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
