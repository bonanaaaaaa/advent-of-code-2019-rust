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

  let mut you_path = vec![];
  let mut santa_path = vec![];
  dfs(&node_map, root, &mut you_path, "YOU");
  dfs(&node_map, root, &mut santa_path, "SAN");

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

  let ans = you_path.len() + santa_path.len() - (2 * count);
  println!("{}", ans);
}

fn dfs<'a>(
  n: &'a HashMap<&str, Vec<&str>>,
  current: &'a str,
  ans: &mut Vec<&'a str>,
  target: &str,
) {
  run_dfs(n, current, &mut vec![current], ans, target);
}

fn run_dfs<'a>(
  n: &'a HashMap<&str, Vec<&str>>,
  current: &'a str,
  path: &mut Vec<&'a str>,
  ans: &mut Vec<&'a str>,
  target: &str,
) {
  // println!("{} {:?}", current, path);
  let children = n.get(current).unwrap();
  if children.len() == 0 {
    return;
  }

  for c in children {
    if *c == target {
      for p in path {
        ans.push(p);
      }
      return;
    }
    path.push(c);
    run_dfs(n, c, path, ans, target);
    path.pop();
  }
}
