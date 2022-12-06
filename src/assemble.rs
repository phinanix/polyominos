#![allow(unused)] 

use std::collections::HashSet;

use smallvec::{smallvec,SmallVec};

use crate::omino::{FreePoint, FreePointList, sum_points, Dir, translate_omino};
use Dir::*;

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


fn dir_to_num(d: Dir) -> i8 {
  match d {
    N => 0, 
    E => 1, 
    S => 2, 
    W => 3
  }
}

pub fn rotate_0(fp: FreePoint) -> FreePoint {
  fp
}

pub fn rotate_cw(FreePoint { x, y }: FreePoint) -> FreePoint {
  FreePoint { x: y, y: -x }
}

pub fn rotate_ccw(FreePoint { x, y }: FreePoint) -> FreePoint {
  FreePoint { x: -y, y: x }
}

pub fn rotate_180(FreePoint { x, y }: FreePoint) -> FreePoint {
  FreePoint { x: -x, y: -y }  
}

pub fn rotate_omino_edge(omino: &FreePointList, Edge(src_point, src_dir): Edge, target_dir: Dir) 
  -> (FreePointList, FreePoint) 
{
  /* takes in an omino, an edge we want to rotate & track, and the direction we'd like
    that edge to face
    returns the rotated omino, and the new (translated) location of that edge. The 
    direction of the edge is target_dir and thus is not returned
  */
  //amount to rotate clockwise, in increments of 90 degrees
  let amt_to_rotate = (dir_to_num(target_dir) - dir_to_num(src_dir) + 4) % 4;
  let rotate_fn = match amt_to_rotate {
    0 => rotate_0,
    1 => rotate_cw, 
    2 => rotate_180,
    3 => rotate_ccw,
    _ => unreachable!("we already did mod 4")
  };

  (omino.iter().map(|&p| rotate_fn(p)).collect(), rotate_fn(src_point))
  
}

pub fn translate_a_to_b(
  FreePoint { x: px, y: py }: FreePoint, 
  FreePoint { x: qx, y: qy }: FreePoint) 
  -> FreePoint
{
  FreePoint { x: qx - px, y: qy - qx }
}

pub fn align_perim(omino: &FreePointList, src: Edge, Edge(target_point, target_dir): Edge)
 -> FreePointList 
{
  let (rotated_omino, rotated_src_pt) = rotate_omino_edge(omino, src, target_dir);
  let translation = translate_a_to_b(rotated_src_pt, target_point);
  return translate_omino(rotated_omino, translation)
}


pub mod test {
  use super::*;

    fn point_assert(fp: FreePoint) {
      assert_eq!(fp, rotate_180(rotate_180(fp)));
      assert_eq!(fp, rotate_cw(rotate_ccw(fp)));
      assert_eq!(rotate_cw(fp), rotate_ccw(rotate_ccw(rotate_ccw(fp))));
      assert_eq!(rotate_ccw(rotate_ccw(fp)), rotate_cw(rotate_cw(fp)));
    }
    
    #[test]
    fn point_fiddling() {
      let pts = [(0,0), (1,3), (4,4), (-3, 6), (3, -5)].map(|(x,y)| FreePoint{x, y});
      pts.map(|p|point_assert(p));
    }

}