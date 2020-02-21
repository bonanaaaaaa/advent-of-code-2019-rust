use crate::reader;

pub fn run() {
  println!("Day 5 Part 1");

  let contents = reader::read("src/day05/input1.txt".to_string());

  let mut arr: Vec<i32> = contents
    .split(",")
    .map(|s| s.trim().parse().unwrap())
    .collect();

  // println!("{}", arr.len());

  let mut c = 0;
  loop {
    let op = arr[c] % 100;
    let res = (arr[c] / 100) as i32;

    match op {
      1 | 2 => {
        let m1 = res % 10;
        let m2 = (res / 10) as i32 % 100;
        // println!("Before: {} => {} {}", op, m1, m2);
        let v1 = get_value(m1, c + 1, &arr);
        let v2 = get_value(m2, c + 2, &arr);
        let pos = arr[c + 3] as usize;
        // println!("After: {} => {} {} {}", op, v1, v2, pos);
        arr[pos] = if op == 1 { v1 + v2 } else { v1 * v2 };
        c = c + 4;
      }
      3 => {
        let pos = arr[c + 1] as usize;
        // println!("{} => {}", op, pos);
        arr[pos] = 1;
        c = c + 2;
      }
      4 => {
        let m = res % 10;
        let output = if m == 1 {
          arr[c + 1]
        } else {
          let pos = arr[c + 1] as usize;
          arr[pos]
        };
        if output != 0 {
          println!("{}", output);
          break;
        }
        c = c + 2;
      }
      _ => {
        eprintln!("Something wrong");
        break;
      }
    }
  }
}

fn get_value(mode: i32, v_pos: usize, arr: &Vec<i32>) -> i32 {
  match mode {
    0 => arr[arr[v_pos] as usize],
    1 => arr[v_pos],
    _ => 0,
  }
}
