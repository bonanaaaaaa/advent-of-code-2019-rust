use std::{io, process};

mod reader;

mod day01;
mod day02;
mod day03;

#[derive(Debug)]
enum Part {
  One,
  Two,
}

fn main() {
  let mut i = String::new();
  println!("Please input day:");
  match io::stdin().read_line(&mut i) {
    Ok(_) => {
      let input = i.as_str().trim_end();
      handle_input(input);
    }
    Err(error) => println!("error: {}", error),
  }
}

fn handle_input(input: &str) {
  match input {
    "exit" => process::exit(0),
    "all" => {
      day01::part1::run();
      day01::part2::run();
      day02::part1::run();
      day02::part2::run();
      day03::part1::run();
      day03::part2::run();
    }
    "1" => match handle_part() {
      Ok(Part::One) => day01::part1::run(),
      Ok(Part::Two) => day01::part2::run(),
      Err(err) => eprintln!("{:?}", err),
    },
    "2" => match handle_part() {
      Ok(Part::One) => day02::part1::run(),
      Ok(Part::Two) => day02::part2::run(),
      Err(err) => eprintln!("{:?}", err),
    },
    "3" => match handle_part() {
      Ok(Part::One) => day03::part1::run(),
      Ok(Part::Two) => day03::part2::run(),
      Err(err) => eprintln!("{:?}", err),
    },
    _ => eprintln!("day is invalid"),
  }
}

fn handle_part() -> Result<Part, &'static str> {
  let mut i = String::new();
  println!("Please input part:");
  match io::stdin().read_line(&mut i) {
    Ok(_) => match i.as_str().trim_end() {
      "1" => Ok(Part::One),
      "2" => Ok(Part::Two),
      _ => Err("Part is invalid"),
    },
    Err(_) => Err("error"),
  }
}
