use std::cmp;
use std::collections::{HashMap, HashSet};

fn calc_grid_val(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += serial;
    power *= rack_id;
    power = (power % 1000) / 100;
    power -= 5;
    power
}
fn main() {
    assert_eq!(calc_grid_val(3, 5, 8), 4);
    assert_eq!(calc_grid_val(122, 79, 57), -5);
    assert_eq!(calc_grid_val(217, 196, 39), 0);
    assert_eq!(calc_grid_val(101, 153, 71), 4);

    let serial = 6548;
    let mut grid = HashMap::with_capacity(300 * 300);
    for y in 1..=300 {
        for x in 1..=300 {
            grid.insert((x, y), calc_grid_val(x, y, serial));
        }
    }
    let mut grid_values = HashMap::new();
    for x in 1..300 {
        for y in 1..300 {
            //determine size range
            let max_size = 300 - cmp::max(x, y);
            grid_values.insert((x, y, 1), grid[&(x, y)]);
            for size in 2..max_size {
                let mut sum = grid_values[&(x, y, size - 1)];
                for _x in x..x + size {
                    sum += grid[&(_x, y + size - 1)];
                }
                for _y in y..y + size - 1 {
                    sum += grid[&(x + size - 1, _y)];
                }
                grid_values.insert((x, y, size), sum);
            }
            println!("{:?}", (x, y));
        }
    }
    let ans2 = grid_values.iter().max_by_key(|p| p.1);
    println!("{:?}", ans2);
}
