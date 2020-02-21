use crate::reader;

pub fn run() {
  println!("Day 4 Part 1");
  let contents = reader::read("src/day04/input1.txt".to_string());

  let num: Vec<i32> = contents.split("-").map(|s| s.parse().unwrap()).collect();
  let start = num[0];
  let end = num[1];

  let mut count = 0;

  for i in 1..10 {
    count += think(start, end, false, false, i, 4, i * 10i32.pow(5))
  }

  println!("{}", count);
}

fn think(
  start: i32,
  end: i32,
  repeat: bool,
  is_decrease: bool,
  num: i32,
  digit: i32,
  digits: i32,
) -> i32 {
  if is_decrease {
    return 0;
  }
  if digit < 0 {
    if repeat && !is_decrease && start <= digits && digits <= end {
      // println!("{}", digits);
      return 1;
    } else {
      return 0;
    }
  }

  let mut count = 0;

  for i in 0..10 {
    count += think(
      start,
      end,
      repeat && num == i,
      is_decrease || i < num,
      i,
      digit - 1,
      digits + (i * 10i32.pow(digit as u32)),
    );
  }

  count
}
