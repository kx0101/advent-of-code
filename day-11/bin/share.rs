#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Galaxy,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Coord {
    pub fn calculate_distance(&self, other: &Self) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

pub fn empty_rows(grid: &[Vec<Tile>]) -> Vec<usize> {
    grid.iter()
        .enumerate()
        .filter_map(|(index, row)| {
            if !row.contains(&Tile::Galaxy) {
                Some(index)
            } else {
                None
            }
        })
        .collect()
}

pub fn empty_cols(grid: &Vec<Vec<Tile>>) -> Vec<usize> {
    let mut cols: Vec<Vec<Tile>> = vec![vec![Tile::Empty; grid.len()]; grid[0].len()];

    for (r_index, row) in grid.iter().enumerate() {
        for (c_index, col) in row.iter().enumerate() {
            cols[c_index][r_index] = *col;
        }
    }

    empty_rows(&cols)
}

pub fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Galaxy,
                    _ => panic!("Wtf"),
                })
                .collect()
        })
        .collect()
}

pub fn galaxy_coordinates(grid: &Vec<Vec<Tile>>, expansion: Option<usize>) -> Vec<Coord> {
    let empty_rows = empty_rows(grid);
    let empty_cols = empty_cols(grid);

    let mut galaxies = Vec::new();
    let mut curr_row = 0;
    let mut curr_col = 0;

    for (r_index, row) in grid.iter().enumerate() {
        if empty_rows.contains(&r_index) {
            if let Some(exp) = expansion {
                curr_row += exp;
            } else {
                curr_row += 2;
            }

            continue;
        }

        for (c_index, col) in row.iter().enumerate() {
            if empty_cols.contains(&c_index) {
                if let Some(exp) = expansion {
                    curr_row += exp;
                } else {
                    curr_row += 2;
                }

                continue;
            }

            if *col == Tile::Galaxy {
                galaxies.push(Coord {
                    row: curr_row,
                    col: curr_col,
                })
            }

            curr_col += 1;
        }

        curr_col = 0;
        curr_row += 1;
    }

    galaxies
}

fn main() {}
