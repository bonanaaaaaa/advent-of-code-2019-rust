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