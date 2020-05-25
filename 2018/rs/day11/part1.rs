const SERIAL_NUMBER: i32 = 1788;

struct Grid {
    storage: Vec<Vec<i32>>,
}

impl Grid {
    fn new(capacity: usize) -> Grid {
        Grid {
            storage: vec![Vec::with_capacity(capacity); capacity],
        }
    }

    fn get_memoized(&mut self, x: usize, y: usize) -> i32 {
        if let Some(num) = self.storage[y].get(x) {
            *num
        } else {
            self.storage[y].push(get_power_level(x, y));
            self.storage[y][x]
        }
    }
}

fn get_power_level(x: usize, y: usize) -> i32 {
    let rack_id = (x + 10) as i32;
    let mut power_level = rack_id * y as i32;
    power_level += SERIAL_NUMBER;
    power_level *= rack_id;
    power_level = (power_level / 100) % 10;
    power_level - 5
}

fn main() {
    let mut grid = Grid::new(300);

    let mut result_x: usize = 0;
    let mut result_y: usize = 0;
    let mut largest_grid = 0;

    for y in 0..297 {
        for x in 0..297 {
            let mut grid_power_level = 0;
            for yy in 0..3 {
                for xx in 0..3 {
                    grid_power_level += grid.get_memoized(x + xx, y + yy);
                }
            }

            if grid_power_level > largest_grid {
                result_x = x;
                result_y = y;
                largest_grid = grid_power_level;
            }
        }
    }

    println!("( {}, {} )", result_x, result_y);
}
