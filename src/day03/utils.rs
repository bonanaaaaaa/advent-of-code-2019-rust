pub fn parse(path: &str) -> Vec<Line> {
  let mut origin = Point { x: 0, y: 0 };
  let mut line: Vec<Line> = Vec::new();

  path.split(",").for_each(|path| {
    let l = decode(path, &mut origin);
    line.push(l);
  });

  line
}