use std::env;
use std::fs;
use std::path::PathBuf;

pub fn read(path_name: String) -> String {
  let relative_path = PathBuf::from(path_name);
  let mut absolute_path = env::current_dir().unwrap();
  absolute_path.push(relative_path);
  let abs_path_str = absolute_path.to_str().unwrap();
  let contents = fs::read_to_string(abs_path_str).expect("Something went wrong reading the file");

  contents
}
