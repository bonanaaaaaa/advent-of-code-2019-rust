#[path = "../reader.rs"]
mod reader;

#[derive(Debug)]
struct Point {
  x: i32,
  y: i32,
}

impl Point {
  fn distance(&self, other: &Self) -> i32 {
    (self.x - other.x).abs() + (self.y - other.y).abs()
  }
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
  min: Point,
  max: Point,
  alignment: Alignment,
  length: i32,
}

impl Line {
  pub fn new(a: &Point, b: &Point) -> Line {
    Line {
      start: Point {
        x: a.x,
        y: a.y
      },
      end: Point {
        x: b.x,
        y: b.y,
      },
      min: Point {
        x: if a.x < b.x { a.x } else { b.x },
        y: if a.y < b.y { a.y } else { b.y },
      },
      max: Point {
        x: if a.x > b.x { a.x } else { b.x },
        y: if a.y > b.y { a.y } else { b.y },
      },
      alignment: if a.y == b.y {
        Alignment::Horizontal
      } else {
        Alignment::Vertical
      },
      length: if a.y == b.y {
        (a.x - b.x).abs()
      } else {
        (a.y - b.y).abs()
      }
    }
  }

  pub fn intersect(&self, other: &Self) -> Option<Point> {
    if self.alignment == Alignment::Horizontal && other.alignment == Alignment::Vertical {
      if self.min.x < other.min.x
        && other.min.x < self.max.x
        && other.min.y < self.min.y
        && self.min.y < other.max.y
      {
        return Some(Point {
          x: other.min.x,
          y: self.min.y,
        });
      } else {
        return None;
      }
    } else if self.alignment == Alignment::Vertical && other.alignment == Alignment::Horizontal {
      if self.min.y < other.min.y
        && other.min.y < self.max.y
        && other.min.x < self.min.x
        && self.min.x < other.max.x
      {
        return Some(Point {
          x: self.min.x,
          y: other.min.y,
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

  let mut min_steps = 99999999;

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
