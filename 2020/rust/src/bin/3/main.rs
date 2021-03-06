use std::{error::Error, fs};

struct Slope {
    dx: usize,
    dy: usize,
}

impl From<&(usize, usize)> for Slope {
    fn from(slope_tuple: &(usize, usize)) -> Self {
        Slope {
            dx: slope_tuple.0,
            dy: slope_tuple.1,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "./src/bin/3/input.txt";
    let hill_str = fs::read_to_string(path)?;

    let hill_grid: Vec<Vec<char>> = hill_str
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let slope_tuples = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let slopes: Vec<Slope> = slope_tuples
        .iter()
        .map(|slope_tuple: &(usize, usize)| Slope::from(slope_tuple))
        .collect();

    let slopes_product = slopes.iter().fold(1, |acc: u64, cur| {
        acc * calc_num_of_trees_hit(cur.dx, cur.dy, &hill_grid)
    });

    dbg!(slopes_product);

    Ok(())
}

fn calc_num_of_trees_hit(dx: usize, dy: usize, hill_grid: &Vec<Vec<char>>) -> u64 {
    let mut trees_hit = 0;
    let mut r = dy;
    let mut c = dx;

    // should we make hill_grid's type a sized Vec so compiler knows hill_grid[0]? if hill_grid[0] DNE is panics
    while r < hill_grid.len() {
        let hill_width = hill_grid[0].len();
        if c >= hill_width {
            c %= hill_width;
        }

        if hill_grid[r][c] == '#' {
            trees_hit += 1
        }

        r += dy;
        c += dx;
    }

    trees_hit
}
