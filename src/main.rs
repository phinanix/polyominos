#![allow(unused)]
#![feature(is_sorted)]
#![feature(impl_trait_in_fn_trait_return)]
#![feature(array_methods)]
mod assemble;
mod board;
mod omino;

use assemble::find_arrangement_translation;
use itertools::Itertools;
use std::time::SystemTime;

use crate::{
  assemble::{find_arrangement, has_rotated_corner_arrangement, rotational_deduplicate},
  board::has_arrangement_board,
  omino::{enumerate_polyominos, FreePointList, Grid},
};

fn main() {
  // for i in 1..=10 {
  //   println!("{}-ominoes: {}", i, enumerate_polyominos(i).len());
  // }

  let lim = 13;
  // for i in 5..=6 {
  for i in 1..=25 {
    let start = SystemTime::now();
    let mut untranslateable_ominos = vec![];
    let mut num_ominos = 0;
    let ominos = enumerate_polyominos(i, |pts| {
      num_ominos += 1;
      let fpl: FreePointList = pts.into_iter().map(|pt| pt.clone().into()).collect();
      if !has_arrangement_board(&fpl) {
        untranslateable_ominos.push(fpl);
      }
    });
    // let corner_arrangements =
    //   fpl_ominos.iter().filter(|omino| !has_rotated_corner_arrangement(omino)).collect_vec();
    // dbg!(&corner_arrangements.iter().map(|fpl| Grid::from((**fpl).clone())).collect_vec());
    // println!("{}-ominoes, count: {} corner_able: {}", i, num_ominos, corner_arrangements.len());

    // let untranslateable_ominos =
    //   fpl_ominos.into_iter().filter(|omino| find_arrangement(omino).is_none()).collect_vec();
    let end = SystemTime::now();
    println!(
      "{} ominoes, count: {} untranslateable: {}, took {} seconds per 100k ominos",
      i,
      num_ominos,
      untranslateable_ominos.len(),
      (end.duration_since(start).unwrap().as_secs_f64() / num_ominos as f64) * 100_000.0
    );
    if untranslateable_ominos.len() > 0 {
      dbg!(rotational_deduplicate(&untranslateable_ominos));
    }
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
