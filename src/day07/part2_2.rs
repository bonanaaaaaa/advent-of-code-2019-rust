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

    loop {
      let computer = &mut computers[counter as usize];
      match computer.compute(input) {
        Ok(output) => {
          input = output;
        }
        Err(e) => {
          eprint!("{}", e);
          break;
        }
      }

      counter = if counter + 1 >= computer_len {
        0
      } else {
        counter + 1
      };
    }
    max_output = max_output.max(input);
  }
  println!("{}", max_output);
}

fn generate_all_possible_phase_setting() -> Vec<Vec<i32>> {
  // let phase_setting = vec![5, 6, 7, 8, 9];

  // let mut ans: Vec<Vec<i32>> = vec![];

  // pick(phase_setting, &mut ans);

  // return ans.clone();
  vec![vec![9, 8, 7, 6, 5]]
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

#[derive(Debug)]
struct IntCodeComputer {
  op_code: Vec<i32>,
  phase_setting: i32,
}

impl IntCodeComputer {
  fn new(op_code: &Vec<i32>, phase_setting: i32) -> Self {
    IntCodeComputer {
      op_code: op_code.clone(),
      phase_setting: phase_setting,
    }
  }

  fn compute(&mut self, input_signal: i32) -> Result<i32, &'static str> {
    let mut ptr = 0;
    let mut is_phase_setting_used = false;
    let mut counter = 0;
    while counter < 200 {
      let op = self.op_code[ptr] % 100;
      let res = (self.op_code[ptr] / 100) as i32;

      match op {
        1 | 2 => {
          let m1 = res % 10;
          let m2 = (res / 10) as i32 % 100;
          // let result: i32;
          let v1 = get_value(m1, ptr + 1, &self.op_code)?;
          let v2 = get_value(m2, ptr + 2, &self.op_code)?;
          // println!("{}, {}", v1, v2);
          let result = if op == 1 { v1 + v2 } else { v1 * v2 };

          let pos = self.op_code[ptr + 3] as usize;

          self.op_code[pos] = result;
          ptr = ptr + 4;
        }
        3 => {
          let input = if is_phase_setting_used {
            input_signal
          } else {
            is_phase_setting_used = true;
            self.phase_setting
          };
          // println!("input: {}", input);
          let pos = self.op_code[ptr + 1] as usize;
          self.op_code[pos] = input;
          ptr = ptr + 2;
        }
        4 => {
          let m = res % 10;
          let output = get_value(m, ptr + 1, &self.op_code)?;
          // println!("output: {}", output);
          // if output != 0 {
          return Ok(output);
          // }
          // ptr = ptr + 2;
        }
        5 => {
          let m1 = res % 10;
          let m2 = (res / 10) as i32 % 100;
          let v1 = get_value(m1, ptr + 1, &self.op_code)?;

          ptr = if v1 != 0 {
            get_value(m2, ptr + 2, &self.op_code)? as usize
          } else {
            ptr + 3
          };
        }
        6 => {
          let m1 = res % 10;
          let m2 = (res / 10) as i32 % 100;
          let v1 = get_value(m1, ptr + 1, &self.op_code)?;

          ptr = if v1 == 0 {
            get_value(m2, ptr + 2, &self.op_code)? as usize
          } else {
            ptr + 3
          };
        }
        7 => {
          let m1 = res % 10;
          let m2 = (res / 10) as i32 % 100;
          let v1 = get_value(m1, ptr + 1, &self.op_code)?;
          let v2 = get_value(m2, ptr + 2, &self.op_code)?;
          let pos = self.op_code[ptr + 3] as usize;
          self.op_code[pos] = if v1 < v2 { 1 } else { 0 };
          ptr = ptr + 4;
        }
        8 => {
          let m1 = res % 10;
          let m2 = (res / 10) as i32 % 100;
          let v1 = get_value(m1, ptr + 1, &self.op_code)?;
          let v2 = get_value(m2, ptr + 2, &self.op_code)?;
          let pos = self.op_code[ptr + 3] as usize;
          self.op_code[pos] = if v1 == v2 { 1 } else { 0 };
          ptr = ptr + 4;
        }
        99 => {
          return Err("Something when wrong with (99)");
        }
        _ => {
          return Err("Something when wrong");
        }
      }
      counter += 1;
    }
    Err("Too many loop")
  }
}

fn get_value(mode: i32, v_pos: usize, op_code: &Vec<i32>) -> Result<i32, &'static str> {
  // let op = op_code.clone().to_owned();
  match mode {
    0 => match op_code.get(v_pos) {
      Some(pos) => match op_code.get(*pos as usize) {
        Some(v) => Ok(*v),
        None => Err("Out of range"),
      },
      None => Err("Out of range"),
    },
    1 => match op_code.get(v_pos) {
      Some(v) => Ok(*v),
      None => Err("Out of range"),
    },
    _ => Err("Invalid mode"),
  }
}

// fn get_value(mode: i32, v_pos: usize, op_code: &Vec<i32>) -> Result<i32, &str> {
//   let l = op_code.len();
//   match mode {
//     0 => {
//       if 0 <= v_pos && v_pos < l && 0 <= op_code[v_pos] && op_code[v_pos] < l as i32 {
//         Ok(op_code[op_code[v_pos] as usize])
//       } else {
//         Err("Out of range")
//       }
//     }
//     1 => {
//       if 0 <= v_pos && v_pos < l {
//         Ok(op_code[v_pos])
//       } else {
//         Err("Out of range")
//       }
//     }
//     _ => Err("Invalid mode"),
//   }
// }
