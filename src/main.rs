
mod omino;
mod assemble;

use crate::omino::{enumerate_polyominos, slow_omino_enum};

fn main() {
  for i in 16..=16 {
    println!("{}-ominoes: {}", i, enumerate_polyominos(i).len());
  }
  
  // let n = 6;
  // let slominos : Vec<PointList> = slow_omino_enum(n).into_iter().map(|o|o.into_iter().map(|p|p.into()).collect()).collect();
  // let fastinos = enumerate_polyominos(n).into_iter().map(|mut o|{o.sort(); o}).collect_vec();
  // dbg!(slominos.len());
  // dbg!(fastinos.len());
  // let missing = slominos.into_iter().filter(|o|!fastinos.contains(o)).collect_vec();
  // dbg!(missing.len());
  
  // dbg!(&slominos);
  // dbg!(&fastinos);
  // dbg!(&missing);
  
}
//num ominos, fi`ed:
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
