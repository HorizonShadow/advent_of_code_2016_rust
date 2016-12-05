use super::file_handling::get_input;

pub fn part_1() -> i32 {
  let s = get_input("day_1_input");
  let sums = s.split(",")
              .map(|x| x.trim().split_at(1))
              .fold((0,0,0), |(mut x, mut y, mut s), (dir, dist)| {
                s = match dir { "R" => s + 1, _ => s - 1 };
                s = ((s % 4) + 4) % 4;
                let b = format!("{:02b}", s);
                let b = b.split_at(1);
                let dist: i32 = dist.parse().unwrap();
                let dist = match b.1 { "1" => -dist, _ => dist };
                if b.0 == "0" { x += dist } else { y += dist };
                (x, y, s)
              });
  sums.0.abs() + sums.1.abs()
}
