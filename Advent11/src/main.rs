use std::cmp;

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
    let serial = 6548;
    let ii = IntegralImage::new(
        300,
        300,
        Box::new(move |x, y| calc_grid_val(1 + x as i32, 1 + y as i32, serial)),
    );
    let mut max = (i32::min_value(), (0, 0, 0));
    for x in 0..300 {
        for y in 0..300 {
            //determine size range
            let max_size = 300 - cmp::max(x, y);
            for size in 1..max_size {
                let sum = ii.get_rectangle_sum((x, y), size).expect("get sum");
                if sum > max.0 {
                    max = (sum, (x + 1, y + 1, size));
                }
            }
        }
    }
    println!("ans2v2: {:?}", max.1);
}

struct IntegralImage {
    sums: Vec<Vec<i32>>,
}

impl IntegralImage {
    fn new(width: usize, length: usize, generator: Box<Fn(usize, usize) -> i32>) -> IntegralImage {
        let mut sums = vec![vec![0; width]; length];
        for y in 0..length {
            let mut row_sum = 0;
            for x in 0..width {
                let current_cell = generator(x, y);
                row_sum += current_cell;
                let above_sum = if y == 0 { 0 } else { sums[y - 1][x] };
                sums[y][x] = row_sum + above_sum;
            }
        }

        IntegralImage { sums }
    }

    /// Will return None if rectangle size is larger than bounds of the integral image
    fn get_rectangle_sum(&self, upper_left: (usize, usize), size: usize) -> Option<i32> {
        let (x, y) = upper_left;
        if size == 0 {
            return Some(0);
        }
        let mut sum = *self.sums.get(y + size - 1)?.get(x + size - 1)?;
        if x > 0 {
            sum -= self.sums.get(y + size - 1)?.get(x - 1)?;
        }
        if y > 0 {
            sum -= self.sums.get(y - 1)?.get(x + size - 1)?;
        }
        if x > 0 && y > 0 {
            sum += self.sums.get(y - 1)?.get(x - 1)?;
        }
        Some(sum)
    }
}
