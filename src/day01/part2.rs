use crate::reader;

pub fn run() {
  println!("Day 1 Part 2");
  let contents = reader::read("src/day01/input2.txt".to_string());

  let sum = contents.split("\n").fold(0, |acc, s| {
    let num: i32 = s.parse().unwrap();
    acc + cal(num)
  });

  println!("{}", sum);
}

fn cal(num: i32) -> i32 {
  let mut sum = 0;
  let mut current = num;
  loop {
    current = (current / 3 as i32) - 2;
    if current <= 0 {
      break;
    }
    sum += current;
  }
  sum
}
