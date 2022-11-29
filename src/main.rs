#![allow(unused)] // tests dont seem to count
use itertools::Itertools;
use smallvec::{smallvec,SmallVec};

const GRID_SIZE : usize = 8;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Point{
  x: i8, y: u8
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum TileState {
  Border,
  Occupied,
  Reachable,
  Free
}
use TileState::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Grid{
  grid: [[TileState; GRID_SIZE]; 2*GRID_SIZE-1]
}

impl Default for Grid {
    fn default() -> Self {
        let basic_row : [TileState; GRID_SIZE] = [Free; GRID_SIZE];
        let mut grid = [basic_row; 2*GRID_SIZE-1];
        for x in (0..GRID_SIZE-1) {
          grid[x][0] = TileState::Border;
        }
        
        Self { grid }
    }
}
impl Grid {
  fn get_pos(&self, p : Point) -> TileState {
    let x_raw = p.x + ((GRID_SIZE-1) as i8);
    return self.grid[x_raw as usize][p.y as usize]
  }

  fn set_pos(&mut self, p : Point, new_val : TileState) {
    let x_raw = p.x + ((GRID_SIZE-1) as i8);
    self.grid[x_raw as usize][p.y as usize] = new_val
  }

  fn get_neighbors(p : Point) -> SmallVec<[Point; 4]> {
    let mut out = smallvec![];
    if (p.y as usize) < GRID_SIZE-1 {
      out.push(Point{x: p.x, y: p.y+1});
    }
    if p.y > 0 {
      out.push(Point{x : p.x, y: p.y-1});
    }
    if (p.x as usize) < GRID_SIZE-1 {
      out.push(Point{x: p.x+1, y:p.y});
    }
    if p.x > -1 * ((GRID_SIZE-1) as i8) {
      out.push(Point{x: p.x-1, y:p.y})
    }
    return out
  }
}

fn add_tile(mut grid: Grid, p : Point, mut reachable : SmallVec<[Point; 8]>) {
  //grid.set_pos(p, true); 
  let possible_neighbors = Grid::get_neighbors(p);
  for neighbor in possible_neighbors {
    if !reachable.contains(&neighbor) {
      reachable.push(neighbor);
    }
  }
}

type PointList = SmallVec<[Point; 8]>;

fn enumerate_recursion(out: &mut Vec<PointList>, grid: &mut Grid, 
  mut reachable_set : PointList, occupied_set : &mut PointList, 
  mut cur_omino_size : u8, size : u8) 
{
  assert!(cur_omino_size <= size);
  
  while reachable_set.len() > 0 {
    //dbg!(&grid);
    //dbg!(&reachable_set, cur_omino_size);
    let next_tile = reachable_set.pop().unwrap(); //todo integrate with loop cond
    //dbg!(&next_tile);
    grid.set_pos(next_tile, Occupied);
    //dbg!("after set:", &grid);
    occupied_set.push(next_tile);
    cur_omino_size += 1; 

    if cur_omino_size == size { 
      //we have produced an omino of the desired size 
      out.push(occupied_set.clone());

    } else {
      //we aren't done with this omino yet, so we need to update reachability and so on
      let mut new_reachable_set = reachable_set.clone();
      //dbg!(Grid::get_neighbors(next_tile));
      let free_neighbors: SmallVec<[Point; 4]> = Grid::get_neighbors(next_tile).into_iter()
        .filter(|&neighbor|grid.get_pos(neighbor) == Free)
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
    //but note though it is reachable it is no longer in "reachable set" because that's actually "reachable todo" or something
    grid.set_pos(next_tile, Reachable);
  }
}

fn enumerate_polyominos(size : u8) -> Vec<PointList> {
  
  let mut out = vec![];
  let mut enum_grid = Grid::default(); 
  let mut reachable_set : SmallVec<[Point; 8]> = smallvec![Point{x: 0, y:0}];
  enum_grid.set_pos(Point{x:0, y:0}, TileState::Reachable);
  let mut occupied_set : SmallVec<[Point; 8]> = smallvec![];
  let mut cur_omino_size : u8 = 0; 

  enumerate_recursion(&mut out, &mut enum_grid, reachable_set, &mut occupied_set, cur_omino_size, size);



  return out 
}
fn main() {
  for i in (1..=6) {
    println!("{}-ominoes: {}", i, enumerate_polyominos(i).len());
  }
}
//num ominos, fixed:
/*
1 | 1
2 | 2 
3 | 6
4 | 19
5 | 63
6 | 216
7 | 760
8 | 2725
9 | 9910
10 | 36446 
*/