#![allow(unused)] 

use std::collections::HashSet;
use std::cmp::Ordering;
use Ordering::*;
use smallvec::{smallvec,SmallVec};

use crate::omino::{FreePoint, FreePointList, sum_points, Dir, translate_omino, offset_in_dir, normalize_omino};
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

impl Edge {
  pub fn flip(self) -> Self {
    match self {
      Edge(pt, d) => Edge(offset_in_dir(pt, d), d.flip())
    }
  }
}

pub fn iter_perimeter(fps: &FreePointList) -> Vec<Edge> {
  let points_occupied : HashSet<FreePoint> = fps.iter().cloned().collect(); 
  let mut out = vec![];
  for &pt in fps {
    for (neighbor, dir) in pt.get_neighbors_with_directions() {
      if !points_occupied.contains(&neighbor) {
        out.push(Edge(pt, dir));
      }
    }
  }
  out
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

pub fn rotational_equivalence(omino: &FreePointList, omino2: &FreePointList) -> bool {
  let mut sorted_omino = normalize_omino(omino.clone());
  sorted_omino.sort();
  let rotate_fns = [rotate_0, rotate_cw, rotate_180, rotate_ccw];
  
  rotate_fns.into_iter().any(|f| {
    let mut rotated_omino2: FreePointList = normalize_omino(
      omino2.iter().map(|&pt|f(pt)).collect());
    rotated_omino2.sort();
    rotated_omino2 == sorted_omino
  })
}

pub fn rotational_deduplicate(ominos: &Vec<FreePointList>) -> Vec<FreePointList> {
  let mut out = vec![];
  for omino in ominos {
    if ! out.iter().any(|prev_omino|rotational_equivalence(prev_omino, omino)) {
      out.push(omino.clone());
    }
  }

  out 
}

pub fn rotate_omino(omino: &FreePointList, rotate_amt: u8) -> FreePointList {
  //invariant: rotate_amt is 0,1,2,3 and represents the number of cw turns
  todo!()
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

pub fn translation_of_a_to_b(
  FreePoint { x: px, y: py }: FreePoint, 
  FreePoint { x: qx, y: qy }: FreePoint) 
  -> FreePoint
{
  FreePoint { x: qx - px, y: qy - py }
}

pub fn align_perim(omino: &FreePointList, src: Edge, Edge(target_point, target_dir): Edge)
 -> FreePointList 
{
  let (rotated_omino, rotated_src_pt) = rotate_omino_edge(omino, src, target_dir);
  let translation = translation_of_a_to_b(rotated_src_pt, target_point);
  return translate_omino(rotated_omino, translation)
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ConfigurationTranslation{
  pts: FreePointList, //invariant: sorted
  translations: SmallVec<[FreePoint; 4]>
}

pub fn merge_pts_slow(pts : &FreePointList, mut new_pts : FreePointList) -> Option<FreePointList> {
  /* invariants: pts is sorted. new_pts is not sorted. 
  if pts and new_pts overlap, then return None. 
  else, return a new sorted list of pts that is their union. 
   */
  let set: HashSet<_> = HashSet::from_iter(pts.iter());
  if new_pts.iter().any(|pt| set.contains(pt)) {
    return None
  } else {
    new_pts.extend(pts.clone());
    return Some(new_pts)
  }

}

pub fn merge_pts(pts : &FreePointList, mut new_pts : FreePointList) -> Option<FreePointList> {
  /* invariants: pts is sorted. new_pts is not sorted. 
  if pts and new_pts overlap, then return None. 
  else, return a new sorted list of pts that is their union. 
  */
  let mut out = smallvec![];
  new_pts.sort_unstable();
  let mut pts_index = 0; 
  let mut new_pts_index = 0;
  while pts_index < pts.len() && new_pts_index < new_pts.len() {
    match pts[pts_index].cmp(&new_pts[new_pts_index]) {
      Equal => return None, 
      Less => {out.push(pts[pts_index]); pts_index += 1},
      Greater => {out.push(new_pts[new_pts_index]); new_pts_index += 1},
    }
  }

  while pts_index < pts.len() {
    out.push(pts[pts_index]);
    pts_index += 1; 
  }
  
  while new_pts_index < new_pts.len() {
    out.push(new_pts[new_pts_index]);
    new_pts_index += 1; 
  }

  assert!(out.len() == pts.len() + new_pts.len());
  Some(out)
}

pub fn next_edge_to_cover(pts: &FreePointList) -> Option<Edge> { 
  //returns one of [((0, 1), S), ((1, 0), W), ((0, -1), N), ((-1, 0), E)]
  let edges_to_cover = [N, E, S, W].map(|d|Edge(offset_in_dir(FreePoint{x: 0, y: 0}, d), d.flip()));
  edges_to_cover.iter().copied().filter(|Edge(pt, _d)|!pts.contains(pt)).next()
}

pub fn translate_a_to_b(pts: FreePointList, a: FreePoint, b: FreePoint) 
  -> (FreePointList, FreePoint) 
{
  // dbg!("translating", a, b);
  let translation = translation_of_a_to_b(a, b);
  // dbg!("translation", translation);
  (translate_omino(pts, translation), translation)
}

pub fn add_translation_children(omino: &FreePointList, perimeter: &Vec<Edge>,
  stack: &mut Vec<ConfigurationTranslation>, ConfigurationTranslation { pts, translations }: ConfigurationTranslation)
  -> Option<SmallVec<[FreePoint; 4]>> 
{
  /* adds the children of the given configuration to the stack, and returns None, 
  unless a successful configuration is found, in which case it is returned */

  let Edge(pt_to_cover, dir_to_cover) = next_edge_to_cover(&pts).unwrap();
  // dbg!(&dir_to_cover, &pt_to_cover);
  let mut possible_edges = perimeter.iter()
    .filter(|&&(Edge(fp, d))| d == dir_to_cover);
  let mut translated_ominos_and_translations = possible_edges
    .map(|&(Edge(src_pt, _src_dir))| translate_a_to_b(omino.clone(), src_pt, pt_to_cover));
  for (translated_omino, translation) in translated_ominos_and_translations {
    // dbg!(&translated_omino, &translation);
    if let Some(merged_pts) = merge_pts(&pts, translated_omino) {
      let mut new_translations = translations.clone();
      new_translations.push(translation);
      if let None = next_edge_to_cover(&merged_pts) {
        //we win, return that
        return Some(new_translations)
      } else {
        let new_config = ConfigurationTranslation{pts: merged_pts, translations: new_translations};
        // dbg!("new config:", &new_config);
        stack.push(new_config);
      }
    }
  }

  None
}

pub fn find_arrangement_translation(omino: &FreePointList) -> Option<SmallVec<[FreePoint;4]>> {
  /*
  Given an omino, searches for a set of translations which arrange that omino to 
  surround the hole (0,0) with no overlap, or returns None if there are none

  to do this, we progressively try to cover the sides of the hole in CW order 
  starting with N via a depth first search, backtracking whenever there is no
  way to proceed. Since we are only looking at translations and not rotations, 
  to cover the N side of the hole, we must use a S facing edge, and so on. 
   */
  let mut stack : Vec<ConfigurationTranslation> = vec![ConfigurationTranslation::default()]; //todo, empty configuration
  let perimeter = iter_perimeter(omino);
  while let Some(config) = stack.pop() {
    // dbg!(&config);
    match add_translation_children(omino, &perimeter, &mut stack, config) {
      Some(ans) => return Some(ans), 
      None => (),
    }
  }
  None
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Configuration{
  pts: FreePointList, //invariant: sorted
  edge_pairs: SmallVec<[(Edge, Edge); 4]>
}

pub fn add_tr_children(ominos: &[FreePointList; 4], perimeters: &[Vec<Edge>; 4],
  stack: &mut Vec<Configuration>, Configuration { pts, edge_pairs }: Configuration)
  -> Option<SmallVec<[(Edge, Edge); 4]>> 
{
  /* adds the children of the given configuration to the stack, and returns None, 
  unless a successful configuration is found, in which case it is returned */

  let edge_to_cover = next_edge_to_cover(&pts).unwrap();
  
  //dbg!(edge_to_cover);

  fn extract_possible_pairs<'a>(omino: &'a FreePointList, perimeter: &'a Vec<Edge>, edge_to_cover: Edge) 
  -> impl Iterator<Item = (FreePointList, (Edge, Edge))> + 'a {
    let Edge(pt_to_cover, dir_to_cover) = edge_to_cover;
    perimeter.iter().filter_map(move |&(Edge(fp, d))| {
      if (d == dir_to_cover) {
        Some((translate_a_to_b(omino.clone(), fp, pt_to_cover).0, 
              (Edge(fp, d), edge_to_cover)))
      } else {
        None 
      }
    })
  }

  let mut possible_ominos_and_edge_pairs = (0..=3).flat_map(|i|
    extract_possible_pairs(&ominos[i], &perimeters[i], edge_to_cover));

  for (moved_omino, edge_pair) in possible_ominos_and_edge_pairs {
    if let Some(merged_pts) = merge_pts(&pts, moved_omino) {
      let mut new_pairs = edge_pairs.clone();
      new_pairs.push(edge_pair);
      if let None = next_edge_to_cover(&merged_pts) {
        //we win, return that
        return Some(edge_pairs)
      } else {
        let new_config = Configuration{pts: merged_pts, edge_pairs: new_pairs};
        // dbg!("new config:", &new_config);
        stack.push(new_config);
      }
    }
  }

  None
}

pub fn find_arrangement(omino: &FreePointList) -> Option<SmallVec<[(Edge, Edge);4]>> {
  /*
  Given an omino, searches for a set of translation+rotationss which arrange 
  that omino to surround the hole (0,0) with nooverlap, or returns None if 
  there are none

  to do this, we progressively try to cover the sides of the hole in CW order 
  starting with N via a depth first search, backtracking whenever there is no
  way to proceed. 
   */
  let mut stack = vec![Configuration::default()];
  let rotated_ominos = [0,1,2,3].map(|amt|rotate_omino(omino, amt));
  let perimeters = rotated_ominos.clone().map(|omino|iter_perimeter(&omino));
  while let Some(config) = stack.pop() {
    //dbg!(&config);
    match add_tr_children(&rotated_ominos, &perimeters, &mut stack, config) {
      Some(ans) => return Some(ans), 
      None => (),
    }
  }
  None
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
      let pts = [(0,0), (1,3), (4,4), (-3, 6), (3, -5), (20, 0)].map(|(x,y)| FreePoint{x, y});
      pts.map(|p|point_assert(p));
    }

}