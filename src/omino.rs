#![allow(unused)]

use std::{cmp::Ordering, collections::HashSet, fmt::Write, fs::File};

use itertools::Itertools;
use smallvec::{smallvec, SmallVec};
use std::fmt::Debug;

use pprof::Report;

const GRID_SIZE: usize = 17;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Point {
  x: i8,
  y: u8,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum TileState {
  Border,
  Occupied,
  Reachable,
  Free,
}
use TileState::*;

impl TileState {
  pub fn to_char(&self) -> char {
    match self {
      Border => 'B',
      Occupied => '#',
      Reachable => 'r',
      Free => '.',
    }
  }
}
impl Debug for TileState {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char(self.to_char())
  }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid {
  grid: [[TileState; GRID_SIZE]; 2 * GRID_SIZE - 1],
}

impl Default for Grid {
  fn default() -> Self {
    let basic_row: [TileState; GRID_SIZE] = [Free; GRID_SIZE];
    let mut grid = [basic_row; 2 * GRID_SIZE - 1];
    for x in (0..GRID_SIZE - 1) {
      grid[x][0] = TileState::Border;
    }

    Self { grid }
  }
}

impl Debug for Grid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    struct Row([TileState; GRID_SIZE]);
    impl Debug for Row {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ts in self.0 {
          f.write_char(ts.to_char());
        }
        Ok(())
      }
    }
    f.debug_struct("Grid").field("grid", &self.grid.map(|x| Row(x))).finish()
  }
}

impl Grid {
  pub fn get_pos(&self, p: Point) -> TileState {
    let x_raw = p.x + ((GRID_SIZE - 1) as i8);
    return self.grid[x_raw as usize][p.y as usize];
  }

  pub fn set_pos(&mut self, p: Point, new_val: TileState) {
    let x_raw = p.x + ((GRID_SIZE - 1) as i8);
    self.grid[x_raw as usize][p.y as usize] = new_val
  }

  pub fn get_neighbors(p: Point) -> SmallVec<[Point; 4]> {
    let mut out = smallvec![];
    if (p.y as usize) < GRID_SIZE - 1 {
      out.push(Point { x: p.x, y: p.y + 1 });
    }
    if p.y > 0 {
      out.push(Point { x: p.x, y: p.y - 1 });
    }
    if p.x < (GRID_SIZE - 1) as i8 {
      out.push(Point { x: p.x + 1, y: p.y });
    }
    if p.x > -1 * ((GRID_SIZE - 1) as i8) {
      out.push(Point { x: p.x - 1, y: p.y })
    }
    return out;
  }
}

type PointList = SmallVec<[Point; 16]>;

fn enumerate_recursion(
  out: &mut Vec<PointList>,
  grid: &mut Grid,
  mut untried_set: PointList,
  occupied_set: &mut PointList,
  mut cur_omino_size: u8,
  size: u8,
) {
  assert!(cur_omino_size <= size);
  while let Some(next_tile) = untried_set.pop() {
    grid.set_pos(next_tile, Occupied);
    //dbg!("after set:", &grid);
    occupied_set.push(next_tile);
    cur_omino_size += 1;

    if cur_omino_size == size {
      //we have produced an omino of the desired size
      out.push(occupied_set.clone());
      if out.len() % 1000000 == 0 {
        println!("out.len {}", out.len());
      }
    } else {
      //we aren't done with this omino yet, so we need to update reachability and so on
      let mut new_reachable_set = untried_set.clone();

      let free_neighbors: SmallVec<[Point; 4]> = Grid::get_neighbors(next_tile)
        .into_iter()
        .filter(|&neighbor| grid.get_pos(neighbor) == Free)
        .collect();
      for &neighbor in free_neighbors.iter() {
        new_reachable_set.push(neighbor);
        grid.set_pos(neighbor, Reachable);
      }
      enumerate_recursion(out, grid, new_reachable_set, occupied_set, cur_omino_size, size);
      for neighbor in free_neighbors {
        grid.set_pos(neighbor, Free);
      }
    }
    occupied_set.pop();
    cur_omino_size -= 1;
    //but note though it is reachable it is no longer in "reachable set" because that's actually
    // "reachable todo" or something
    grid.set_pos(next_tile, Reachable);
  }
}

pub fn enumerate_polyominos(size: u8) -> Vec<PointList> {
  let mut out = vec![];
  let mut enum_grid = Grid::default();
  let mut reachable_set: PointList = smallvec![Point { x: 0, y: 0 }];
  enum_grid.set_pos(Point { x: 0, y: 0 }, TileState::Reachable);
  let mut occupied_set: PointList = smallvec![];
  let mut cur_omino_size: u8 = 0;

  enumerate_recursion(
    &mut out,
    &mut enum_grid,
    reachable_set,
    &mut occupied_set,
    cur_omino_size,
    size,
  );

  return out;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Dir {
  N,
  E,
  S,
  W,
}
use Dir::*;

impl Dir {
  pub fn flip(self: Dir) -> Dir {
    match self {
      N => S,
      S => N,
      E => W,
      W => E,
    }
  }
}
pub fn dir_to_offset(d: Dir) -> FreePoint {
  match d {
    N => FreePoint { x: 0, y: 1 },
    E => FreePoint { x: 1, y: 0 },
    S => FreePoint { x: 0, y: -1 },
    W => FreePoint { x: -1, y: 0 },
  }
}

pub fn offset_in_dir(fp: FreePoint, d: Dir) -> FreePoint {
  sum_points(fp, dir_to_offset(d))
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct FreePoint {
  pub x: i8,
  pub y: i8,
}

pub type FreePointList = SmallVec<[FreePoint; 16]>;

impl From<Point> for FreePoint {
  fn from(Point { x, y }: Point) -> Self {
    FreePoint { x, y: y.try_into().unwrap() }
  }
}

impl From<FreePoint> for Point {
  fn from(FreePoint { x, y }: FreePoint) -> Self {
    Point { x, y: y.try_into().unwrap() }
  }
}

impl FreePoint {
  pub fn get_neighbors(&self) -> FreePointList {
    let mut out = smallvec![];
    match self {
      FreePoint { x, y } => {
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
          out.push(FreePoint { x: x + dx, y: y + dy });
        }
      }
    }
    out
  }

  pub fn get_neighbors_with_directions(self) -> [(FreePoint, Dir); 4] {
    [N, E, S, W].map(|d| (offset_in_dir(self, d), d))
  }

  pub fn get_all_neighbors(pts: &FreePointList) -> FreePointList {
    let mut neighbor_pts = HashSet::new();
    for pt in pts {
      neighbor_pts.extend(pt.get_neighbors());
    }
    neighbor_pts.into_iter().collect()
  }
}

pub fn sum_points(
  FreePoint { x: px, y: py }: FreePoint,
  FreePoint { x: qx, y: qy }: FreePoint,
) -> FreePoint {
  FreePoint { x: px + qx, y: py + qy }
}

pub fn compare_points(
  FreePoint { x: px, y: py }: &FreePoint,
  FreePoint { x: qx, y: qy }: &FreePoint,
) -> Ordering {
  match py.cmp(qy) {
    Ordering::Less => Ordering::Less,
    Ordering::Greater => Ordering::Greater,
    Ordering::Equal => px.cmp(qx),
  }
}

pub fn translate_omino(omino: &FreePointList, translation: FreePoint) -> FreePointList {
  omino.iter().map(|&pt| sum_points(pt, translation)).collect()
}

pub fn invert_point(FreePoint { x, y }: FreePoint) -> FreePoint {
  FreePoint { x: -1 * x, y: -1 * y }
}

pub fn normalize_omino(omino: FreePointList) -> FreePointList {
  //takes an omino and translates it so it matches the Redelmeir normalization (y >= 0, y=0 => x>= 0)
  //dbg!(&omino);
  let min_point = *omino.iter().min_by(|p, q| compare_points(p, q)).unwrap();
  let translation = invert_point(min_point);
  //dbg!(&translation);
  let mut translated_omino = translate_omino(&omino, translation);
  translated_omino.sort();
  //dbg!(&translated_omino);
  translated_omino
}

pub fn slow_omino_enum(size: u8) -> Vec<FreePointList> {
  if size == 1 {
    return vec![smallvec![FreePoint { x: 0, y: 0 }]];
  }
  let smaller_ominos = slow_omino_enum(size - 1);
  let mut out = HashSet::new();
  for smaller_omino in smaller_ominos {
    let neighbors = FreePoint::get_all_neighbors(&smaller_omino)
      .into_iter()
      .filter(|pt| !smaller_omino.contains(pt));
    for neighbor in neighbors {
      let mut new_omino = smaller_omino.clone();
      new_omino.push(neighbor);
      out.insert(normalize_omino(new_omino));
    }
  }
  out.into_iter().collect()
}

pub mod test {
  use super::*;

  #[test]
  fn neighbors_correct() {
    let mut ans: PointList = smallvec![
      Point { x: -2, y: 3 },
      Point { x: 0, y: 3 },
      Point { x: -1, y: 4 },
      Point { x: -1, y: 2 }
    ];
    ans.sort();
    let mut neighbors = Grid::get_neighbors(Point { x: -1, y: 3 });
    neighbors.sort();
    assert_eq!(neighbors, ans)
  }
}
