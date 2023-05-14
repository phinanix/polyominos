use crate::{assemble::*, omino::*};
use Dir::*;

const OFFSET: usize = 32;
const MAX_SIZE: usize = OFFSET * 2;

pub struct Board(pub [[bool; MAX_SIZE]; MAX_SIZE]);

impl Board {
  pub fn empty() -> Self {
    Self([[false; MAX_SIZE]; MAX_SIZE])
  }
  pub fn add_always(&mut self, fpl: &[FreePoint]) {
    for pt in fpl {
      *self.get_mut(*pt) = true;
    }
  }
  pub fn add(&mut self, fpl: &[FreePoint]) -> bool {
    for pt in fpl {
      if self.contains(*pt) {
        return false;
      }
    }
    for pt in fpl {
      *self.get_mut(*pt) = true;
    }
    return true;
  }
  pub fn undo(&mut self, fpl: &[FreePoint]) {
    for pt in fpl {
      assert!(self.0[OFFSET + pt.x as usize][OFFSET + pt.y as usize]);
      self.0[OFFSET + pt.x as usize][OFFSET + pt.y as usize] = false;
    }
  }
  #[inline]
  pub fn add_<I>(&mut self, fpl: impl Fn() -> I) -> bool
  where
    I: Iterator<Item = FreePoint>,
  {
    for pt in fpl() {
      if self.contains(pt) {
        return false;
      }
    }
    for pt in fpl() {
      *self.get_mut(pt) = true;
    }
    return true;
  }
  #[inline]
  pub fn undo_<I>(&mut self, fpl: impl Fn() -> I)
  where
    I: Iterator<Item = FreePoint>,
  {
    for pt in fpl() {
      // assert!(self.0[OFFSET + pt.x as usize][OFFSET + pt.y as usize]);
      *self.get_mut(pt) = false;
    }
  }
  #[inline]
  pub fn contains(&self, pt: FreePoint) -> bool {
    // self.0[OFFSET + pt.x as usize][OFFSET + pt.y as usize]
    unsafe { *self.0.get_unchecked(OFFSET + pt.x as usize).get_unchecked(OFFSET + pt.y as usize) }
  }
  #[inline]
  pub fn get_mut(&mut self, pt: FreePoint) -> &mut bool {
    unsafe {
      self.0.get_unchecked_mut(OFFSET + pt.x as usize).get_unchecked_mut(OFFSET + pt.y as usize)
    }
  }
  pub fn next_edge_to_cover(&self) -> Option<Edge> {
    let edges_to_cover =
      [N, E, S, W].map(|d| Edge(offset_in_dir(FreePoint { x: 0, y: 0 }, d), d.flip()));
    edges_to_cover.iter().copied().filter(|Edge(pt, _d)| !self.contains(*pt)).next()
  }
}

pub fn covers_board(
  ominos: &[&[FreePoint]; 4],
  perimeters: &[&[Edge]; 4],
  board: &mut Board,
) -> bool {
  let Some(edge_to_cover) = board.next_edge_to_cover() else { return true };
  let Edge(pt_to_cover, dir_to_cover) = edge_to_cover;

  for i in 0..=3 {
    for &(Edge(fp, d)) in perimeters[i] {
      if (d == dir_to_cover) {
        let translation = translation_of_a_to_b(fp, pt_to_cover);
        if board.add_(|| translate_omino_iter(&ominos[i], translation)) {
          if covers_board(ominos, perimeters, board) {
            return true;
          }
          board.undo_(|| translate_omino_iter(&ominos[i], translation));
        }
      }
    }
  }

  false
}

pub fn has_arrangement_board(omino: &FreePointList) -> bool {
  /*
  Given an omino, searches for a set of translation+rotationss which arrange
  that omino to surround the hole (0,0) with nooverlap, or returns None if
  there are none

  to do this, we progressively try to cover the sides of the hole in CW order
  starting with N via a depth first search, backtracking whenever there is no
  way to proceed.
   */
  if has_corner_arrangement_unsorted(omino) {
    return true;
  }
  let mut rotated_ominos = [0, 1, 2, 3].map(|amt| rotate_omino(omino, amt));
  // for i in (0..=3) {
  //   rotated_ominos[i].sort_unstable();
  //   if has_corner_arrangement(&rotated_ominos[i]) {
  //     return true;
  //   }
  // }
  let perimeters = rotated_ominos.each_ref().map(|omino| iter_perimeter(&omino));
  let mut board = Board::empty();
  let rotated_ominos_borrows: [&[FreePoint]; 4] = rotated_ominos.each_ref().map(|x| &x[..]);
  covers_board(&rotated_ominos_borrows, &perimeters.each_ref().map(|x| &x[..]), &mut board)
}

fn has_corner_arrangement_unsorted(omino: &[FreePoint]) -> bool {
  let mut min_x = omino[0].x;
  let mut max_x = omino[0].x;
  let mut min_y = omino[0].y;
  let mut max_y = omino[0].y;
  for pt in omino {
    min_x = min_x.min(pt.x);
    max_x = max_x.max(pt.x);
    min_y = min_y.min(pt.y);
    max_y = max_y.max(pt.y);
  }
  for pt in omino {
    if (pt.x == min_x || pt.x == max_x) && (pt.y == min_y || pt.y == max_y) {
      return true;
    }
  }
  false
}
