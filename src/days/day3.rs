use super::AocDay;
use std::io::BufRead;

pub struct Day3;

fn follow_slop(matrix: &[Vec<bool>], dx: usize, dy: usize) -> usize {
    let height = matrix.len();
    let width = matrix[0].len();

    let mut x = 0;
    let mut y = 0;

    let mut trees_counter = 0;

    while y + dy < height {
        y += dy;
        x = (x + dx) % width;

        trees_counter += matrix[y][x] as usize;
    }

    trees_counter
}

impl AocDay for Day3 {
    fn run<R: BufRead>(reader: R) {
        // matrix of booleans, true means that there is a tree, false means that there
        // is no tree (empty space)
        let matrix = reader
            .lines()
            .filter_map(|l| {
                l.ok()
                    .map(|l| l.chars().map(|c| c == '#').collect::<Vec<bool>>())
            })
            .collect::<Vec<Vec<bool>>>();

        println!("Part1: {}", follow_slop(&matrix, 3, 1));
        println!(
            "Part2: {}",
            follow_slop(&matrix, 1, 1)
                * follow_slop(&matrix, 3, 1)
                * follow_slop(&matrix, 5, 1)
                * follow_slop(&matrix, 7, 1)
                * follow_slop(&matrix, 1, 2)
        );
    }
}
