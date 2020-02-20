#[path = "../reader.rs"]
mod reader;

pub fn run () {
  let contents = reader::read("src/day01/input1.txt".to_string());

  let sum = contents.split("\n").fold(0, |acc, s| {
    let num: i32 = s.parse().unwrap();
    acc + ((num / 3 as i32) - 2)
  });

  println!("{}", sum);
}