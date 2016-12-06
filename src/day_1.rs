use super::file_handling::get_input;

pub fn part_1() -> i32 {
  let path = calc_path();
  let last = *path.last().unwrap();
  calc_man_dist(last, (0,0))
}

fn calc_man_dist(a: (i32,i32), b: (i32,i32)) -> i32 {
  (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn calc_path() -> Vec<(i32, i32)> {
  let s = get_input("day_1_input");
  let insts = s.split(", ")
              .map(|x| x.split_at(1));
  let mut rot = 0;
  let mut path = vec![(0,0)];
  for (dir, dist) in insts {
    rot = match dir { "R" => rot + 1, _ => rot - 1 };
    rot = ((rot % 4) + 4) % 4;
    let b = format!("{:02b}", rot);
    let b = b.split_at(1);
    let dist: i32 = dist.parse().unwrap();
    let dist = match b.1 { "1" => -dist, _ => dist };
    let (mut x, mut y) = *path.last().unwrap();
    if b.0 == "0" { x += dist } else { y += dist };
    path.push((x, y));
  };
  path
}
