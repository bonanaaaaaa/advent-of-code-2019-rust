use crate::reader;

pub fn run() {
  println!("Day 4 Part 2");
  let contents = reader::read("src/day04/input2.txt".to_string());

  let num: Vec<i32> = contents.split("-").map(|s| s.parse().unwrap()).collect();
  let start = num[0];
  let end = num[1];

  let mut count = 0;

  for i in 1..10 {
    count += think(start, end, 0, Vec::new(), i, 4, i * 10i32.pow(5))
  }

  println!("{}", count);
}

fn think(
  start: i32,
  end: i32,
  repeat: i32,
  his: Vec<i32>,
  num: i32,
  digit: i32,
  digits: i32,
) -> i32 {
  if digit < 0 {
    if check(his, repeat) && start <= digits && digits <= end {
      return 1;
    } else {
      return 0;
    }
  }

  let mut count = 0;

  for i in num..10 {
    let r = if num == i { repeat + 1 } else { 0 };

    let h = if num != i {
      let mut n_vec = his.clone();
      n_vec.push(repeat);
      n_vec.to_vec()
    } else {
      his.clone()
    };

    count += think(
      start,
      end,
      r,
      h,
      i,
      digit - 1,
      digits + (i * 10i32.pow(digit as u32)),
    );
  }

  count
}

fn check(v: Vec<i32>, r: i32) -> bool {
  if r == 1 {
    return true;
  }
  for n in v {
    if n == 1 {
      return true;
    }
  }
  false
}
