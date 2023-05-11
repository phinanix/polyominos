use crate::{assemble::*, omino::*};
use Dir::*;

pub struct Board(pub [[bool; 50]; 50]);

const OFFSET: usize = 25;

impl Board {
  pub fn empty() -> Self {
    Self([[false; 50]; 50])
  }
  pub fn add(&mut self, fpl: &[FreePoint]) -> bool {
    for pt in fpl {
      if self.0[OFFSET + pt.x as usize][OFFSET + pt.y as usize] {
        return false;
      }
    }
    for pt in fpl {
      self.0[OFFSET + pt.x as usize][OFFSET + pt.y as usize] = true;
    }
    return true;
  }
  pub fn undo(&mut self, fpl: &[FreePoint]) {
    for pt in fpl {
      assert!(self.0[OFFSET + pt.x as usize][OFFSET + pt.y as usize]);
      self.0[OFFSET + pt.x as usize][OFFSET + pt.y as usize] = false;
    }
  }
  pub fn contains(&self, pt: FreePoint) -> bool {
    self.0[OFFSET + pt.x as usize][OFFSET + pt.y as usize]
  }
  pub fn next_edge_to_cover(&self) -> Option<Edge> {
    let edges_to_cover =
      [N, E, S, W].map(|d| Edge(offset_in_dir(FreePoint { x: 0, y: 0 }, d), d.flip()));
    edges_to_cover.iter().copied().filter(|Edge(pt, _d)| !self.contains(*pt)).next()
  }
}

pub fn covers_board(
  ominos: &[FreePointList; 4],
  perimeters: &[Vec<Edge>; 4],
  board: &mut Board,
) -> bool {
  let Some(edge_to_cover) = board.next_edge_to_cover() else { return true };
  let Edge(pt_to_cover, dir_to_cover) = edge_to_cover;

  for i in 0..=3 {
    for &(Edge(fp, d)) in &perimeters[i] {
      if (d == dir_to_cover) {
        let fpl = translate_a_to_b(&ominos[i], fp, pt_to_cover).0;
        if board.add(&fpl) {
          if covers_board(ominos, perimeters, board) {
            return true;
          }
          board.undo(&fpl);
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
  let mut rotated_ominos = [0, 1, 2, 3].map(|amt| rotate_omino(omino, amt));
  for i in (0..=3) {
    rotated_ominos[i].sort_unstable();
    if has_corner_arrangement(&rotated_ominos[i]) {
      return true;
    }
  }
  let perimeters = rotated_ominos.clone().map(|omino| iter_perimeter(&omino));
  let mut board = Board::empty();
  covers_board(&rotated_ominos, &perimeters, &mut board)
}
