
mod omino;
mod assemble;

use assemble::find_arrangement_translation;
use itertools::Itertools;

use crate::{omino::{enumerate_polyominos}, assemble::rotational_deduplicate};

fn main() {
  // for i in 16..=16 {
  //   println!("{}-ominoes: {}", i, enumerate_polyominos(i).len());
  // }
  
  for i in 8..=8 {
    let ominos = enumerate_polyominos(i);
    let num_ominos = ominos.len();
    let untranslateable_ominos = ominos.into_iter()
      .map(|pts|pts.into_iter().map(|pt|pt.into()).collect())
      .filter(|omino|find_arrangement_translation(omino).is_none())
      .collect_vec();
    println!("{}-ominoes, count: {} untranslateable: {}",
      i, num_ominos, untranslateable_ominos.len());
    dbg!(rotational_deduplicate(&untranslateable_ominos));
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
