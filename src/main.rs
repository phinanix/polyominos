#![allow(unused)] // tests dont seem to count
use itertools::Itertools;
use smallvec::{smallvec,SmallVec};

const GRID_SIZE : usize = 8;

#[derive(PartialEq, Eq, Clone, Copy)]
struct Point{
  x: i8, y: u8
}

#[derive(Default, Clone, PartialEq, Eq, Hash)]
struct Grid{
  grid: [[bool; GRID_SIZE]; 2*GRID_SIZE+1]
}

impl Grid {
  fn get_pos(self, p : Point) -> bool {
    let x_raw = (p.x as usize) + GRID_SIZE;
    return self.grid[x_raw][p.y as usize]
  }

  fn set_pos(mut self, p : Point, new_val : bool) {
    let x_raw = (p.x as usize) + GRID_SIZE;
    self.grid[x_raw][p.y as usize] = new_val
  }

  fn get_neighbors(p : Point) -> SmallVec<[Point; 4]> {
    let mut out = smallvec![];
    if (p.y as usize) < GRID_SIZE {
      out.push(Point{x: p.x, y: p.y+1});
    }
    if p.y > 0 && p.x > 0 {
      out.push(Point{x : p.x, y: p.y-1});
    }
    if (p.x as usize) < GRID_SIZE {
      out.push(Point{x: p.x+1, y:p.y});
    }
    if p.x > -1 * (GRID_SIZE as i8) {
      out.push(Point{x: p.x-1, y:p.y})
    }
    return out
  }
}

fn add_tile(mut grid: Grid, p : Point, mut reachable : SmallVec<[Point; 8]>) {
  grid.set_pos(p, true); 
  let possible_neighbors = Grid::get_neighbors(p);
  for neighbor in possible_neighbors {
    if !reachable.contains(&neighbor) {
      reachable.push(neighbor);
    }
  }
}

fn enumerate_polyominos(size : u8) -> Vec<Grid> {
  
  let out = vec![];
  let mut enum_grid = Grid::default(); 
  let mut reachable_set : SmallVec<[Point; 8]> = smallvec![Point{x: 0, y:0}];
  enum_grid.set_pos(Point{x:0, y:0}, true);
  return out 
}
fn main() {
    println!("Hello, world!");
}
