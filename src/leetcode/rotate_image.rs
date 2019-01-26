use std::ops::{Div, Sub};

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        for i in 0..=matrix.len().div(2) {
            for j in i..matrix.len().sub(i + 1) {
                let mut value: i32 = matrix[i][j];
                for (row, col) in Solution::get_transformation_tuples(i, j, matrix.len()) {
                    let temp = matrix[row][col];
                    matrix[row][col] = value;
                    value = temp;
                }
            }
        }
    }

    fn get_transformation_tuples(row: usize, column: usize, size: usize) -> Vec<(usize, usize)> {
        let mut output: Vec<(usize, usize)> = Vec::new();
        output.push((column, size - row - 1));
        output.push((size - row - 1, size - column - 1));
        output.push((size - column - 1, row));
        output.push((row, column));
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        assert_eq!(expected, matrix);
    }

    #[test]
    fn test_example_2() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];

        Solution::rotate(&mut matrix);
        let expected = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];
        assert_eq!(expected, matrix);
    }
}