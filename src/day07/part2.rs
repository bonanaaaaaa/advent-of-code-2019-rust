use crate::reader;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub fn run() {
  println!("Day 7 Part 2");

  let contents = reader::read("src/day07/input.txt".to_string());

  let op_code: Vec<i32> = contents
    .split(",")
    .map(|s| s.trim().parse().unwrap())
    .collect();

  let sequences = generate_all_possible_phase_setting();
  let sequences_len = sequences.len();

  let mut max_output = std::i32::MIN;

  let (tx, rx) = channel();

  let mut children = Vec::new();

  for sequence in sequences {
    let op_c = op_code.clone();
    let tx = tx.clone();
    let child = thread::spawn(move || {
      let (tx1, rx1) = channel();
      let (tx2, rx2) = channel();
      let (tx3, rx3) = channel();
      let (tx4, rx4) = channel();
      let (tx5, rx5) = channel();
      let mut channel_list: Vec<(Sender<i32>, Receiver<i32>)> = Vec::new();
      for _ in 0..sequences_len {
        channel_list.push(channel());
      }

      tx5.send(sequence[0]).unwrap();
      tx5.send(0).unwrap();
      tx1.send(sequence[1]).unwrap();
      tx2.send(sequence[2]).unwrap();
      tx3.send(sequence[3]).unwrap();
      tx4.send(sequence[4]).unwrap();

      let mut children = Vec::new();

      let op_c1 = op_c.clone();
      let c1 = thread::spawn(move || {
        let mut computer1 = IntCodeComputer::new(&op_c1, &tx1, &rx5);
        computer1.compute().unwrap();
      });
      let op_c2 = op_c.clone();
      let c2 = thread::spawn(move || {
        let mut computer2 = IntCodeComputer::new(&op_c2, &tx2, &rx1);
        computer2.compute().unwrap();
      });
      let op_c3 = op_c.clone();
      let c3 = thread::spawn(move || {
        let mut computer3 = IntCodeComputer::new(&op_c3, &tx3, &rx2);
        computer3.compute().unwrap();
      });
      let op_c4 = op_c.clone();
      let c4 = thread::spawn(move || {
        let mut computer4 = IntCodeComputer::new(&op_c4, &tx4, &rx3);
        computer4.compute().unwrap();
      });
      let op_c5 = op_c.clone();
      let c5 = thread::spawn(move || {
        let mut computer5 = IntCodeComputer::new(&op_c5, &tx5, &rx4);
        match computer5.compute() {
          Ok(result) => {
            tx.send(result).unwrap();
          }
          Err(e) => {
            eprintln!("Error: {}", e);
          }
        }
      });

      children.push(c1);
      children.push(c2);
      children.push(c3);
      children.push(c4);
      children.push(c5);

      for c in children {
        c.join().unwrap();
      }
    });
    children.push(child);
  }

  for _ in 0..sequences_len {
    let output = rx.recv().unwrap();
    max_output = max_output.max(output);
  }

  for child in children {
    child.join().unwrap();
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

struct IntCodeComputer<'a> {
  op_code: Vec<i32>,
  sender: &'a Sender<i32>,
  receiver: &'a Receiver<i32>,
  ptr: usize,
  current_output: i32,
}

impl IntCodeComputer<'_> {
  fn new<'a>(
    op_code: &Vec<i32>,
    sender: &'a Sender<i32>,
    receiver: &'a Receiver<i32>,
  ) -> IntCodeComputer<'a> {
    IntCodeComputer {
      op_code: op_code.clone(),
      sender: sender,
      receiver: receiver,
      ptr: 0,
      current_output: 0,
    }
  }

  fn compute(&mut self) -> Result<i32, &str> {
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
        3 => match self.receiver.recv() {
          Ok(input) => {
            let pos = self.op_code[self.ptr + 1] as usize;
            self.op_code[pos] = input;
            self.ptr = self.ptr + 2;
          }
          Err(_) => {
            return Err("receiver error");
          }
        },
        4 => {
          let m = res % 10;
          let output = get_value(m, self.ptr + 1, &self.op_code);
          self.ptr = self.ptr + 2;
          self.current_output = output;
          match self.sender.send(output) {
            Ok(_) => continue,
            Err(_) => {
              return Ok(self.current_output);
            }
          }
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
          return Ok(self.current_output);
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
