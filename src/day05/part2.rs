use crate::reader;

pub fn run() {
  println!("Day 5 Part 2");

  let contents = reader::read("src/day05/input2.txt".to_string());

  let mut arr: Vec<i32> = contents
    .split(",")
    .map(|s| s.trim().parse().unwrap())
    .collect();

  let input = 5;

  let mut ptr = 0;
  loop {
    let op = arr[ptr] % 100;
    let res = (arr[ptr] / 100) as i32;

    match op {
      1 | 2 => {
        let m1 = res % 10;
        let m2 = (res / 10) as i32 % 100;

        let v1 = get_value(m1, ptr + 1, &arr);
        let v2 = get_value(m2, ptr + 2, &arr);
        let pos = arr[ptr + 3] as usize;

        arr[pos] = if op == 1 { v1 + v2 } else { v1 * v2 };
        ptr = ptr + 4;
      }
      3 => {
        let pos = arr[ptr + 1] as usize;
        arr[pos] = input;
        ptr = ptr + 2;
      }
      4 => {
        let m = res % 10;
        let output = get_value(m, ptr + 1, &arr);
        if output != 0 {
          println!("{}", output);
          break;
        }
        ptr = ptr + 2;
      }
      5 => {
        let m1 = res % 10;
        let m2 = (res / 10) as i32 % 100;
        let v1 = get_value(m1, ptr + 1, &arr);

        ptr = if v1 != 0 {
          get_value(m2, ptr + 2, &arr) as usize
        } else {
          ptr + 3
        };
      }
      6 => {
        let m1 = res % 10;
        let m2 = (res / 10) as i32 % 100;
        let v1 = get_value(m1, ptr + 1, &arr);

        ptr = if v1 == 0 {
          get_value(m2, ptr + 2, &arr) as usize
        } else {
          ptr + 3
        };
      }
      7 => {
        let m1 = res % 10;
        let m2 = (res / 10) as i32 % 100;
        let v1 = get_value(m1, ptr + 1, &arr);
        let v2 = get_value(m2, ptr + 2, &arr);
        let pos = arr[ptr + 3] as usize;
        arr[pos] = if v1 < v2 { 1 } else { 0 };
        ptr = ptr + 4;
      }
      8 => {
        let m1 = res % 10;
        let m2 = (res / 10) as i32 % 100;
        let v1 = get_value(m1, ptr + 1, &arr);
        let v2 = get_value(m2, ptr + 2, &arr);
        let pos = arr[ptr + 3] as usize;
        arr[pos] = if v1 == v2 { 1 } else { 0 };
        ptr = ptr + 4;
      }
      99 => {
        println!("0");
        break;
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
