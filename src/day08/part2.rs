use crate::reader;

const WIDTH: usize = 25;
const TALL: usize = 6;

pub fn run() {
  let contents = reader::read("src/day08/input2.txt".to_string());

  let mut img_data = &contents[..];

  let mut result = vec![];

  for _ in 0..WIDTH * TALL {
    result.push(2);
  }

  loop {
    let (str_layer, rest) = img_data.split_at(WIDTH * TALL);
    img_data = rest;

    for (i, c) in str_layer.chars().enumerate() {
      match c {
        '0' => {
          if result[i] == 2 {
            result[i] = 0;
          }
        }
        '1' => {
          if result[i] == 2 {
            result[i] = 1;
          }
        }
        _ => continue,
      }
    }
    if rest == "" {
      break;
    }
  }
  print_image(result);
}

fn print_image(img: Vec<i32>) {
  for (i, c) in img.iter().enumerate() {
    match c {
      0 => print!("■"),
      1 => print!("□"),
      _ => print!(" "),
    }
    if (i + 1) % WIDTH == 0 {
      println!("");
    }
  }
}
