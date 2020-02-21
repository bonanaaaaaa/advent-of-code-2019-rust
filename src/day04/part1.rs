const START: i32 = 271973;
const END: i32 = 785961;

pub fn run() {
  println!("Day 4 Part 1");

  let mut count = 0;

  for i in 1..10 {
    count += think(false, i, 4, i * 10i32.pow(5));
  }

  println!("{}", count);
}

fn think(repeat: bool, num: i32, digit: i32, digits: i32) -> i32 {
  if digit < 0 {
    if repeat && START <= digits && digits <= END {
      return 1;
    } else {
      return 0;
    }
  }

  let mut count = 0;

  for i in num..10 {
    count += think(
      repeat || num == i,
      i,
      digit - 1,
      digits + (i * 10i32.pow(digit as u32)),
    );
  }

  count
}
