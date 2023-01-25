#![allow(unused)]
mod assemble;
mod omino;

use assemble::find_arrangement_translation;
use itertools::Itertools;

use crate::{
  assemble::{find_arrangement, rotational_deduplicate},
  omino::enumerate_polyominos,
};

fn main() {
  // for i in 16..=16 {
  //   println!("{}-ominoes: {}", i, enumerate_polyominos(i).len());
  // }

  let lim = 12;
  for i in lim..=lim {
    let ominos = enumerate_polyominos(i);
    let num_ominos = ominos.len();
    let untranslateable_ominos = ominos
      .into_iter()
      .map(|pts| pts.into_iter().map(|pt| pt.into()).collect())
      .filter(|omino| find_arrangement(omino).is_none())
      .collect_vec();
    println!(
      "{}-ominoes, count: {} untranslateable: {}",
      i,
      num_ominos,
      untranslateable_ominos.len()
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
