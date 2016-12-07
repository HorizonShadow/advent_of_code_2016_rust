use super::file_handling::get_input;
use std::cmp;
use std::iter::Rev;
use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
struct Point { x: i32, y: i32 }

enum BiRange { Forward(Range<i32>), Backward(Rev<Range<i32>>) }

pub fn part_1() -> i32 {
  let path = calc_path();
  let last = path.last().unwrap();
  calc_man_dist(last, &Point{ x: 0, y: 0})
}

pub fn part_2() -> i32 {
  let path = calc_path();
  let mut visited = Vec::new();
  for coords in path.iter() {
    if visited.contains(coords) {
      visited.push(coords.clone());
      break;
    }
    visited.push(coords.clone());
  }
  let last = visited.last().unwrap();
  calc_man_dist(last, &Point{x: 0, y: 0})
}

fn calc_man_dist(a: &Point, b: &Point) -> i32 {
  (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn cycle(tar: i32, amt: i32, max: i32) -> i32 {
  (((tar + amt) % max) + max) % max
}

fn to_bin_arr(tar: i32) -> (i32, i32) {
  let b = format!("{:02b}", tar);
  let b = b.split_at(1);
  (b.0.parse().unwrap(), b.1.parse().unwrap())
}

fn each<F>(bi_range: BiRange, mut f: F) where F: FnMut(i32) {
  match bi_range {
    BiRange::Forward(r) => for i in r { f(i) },
    BiRange::Backward(r) => for i in r { f(i) }
  }
}

fn parse_input(s: &str) -> Vec<(&str, i32)> {
  s.split(", ")
   .map(|x| x.split_at(1))
   .map(|(dir, dist)| (dir, dist.parse::<i32>().unwrap()))
   .collect::<Vec<(&str, i32)>>()
}

fn calc_path() -> Vec<Point> {
  let s = get_input("day_1_input");
  let insts = parse_input(&s);
  let mut rot = 0;
  let mut path = vec![Point{x: 0, y: 0}];
  for (dir, dist) in insts {
    rot = match dir { "R" => cycle(rot, 1, 4), _ => cycle(rot, -1, 4) };
    let b = to_bin_arr(rot);
    let dist = match b.0 { 1 => -dist, _ => dist };
    let last = path.last().cloned().unwrap();
    if b.1 == 1 {
      let range = get_range(last.x, last.x + dist);
      each(range, |i| path.push(Point{x: i, y: last.y}));
    } else {
      let range = get_range(last.y, last.y + dist);
      each(range, |i| path.push(Point{x: last.x, y: i}));
    }
  }
  path
}

fn get_range(old: i32, new: i32) -> BiRange {
  let max = cmp::max(new, old);
  let min = cmp::min(new, old);
  if old > new {
    BiRange::Backward((min..max).rev())
  } else {
    BiRange::Forward((min+1)..(max+1))
  }
}