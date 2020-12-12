use aoc_derive::impl_day;

type NearbySearchFunc = fn(&Vec<Vec<char>>, usize, usize, usize, usize) -> usize;

#[inline]
fn nearby_search_p1(
    matrix: &Vec<Vec<char>>,
    height: usize,
    width: usize,
    i: usize,
    j: usize,
) -> usize {
    let mut occupied = 0;

    for x in -1..=1 {
        for y in -1..=1 {
            if x | y != 0 {
                let i = i as isize + x;
                let j = j as isize + y;

                if i >= 0 && i < height as isize && j >= 0 && j < width as isize {
                    occupied += (matrix[i as usize][j as usize] == '#') as usize;
                }
            }
        }
    }

    occupied
}

#[inline]
fn nearby_search_p2(
    matrix: &Vec<Vec<char>>,
    height: usize,
    width: usize,
    i: usize,
    j: usize,
) -> usize {
    let mut occupied = 0;

    for x in -1..=1 {
        for y in -1..=1 {
            if x | y != 0 {
                let mut i = i as isize + x;
                let mut j = j as isize + y;

                while i >= 0
                    && i < height as isize
                    && j >= 0
                    && j < width as isize
                    && matrix[i as usize][j as usize] == '.'
                {
                    i += x;
                    j += y;
                }

                if i >= 0 && i < height as isize && j >= 0 && j < width as isize {
                    occupied += (matrix[i as usize][j as usize] == '#') as usize;
                }
            }
        }
    }

    occupied
}

fn get_occupied(
    mut matrix: Vec<Vec<char>>,
    handler: NearbySearchFunc,
    occupied_min: usize,
) -> usize {
    let height = matrix.len();
    let width = matrix[0].len();

    let mut changed = true;

    while changed {
        let mut new_matrix = vec![vec!['.'; width]; height];

        changed = false;

        for i in 0..height {
            for j in 0..width {
                if matrix[i][j] != '.' {
                    let occupied = handler(&matrix, height, width, i, j);

                    new_matrix[i][j] = match matrix[i][j] {
                        '#' => {
                            if occupied >= occupied_min {
                                changed = true;
                                'L'
                            } else {
                                '#'
                            }
                        }
                        'L' => {
                            if occupied > 0 {
                                'L'
                            } else {
                                changed = true;
                                '#'
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }

        matrix = new_matrix;
    }

    matrix.iter().flatten().filter(|&c| c == &'#').count()
}

impl_day!(11, |reader| {
    let matrix: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|l| l.ok())
        .take_while(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let p1 = get_occupied(matrix.to_vec(), nearby_search_p1, 4);
    let p2 = get_occupied(matrix.to_vec(), nearby_search_p2, 5);

    println!("Part1: {}", p1);
    println!("Part2: {}", p2);
});
