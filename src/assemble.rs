#![allow(unused)] 

use std::collections::HashSet;

use smallvec::{smallvec,SmallVec};

use crate::omino::{FreePoint, FreePointList, sum_points, Dir};


/*
The goal of this module is to efficiently test whether an omino can surround a 1x1 
hole. We fix the 1x1 hole to surround to always be (0,0). 
The simplest algorithm for this problem is: given an omino, iterate through 
it's whole perimeter, and try putting that bit of the perimeter adjacent to 
1 of the 4 sides of the hole. Brute force this for all 4 sides and see if any work. 
There's a complication, which is that you can also surround the whole with only 
1, 2 or 3 omino copies. We can separately detect 1x1 holes, so thinking about 2 and 
3, 3 there is a symmetry such that we can assume WLOG that we are bruteforcing the
N, E, W bits of the hole and the S bit is covered by one of the 3 copies. With 
2 ominos, either they each cover a pair of adjacent sides, or one omino covers 3 
sides and the other omino covers 1 side. In both cases, one omino touches N and the
other touches S. 

An algorithm that might actually be easier is DFS: try all possible perimeter segments
for the N face, then go clockwise. When you reach an uncovered bit of the hole, then
try all possible omino adjacencies, and continue with any that don't overlap everything
already placed. In the worst case this is just as slow as brute force, but it is 
better at finding something that works quickly and early exiting (I think). 

There's a more complex algorithm that basically looks like compute the possible 
overlaps once then search inside that graph

Subcomponents: ability to rotate + translate an omino, ability to intersect two 
ominos, iterate an omino's perimeter

 */

//kind of a half edge
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Edge(FreePoint, Dir);


pub fn iter_perimeter(fps: &FreePointList) -> impl Iterator<Item=Edge> {
  let points_occupied : HashSet<FreePoint> = fps.iter().cloned().collect(); 
  let mut out = vec![];
  for &pt in fps {
    for (neighbor, dir) in pt.get_neighbors_with_directions() {
      if !points_occupied.contains(&neighbor) {
        out.push(Edge(pt, dir));
      }
    }
  }
  out.into_iter()
} 

pub fn align_perim(omino: &FreePointList, src: Edge, target: Edge) -> FreePointList {
  todo!()
}

pub mod test {
  use super::*;

    #[test]
    fn neighbors_correct(){
      // let mut ans: PointList = smallvec![Point{x: -2, y: 3}, Point{x: 0, y: 3}, Point{x: -1, y: 4}, Point{x: -1, y: 2}];
      // ans.sort();
      // let mut neighbors = Grid::get_neighbors(Point{x: -1, y: 3});
      // neighbors.sort();
      // assert_eq!(neighbors, ans)
    }

}