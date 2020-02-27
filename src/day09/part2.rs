use crate::reader;

use std::collections::HashMap;

pub fn run() {
  println!("Day 9 Part 2");

  let contents = reader::read("src/day09/input.txt".to_string());

  let op_code: Vec<i64> = contents
    .split(",")
    .map(|s| s.trim().parse().unwrap())
    .collect();

  let mut op_code_map: HashMap<i64, i64> = HashMap::new();
  for (i, n) in op_code.iter().enumerate() {
    op_code_map.insert(i as i64, *n);
  }

  let mut computer = IntCodeComputer::new(&op_code_map);
  match computer.compute() {
    Ok(output) => {
      println!("{}", output);
    }
    Err(e) => {
      eprintln!("{}", e);
    }
  }
}

struct IntCodeComputer {
  op_code: HashMap<i64, i64>,
  ptr: i64,
  relative_ptr: i64,
}

impl IntCodeComputer {
  fn new(op_code: &HashMap<i64, i64>) -> Self {
    IntCodeComputer {
      op_code: op_code.clone(),
      ptr: 0,
      relative_ptr: 0,
    }
  }

  fn compute(&mut self) -> Result<i64, &str> {
    let mut op_code = self.op_code.clone();

    loop {
      let op = op_code.get(&self.ptr).unwrap() % 100;
      let res = (op_code.get(&self.ptr).unwrap() / 100) as i64;

      match op {
        1 | 2 => {
          let m1 = res % 10;
          let m2 = (res / 10) as i64 % 10;
          let m3 = res / 100 as i64;

          let v1 = get_value(m1, self.ptr + 1, &op_code, self.relative_ptr);
          let v2 = get_value(m2, self.ptr + 2, &op_code, self.relative_ptr);
          let pos = get_pos(m3, self.ptr + 3, &op_code, self.relative_ptr);

          op_code.insert(pos, if op == 1 { v1 + v2 } else { v1 * v2 });
          self.ptr = self.ptr + 4;
        }
        3 => {
          let m = res % 10;
          let pos = get_pos(m, self.ptr + 1, &op_code, self.relative_ptr);

          op_code.insert(pos, 2);
          self.ptr = self.ptr + 2;
        }
        4 => {
          let m = res % 10;
          let output = get_value(m, self.ptr + 1, &op_code, self.relative_ptr);
          print!("{} ", output);
          self.ptr = self.ptr + 2;
        }
        5 => {
          let m1 = res % 10;
          let m2 = (res / 10) as i64 % 10;
          let v1 = get_value(m1, self.ptr + 1, &op_code, self.relative_ptr);

          self.ptr = if v1 != 0 {
            get_value(m2, self.ptr + 2, &op_code, self.relative_ptr)
          } else {
            self.ptr + 3
          };
        }
        6 => {
          let m1 = res % 10;
          let m2 = (res / 10) as i64 % 10;
          let v1 = get_value(m1, self.ptr + 1, &op_code, self.relative_ptr);

          self.ptr = if v1 == 0 {
            get_value(m2, self.ptr + 2, &op_code, self.relative_ptr)
          } else {
            self.ptr + 3
          };
        }
        7 => {
          let m1 = res % 10;
          let m2 = (res / 10) as i64 % 10;
          let m3 = res / 100 as i64;
          let v1 = get_value(m1, self.ptr + 1, &op_code, self.relative_ptr);
          let v2 = get_value(m2, self.ptr + 2, &op_code, self.relative_ptr);
          let pos = get_pos(m3, self.ptr + 3, &op_code, self.relative_ptr);
          op_code.insert(pos, if v1 < v2 { 1 } else { 0 });
          self.ptr = self.ptr + 4;
        }
        8 => {
          let m1 = res % 10;
          let m2 = (res / 10) as i64 % 10;
          let m3 = res / 100 as i64;
          let v1 = get_value(m1, self.ptr + 1, &op_code, self.relative_ptr);
          let v2 = get_value(m2, self.ptr + 2, &op_code, self.relative_ptr);
          let pos = get_pos(m3, self.ptr + 3, &op_code, self.relative_ptr);
          op_code.insert(pos, if v1 == v2 { 1 } else { 0 });
          self.ptr = self.ptr + 4;
        }
        9 => {
          let m = res % 10;
          let v = get_value(m, self.ptr + 1, &op_code, self.relative_ptr);
          self.relative_ptr += v;
          self.ptr = self.ptr + 2;
        }
        99 => {
          return Err("Something when wrong with (99)");
        }
        _ => {
          return Err("Something when wrong");
        }
      }
    }
  }
}

fn get_value(mode: i64, v_pos: i64, op_code: &HashMap<i64, i64>, relative_pos: i64) -> i64 {
  match mode {
    0 => match op_code.get(&v_pos) {
      Some(t) => match op_code.get(t) {
        Some(v) => *v,
        None => 0,
      },
      None => 0,
    },
    1 => *op_code.get(&v_pos).unwrap(),
    2 => match op_code.get(&v_pos) {
      Some(t) => match op_code.get(&(*t + relative_pos)) {
        Some(v) => *v,
        None => 0,
      },
      None => 0,
    },
    _ => 0i64,
  }
}

fn get_pos(mode: i64, v_pos: i64, op_code: &HashMap<i64, i64>, relative_pos: i64) -> i64 {
  match mode {
    0 => match op_code.get(&v_pos) {
      Some(v) => *v,
      None => 0,
    },
    2 => match op_code.get(&v_pos) {
      Some(v) => *v + relative_pos,
      None => 0,
    },
    _ => 0i64,
  }
}
