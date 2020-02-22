use crate::reader;

use std::collections::HashMap;

pub fn run() {
  println!("Day 6 Part 2");

  let contents = reader::read("src/day06/input2.txt".to_string());

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

  let mut tmp: &str = "";

  for n in node_list {
    if !children_list.contains(&n) {
      tmp = n;
      break;
    }
  }

  let root: &str = tmp.clone();

  let you_path = dfs(&node_map, root, vec![root.to_string()], "YOU").unwrap();
  let santa_path = dfs(&node_map, root, vec![root.to_string()], "SAN").unwrap();

  let mut y_iter = you_path.iter();
  let mut s_iter = santa_path.iter();
  let mut intersect_path: Vec<String> = vec![];

  // println!("{:?}", you_path);
  // println!("{:?}", santa_path);

  let mut count = 0;
  loop {
    let y = y_iter.next().unwrap();
    let s = s_iter.next().unwrap();
    if y == s {
      intersect_path.push(y.to_string());
      count += 1;
    } else {
      break;
    }
  }

  // println!("{:?}", intersect_path);
  // println!("{}", count);

  let ans = you_path.len() - count + santa_path.len() - count - 2;
  println!("{}", ans);
}

fn dfs(
  n: &HashMap<&str, Vec<&str>>,
  current: &str,
  path: Vec<String>,
  target: &str,
) -> Option<Vec<String>> {
  // println!("{} {}", current, target);
  if current == target {
    return Some(path.to_vec());
  }
  let children = n.get(current).unwrap();
  if children.len() == 0 {
    return None;
  }

  let mut ans = vec![];

  for c in children {
    match dfs(n, c, create_new_vec(&path, c.to_string()), target) {
      Some(n) => {
        ans = n;
        break;
      }
      _ => (),
    }
  }

  if ans.is_empty() {
    None
  } else {
    Some(ans)
  }
}

fn create_new_vec(v: &Vec<String>, s: String) -> Vec<String> {
  let mut new_v = v.clone();
  new_v.push(s);
  new_v.clone()
}
