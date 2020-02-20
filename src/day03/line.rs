use crate::day03::point::Point;

#[derive(PartialEq, Debug)]
enum Alignment {
  Horizontal,
  Vertical,
}

#[derive(Debug)]
pub struct Line {
  pub start: Point,
  pub end: Point,
  min: Point,
  max: Point,
  alignment: Alignment,
  pub length: i32,
}

impl Line {
  pub fn new(a: &Point, b: &Point) -> Line {
    Line {
      start: Point { x: a.x, y: a.y },
      end: Point { x: b.x, y: b.y },
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
      },
    }
  }

  pub fn intersect(&self, other: &Self) -> Option<Point> {
    match (&self.alignment, &other.alignment) {
      (Alignment::Horizontal, Alignment::Vertical) if Self::is_intersect(other, self) => {
        Some(Point {
          x: other.min.x,
          y: self.min.y,
        })
      }
      (Alignment::Vertical, Alignment::Horizontal) if Self::is_intersect(self, other) => {
        Some(Point {
          x: self.min.x,
          y: other.min.y,
        })
      }
      _ => None,
    }
  }

  fn is_intersect(vertical: &Line, horizontal: &Line) -> bool {
    return vertical.min.y < horizontal.min.y
      && horizontal.min.y < vertical.max.y
      && horizontal.min.x < vertical.min.x
      && vertical.min.x < horizontal.max.x;
  }
}
