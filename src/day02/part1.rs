#[path = "../reader.rs"]
mod reader;

pub fn run() {
  println!("Day 2 Part 1");

  let contents = reader::read("src/day02/input1.txt".to_string());

  let mut arr: Vec<i32> = contents
    .split(",")
    .map(|s| s.trim().parse().unwrap())
    .collect();

  arr[1] = 11;
  arr[2] = 11;

  let mut c = 0;
  loop {
    let op = arr[c];
    match op {
      1 | 2 => {
        let p1 = arr[c + 1] as usize;
        let p2 = arr[c + 2] as usize;
        let pos = arr[c + 3] as usize;
        arr[pos] = if op == 1 {
          arr[p1] + arr[p2]
        } else {
          arr[p1] * arr[p2]
        };
        c = c + 4;
      }
      99 => {
        break;
      }
      _ => {
        eprintln!("Something wrong");
        break;
      }
    }
  }

  println!("{}", arr[0]);
}
