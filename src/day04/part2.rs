#[derive(Copy, Clone, Debug)]
enum State {
  NoPair,
  Pair2,
  PairMoreThan2,
  Chill,
  Accept,
  Reject,
}

const START: i32 = 271973;
const END: i32 = 785961;

pub fn run() {
  println!("Day 4 Part 2");

  let mut count = 0;

  for i in 1..10 {
    count += compute(State::NoPair, i, 4, i * 10i32.pow(5));
  }

  println!("{}", count);
}

fn compute(state: State, number: i32, digit: i32, value: i32) -> i32 {
  match state {
    State::NoPair => {
      let mut count = 0;
      count += compute(
        if digit - 1 < 0 {
          State::Accept
        } else {
          State::Pair2
        },
        number,
        digit - 1,
        value + (number * 10i32.pow(digit as u32)),
      );
      for i in (number + 1)..10 {
        count += compute(
          if digit - 1 < 0 {
            State::Reject
          } else {
            State::NoPair
          },
          i,
          digit - 1,
          value + (i * 10i32.pow(digit as u32)),
        );
      }
      count
    }
    State::Pair2 => {
      let mut count = 0;
      count += compute(
        if digit - 1 < 0 {
          State::Reject
        } else {
          State::PairMoreThan2
        },
        number,
        digit - 1,
        value + (number * 10i32.pow(digit as u32)),
      );
      for i in number + 1..10 {
        count += compute(
          if digit - 1 < 0 {
            State::Accept
          } else {
            State::Chill
          },
          i,
          digit - 1,
          value + (i * 10i32.pow(digit as u32)),
        );
      }
      count
    }
    State::PairMoreThan2 => {
      let mut count = 0;
      count += compute(
        if digit - 1 < 0 {
          State::Reject
        } else {
          State::PairMoreThan2
        },
        number,
        digit - 1,
        value + (number * 10i32.pow(digit as u32)),
      );
      for i in number + 1..10 {
        count += compute(
          if digit - 1 < 0 {
            State::Reject
          } else {
            State::NoPair
          },
          i,
          digit - 1,
          value + (i * 10i32.pow(digit as u32)),
        );
      }
      count
    }
    State::Chill => {
      let mut count = 0;
      for i in number..10 {
        count += compute(
          if digit - 1 < 0 {
            State::Accept
          } else {
            State::Chill
          },
          i,
          digit - 1,
          value + (i * 10i32.pow(digit as u32)),
        );
      }
      count
    }
    State::Accept => {
      if START <= value && value <= END {
        1
      } else {
        0
      }
    }
    State::Reject => 0,
  }
}
