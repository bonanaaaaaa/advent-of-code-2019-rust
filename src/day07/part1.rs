use crate::reader;

pub fn run() {
  println!("Day 7 Part 1");

  let contents = reader::read("src/day07/input1.txt".to_string());

  let op_code: Vec<i32> = contents
    .split(",")
    .map(|s| s.trim().parse().unwrap())
    .collect();

  let all_possible = generate_all_possible_phase_setting();

  let mut max_output = std::i32::MIN;
  let mut max_sequence = vec![];
  for possible in all_possible {
    let mut input = 0;
    let mut skip = false;
    for n in possible.clone() {
      let computer = IntCodeComputer::new(&op_code, input, n);
      match computer.compute() {
        Ok(output) => {
          input = output;
        }
        Err(_e) => {
          // eprintln!("{}", e);
          skip = true;
          break;
        }
      }
    }
    if skip {
      continue;
    }
    if max_output < input {
      max_output = input.clone();
      max_sequence = possible.clone();
    }
  }
  println!("{} {:?}", max_output, max_sequence);
}

fn generate_all_possible_phase_setting() -> Vec<Vec<i32>> {
  let phase_setting = vec![0, 1, 2, 3, 4];

  let mut ans: Vec<Vec<i32>> = vec![];

  pick(phase_setting, &mut ans);

  return ans.clone();
}

fn pick(data: Vec<i32>, ans: &mut Vec<Vec<i32>>) {
  run_pick(data, vec![], ans);
}

fn run_pick(data: Vec<i32>, result: Vec<i32>, ans: &mut Vec<Vec<i32>>) {
  if data.len() == 0 {
    ans.push(result.clone());
  }

  let a = data.to_vec();
  for i in 0..data.len() {
    let mut aa = a.clone();
    let p = aa.remove(i);
    let mut r = result.clone();
    r.push(p);

    run_pick(aa, r.clone(), ans);
  }
}

struct IntCodeComputer {
  input: i32,
  op_code: Vec<i32>,
  phase_setting: i32,
}

impl IntCodeComputer {
  fn new(op_code: &Vec<i32>, input: i32, phase_setting: i32) -> Self {
    IntCodeComputer {
      op_code: op_code.clone(),
      input: input,
      phase_setting: phase_setting,
    }
  }

  fn compute(&self) -> Result<i32, &str> {
    let mut op_code = self.op_code.clone();
    let mut ptr = 0;
    let mut is_phase_setting_used = false;
    loop {
      let op = op_code[ptr] % 100;
      let res = (op_code[ptr] / 100) as i32;

      match op {
        1 | 2 => {
          let m1 = res % 10;
          let m2 = (res / 10) as i32 % 100;

          let v1 = get_value(m1, ptr + 1, &op_code);
          let v2 = get_value(m2, ptr + 2, &op_code);
          let pos = op_code[ptr + 3] as usize;

          op_code[pos] = if op == 1 { v1 + v2 } else { v1 * v2 };
          ptr = ptr + 4;
        }
        3 => {
          let input = if is_phase_setting_used {
            self.input
          } else {
            is_phase_setting_used = true;
            self.phase_setting
          };
          // println!("input: {}", input);
          let pos = op_code[ptr + 1] as usize;
          op_code[pos] = input;
          ptr = ptr + 2;
        }
        4 => {
          let m = res % 10;
          let output = get_value(m, ptr + 1, &op_code);
          // println!("output: {}", output);
          // if output != 0 {
          return Ok(output);
          // }
          // ptr = ptr + 2;
        }
        5 => {
          let m1 = res % 10;
          let m2 = (res / 10) as i32 % 100;
          let v1 = get_value(m1, ptr + 1, &op_code);

          ptr = if v1 != 0 {
            get_value(m2, ptr + 2, &op_code) as usize
          } else {
            ptr + 3
          };
        }
        6 => {
          let m1 = res % 10;
          let m2 = (res / 10) as i32 % 100;
          let v1 = get_value(m1, ptr + 1, &op_code);

          ptr = if v1 == 0 {
            get_value(m2, ptr + 2, &op_code) as usize
          } else {
            ptr + 3
          };
        }
        7 => {
          let m1 = res % 10;
          let m2 = (res / 10) as i32 % 100;
          let v1 = get_value(m1, ptr + 1, &op_code);
          let v2 = get_value(m2, ptr + 2, &op_code);
          let pos = op_code[ptr + 3] as usize;
          op_code[pos] = if v1 < v2 { 1 } else { 0 };
          ptr = ptr + 4;
        }
        8 => {
          let m1 = res % 10;
          let m2 = (res / 10) as i32 % 100;
          let v1 = get_value(m1, ptr + 1, &op_code);
          let v2 = get_value(m2, ptr + 2, &op_code);
          let pos = op_code[ptr + 3] as usize;
          op_code[pos] = if v1 == v2 { 1 } else { 0 };
          ptr = ptr + 4;
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

fn get_value(mode: i32, v_pos: usize, op_code: &Vec<i32>) -> i32 {
  match mode {
    0 => op_code[op_code[v_pos] as usize],
    1 => op_code[v_pos],
    _ => 0,
  }
}
