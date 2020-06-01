//! Find the minimum count of blocks to fit in given shape.
//! 
//! Gien shape an blocks are represeted as coordiate point see https://www.mathworks.com/help/images/image-coordinate-systems.html
//! 
//! ```
//! 
//! If given problem is
//! 
//! [
//!     [1, 1, 0, 0],
//!     [0, 1, 0, 0],
//!     [0, 1, 1, 1],
//!     [0, 1, 0, 0]
//! ]
//! 
//! As corodinate system it's ((0,0), (1,0), (1.1), (1,2), (1,3), (2,2), (3,2))
//! 
//! then minumum is 3 like below
//! 
//! [
//!     [1, 4, 0, 0],
//!     [0, 4, 0, 0],
//!     [0, 4, 2, 2],
//!     [0, 4, 0, 0]
//! ]
//! 
//! [
//!     [2, 2, 0, 0],
//!     [0, 3, 0, 0],
//!     [0, 3, 2, 2],
//!     [0, 3, 0, 0]
//! ]
//! 
//! this is 4 so it's not an answer.
//! 
//! [
//!     [2, 2, 0, 0],
//!     [0, 1, 0, 0],
//!     [0, 3, 3, 3],
//!     [0, 1, 0, 0]
//! ]
//! ```
//! 

use std::collections::HashSet;

/// vertex of block which is coordinated system.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Ord, PartialOrd)]
struct Vertex(u32, u32);

/// show only coordinate point without sturct name
impl std::fmt::Debug for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
        .field(&self.0)
        .field(&self.1)
        .finish()
    }
}

/// list of vertex which means shape of the block.
struct Block {
    vertices: Vec<Vertex>,
}

/// basically Shape is list of vertices but It's not a Block
type Shape = Vec<Vertex>;

/// show only list of vertex without sturct name
impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.vertices).finish()
    }
}

/// find the minimum count of blocks to fit in given shape.
fn solution(problem: &Shape, blocks: &Vec<Block>) -> usize {
    println!("#@ given vector of points:  {:?}", problem);
    for block in blocks {
        println!("#@ type of blocks:  {:?}", block);
    }
    
    // given problem is vector but it is not guaranteed to be sorted.
    // so problem could be possible to be [(0,0) (2,0) (1,0)].
    let mut sorted_problem = problem.clone();
    sorted_problem.sort();
    println!("#@ sorted_problem: {:?}", sorted_problem);
    
    // hashsets are copied from vector so vector is still accessible.
    let set_of_problem: HashSet<Vertex> = problem.iter().cloned().collect();
    let mut caculated_points: HashSet<Vertex> = HashSet::new();
    let mut count_of_blocks = 0;
    for point in &sorted_problem {
        if !caculated_points.contains(point) {
            // skip iterate to scan toward right
            let horizontal_max_length = scan_right(point, &set_of_problem);
            let vertical_max_length = scan_above(point, &set_of_problem);
            if horizontal_max_length >= vertical_max_length {
                let mut point_of_block: Vec<Vertex> = Vec::new();
                // insert current point
                point_of_block.push(*point);
                for length in 1..horizontal_max_length {
                    // shape has larger width to fit by given blocks
                    // points fit by block and add to map for preventing iterate again
                    let used_point = Vertex(point.0 + length, point.1);
                    caculated_points.insert(used_point);
                    // insert used point to represent block of shape
                    point_of_block.push(used_point);
                }
                count_of_blocks +=1;
                println!("{:?} block of points are {:?}", count_of_blocks, point_of_block)
            } else {
                let mut point_of_block: Vec<Vertex> = Vec::new();
                // insert current point
                point_of_block.push(*point);
                for length in 1..vertical_max_length {
                    // shape has larger height to fit by given blocks
                    // points fit by block and add to map for preventing iterate again
                    let used_point = Vertex(point.0, point.1 + length);
                    caculated_points.insert(used_point);
                    // insert used point to represent block of shape
                    point_of_block.push(used_point);
                }
                count_of_blocks +=1;
                println!("{:?} block of points are {:?}", count_of_blocks, point_of_block)
            }
        }
    }

    count_of_blocks
}

fn scan_above(current_point: &Vertex, all_of_points: &HashSet<Vertex>) -> u32 {
    // check to above from start point
    let mut next_point = Vertex(current_point.0, current_point.1 + 1);
    let mut max_size = 1;
    while all_of_points.contains(&next_point) {
        max_size += 1;
        next_point = Vertex(next_point.0, next_point.1 + 1);
    }
    max_size
}

fn scan_right(current_point: &Vertex, all_of_points: &HashSet<Vertex>) -> u32 {
    // check to right from start point
    let mut next_point = Vertex(current_point.0 + 1, current_point.1);
    let mut max_size = 1;
    while all_of_points.contains(&next_point) {
        max_size += 1;
        next_point = Vertex(next_point.0 + 1, next_point.1);
    }
    max_size
}

mod tests {
    use super::*;
    
    #[test]
    fn first_shape() {
        // not guaranteed to be sorted
        let problem = vec![
            Vertex(3,2),
            Vertex(2,2),
            Vertex(1,0), Vertex(1,1), Vertex(1,2), Vertex(1,3),
            Vertex(0,0),
        ];

        // not guaranteed to be sorted
        let blocks = vec![
            Block { vertices: vec![Vertex(0,0)] },
            Block { vertices: vec![Vertex(0,0), Vertex(1,0)] },
            Block { vertices: vec![Vertex(0,0), Vertex(1,0), Vertex(2,0)] },
            Block { vertices: vec![Vertex(0,0), Vertex(1,0), Vertex(2,0), Vertex(3,0)] },
        ];

        assert_eq!(solution(&problem, &blocks), 3);
    }
}