#[derive(Debug)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

impl Point {
  pub fn distance(&self, other: &Self) -> i32 {
    (self.x - other.x).abs() + (self.y - other.y).abs()
  }
}
