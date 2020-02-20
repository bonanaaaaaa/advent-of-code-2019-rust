use crate::reader;

pub fn run() {
  println!("Day 2 Part 2");

  let contents = reader::read("src/day02/input2.txt".to_string());

  let arr: Vec<i32> = contents
    .split(",")
    .map(|s| s.trim().parse().unwrap())
    .collect();

  let needed = 19690720;

  let ptr_arr = &arr;

  let mut input_arr: Vec<(i32, i32)> = Vec::new();

  for i in 0..100 {
    for j in 0..100 {
      input_arr.push((i, j))
    }
  }

  for (i, j) in input_arr.iter() {
    let mut copy_arr = ptr_arr.clone();
    copy_arr[1] = i.clone();
    copy_arr[2] = j.clone();
    let ans = find(&copy_arr);
    if ans[0] == needed {
      println!("{} {}", ans[1], ans[2]);
      println!("{}", 100 * ans[1] + ans[2]);
      break;
    }
  }
}

fn find(a: &Vec<i32>) -> Vec<i32> {
  let mut arr = a.clone();
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
  arr
}
