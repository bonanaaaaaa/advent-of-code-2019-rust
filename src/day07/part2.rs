use crate::reader;

pub fn run() {
  println!("Day 7 Part 2");

  let contents = reader::read("src/day07/input2.txt".to_string());

  let op_code: Vec<i32> = contents
    .split(",")
    .map(|s| s.trim().parse().unwrap())
    .collect();

  let all_possible = generate_all_possible_phase_setting();

  let mut max_output = std::i32::MIN;

  for sequence in all_possible {
    let mut input = 0;
    let mut computers: Vec<IntCodeComputer> = sequence
      .iter()
      .map(|&n| IntCodeComputer::new(&op_code, n))
      .collect();

    let mut counter = 0;
    let computer_len = computers.len();
    let mut last_value = 0;

    loop {
      let computer = &mut computers[counter as usize];
      match computer.compute(input) {
        Ok(output) => {
          input = output;
        }
        Err(_) => {
          break;
        }
      }

      counter = if counter + 1 >= computer_len {
        last_value = input;
        0
      } else {
        counter + 1
      };
    }
    max_output = max_output.max(last_value);
  }
  println!("{}", max_output);
}

fn generate_all_possible_phase_setting() -> Vec<Vec<i32>> {
  let phase_setting = vec![5, 6, 7, 8, 9];

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
  op_code: Vec<i32>,
  phase_setting: i32,
  ptr: usize,
  is_phase_setting_used: bool,
}

impl IntCodeComputer {
  fn new(op_code: &Vec<i32>, phase_setting: i32) -> Self {
    IntCodeComputer {
      op_code: op_code.clone(),
      phase_setting: phase_setting,
      ptr: 0,
      is_phase_setting_used: false,
    }
  }

  fn compute(&mut self, input_signal: i32) -> Result<i32, &str> {
    loop {
      let op = self.op_code[self.ptr] % 100;
      let res = (self.op_code[self.ptr] / 100) as i32;

      match op {
        1 | 2 => {
          let m1 = res % 10;
          let m2 = (res / 10) as i32 % 100;

          let v1 = get_value(m1, self.ptr + 1, &self.op_code);
          let v2 = get_value(m2, self.ptr + 2, &self.op_code);
          let pos = self.op_code[self.ptr + 3] as usize;

          self.op_code[pos] = if op == 1 { v1 + v2 } else { v1 * v2 };
          self.ptr = self.ptr + 4;
        }
        3 => {
          let input = if self.is_phase_setting_used {
            input_signal
          } else {
            self.is_phase_setting_used = true;
            self.phase_setting
          };
          let pos = self.op_code[self.ptr + 1] as usize;
          self.op_code[pos] = input;
          self.ptr = self.ptr + 2;
        }
        4 => {
          let m = res % 10;
          let output = get_value(m, self.ptr + 1, &self.op_code);
          self.ptr = self.ptr + 2;
          return Ok(output);
        }
        5 => {
          let m1 = res % 10;
          let m2 = (res / 10) as i32 % 100;
          let v1 = get_value(m1, self.ptr + 1, &self.op_code);

          self.ptr = if v1 != 0 {
            get_value(m2, self.ptr + 2, &self.op_code) as usize
          } else {
            self.ptr + 3
          };
        }
        6 => {
          let m1 = res % 10;
          let m2 = (res / 10) as i32 % 100;
          let v1 = get_value(m1, self.ptr + 1, &self.op_code);

          self.ptr = if v1 == 0 {
            get_value(m2, self.ptr + 2, &self.op_code) as usize
          } else {
            self.ptr + 3
          };
        }
        7 => {
          let m1 = res % 10;
          let m2 = (res / 10) as i32 % 100;
          let v1 = get_value(m1, self.ptr + 1, &self.op_code);
          let v2 = get_value(m2, self.ptr + 2, &self.op_code);
          let pos = self.op_code[self.ptr + 3] as usize;
          self.op_code[pos] = if v1 < v2 { 1 } else { 0 };
          self.ptr = self.ptr + 4;
        }
        8 => {
          let m1 = res % 10;
          let m2 = (res / 10) as i32 % 100;
          let v1 = get_value(m1, self.ptr + 1, &self.op_code);
          let v2 = get_value(m2, self.ptr + 2, &self.op_code);
          let pos = self.op_code[self.ptr + 3] as usize;
          self.op_code[pos] = if v1 == v2 { 1 } else { 0 };
          self.ptr = self.ptr + 4;
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
