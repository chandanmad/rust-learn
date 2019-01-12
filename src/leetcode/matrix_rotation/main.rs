use std::ops::Div;

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        for i in 0..=matrix.len().div(2) {
            for j in (i + 1)..(matrix.len() - i - 1) {
                let im_i32 = 0 - j;
            }
        }
        for arr in matrix {
            println!("{:?}", arr);
        }
    }
}

pub fn main() {
    for (i, j) in (5..=9).enumerate() {
        println!("{} {}", i, j);    
    }
    let mut matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(&mut matrix);
}