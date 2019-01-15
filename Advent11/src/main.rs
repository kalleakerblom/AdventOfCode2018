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
fn calc_integral_image(serial: i32) -> [[i32; 300]; 300] {
    let mut ii = [[0; 300]; 300];
    for y in 0..300 {
        let mut row_sum = 0;
        for x in 0..300 {
            let current_cell = calc_grid_val(1 + x as i32, 1 + y as i32, serial);
            row_sum += current_cell;
            let above_sum = if y == 0 { 0 } else { ii[y - 1][x] };
            ii[y][x] = row_sum + above_sum;
        }
    }
    ii
}
fn sum_from_integral_image(ii: &[[i32; 300]; 300], upper_left: (usize, usize), size: usize) -> i32 {
    let (x, y) = upper_left;
    let mut sum = ii[y + size - 1][x + size - 1];
    if size == 1 {
        return sum;
    }
    if x > 0 {
        sum -= ii[y + size - 1][x - 1];
    }
    if y > 0 {
        sum -= ii[y - 1][x + size - 1];
    }
    if x > 0 && y > 0 {
        sum += ii[y - 1][x - 1];
    }
    sum
}
fn main() {
    let serial = 6548;
    let integral_img = calc_integral_image(serial);
    let mut max = (i32::min_value(), (0, 0, 0));
    for x in 0..300 {
        for y in 0..300 {
            //determine size range
            let max_size = 300 - cmp::max(x, y);
            for size in 2..max_size {
                let sum = sum_from_integral_image(&integral_img, (x, y), size);
                if sum > max.0 {
                    max = (sum, (x + 1, y + 1, size));
                }
            }
        }
    }
    println!("ans2: {:?}", max.1);
}
