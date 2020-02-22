use crate::reader;

use std::collections::HashMap;

pub fn run() {
  println!("Day 6 Part 1");

  let contents = reader::read("src/day06/input1.txt".to_string());

  let arr: Vec<&str> = contents.split("\n").collect();

  let mut node_map: HashMap<&str, Vec<&str>> = HashMap::new();
  let mut node_list: Vec<&str> = Vec::new();
  let mut children_list: Vec<&str> = Vec::new();

  for line in arr {
    let d: Vec<&str> = line.split(")").collect();
    match node_map.get_mut(d[0]) {
      Some(n) => n.push(d[1]),
      None => {
        node_map.insert(d[0], vec![d[1]]);
        ()
      }
    }
    node_list.push(d[0]);
    children_list.push(d[1]);
  }

  for n in &children_list {
    if !node_map.contains_key(n) {
      node_map.insert(n, vec![]);
    }
  }

  let mut q: Vec<&str> = Vec::new();

  for n in node_list {
    if !children_list.contains(&n) {
      q.push(n);
    }
  }

  let mut node_in_level = q.len();
  let mut level = 0i32;
  let mut sum = 0i32;

  while !q.is_empty() {
    // println!("{:?} {}", q, sum);

    sum += level;
    let node_name = q.remove(0);
    node_in_level -= 1;

    // println!("{} {} {}", node_name, node_in_level, level);

    let n_children = node_map.get_mut(node_name).unwrap();
    for n_child in n_children {
      q.push(n_child);
    }

    if node_in_level == 0 {
      level += 1;
      node_in_level = q.len();
    }
  }

  println!("{:?}", sum);
}
