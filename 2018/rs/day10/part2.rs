use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default)]
struct Dot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Dot {
    fn new() -> Dot {
        Default::default()
    }
}

fn get_max(dots: &Vec<Dot>) -> (i32, i32) {
    dots.iter().fold((0, 0), |(x, y), curr| {
        (
            std::cmp::max(x, i32::abs(curr.x)),
            std::cmp::max(y, i32::abs(curr.y)),
        )
    })
}

fn parse_file() -> Vec<Dot> {
    let mut dots: Vec<Dot> = Vec::new();

    let mut buf = String::new();
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .for_each(|line| {
            let mut dot = Dot::new();
            let mut pos_done = false;

            for c in line.unwrap().chars() {
                match c {
                    '>' => {
                        if pos_done {
                            dot.dy = buf.parse().unwrap();
                        } else {
                            dot.y = buf.parse().unwrap();
                            pos_done = true;
                        }
                        buf.clear();
                    }
                    ',' => {
                        if pos_done {
                            dot.dx = buf.parse().unwrap();
                        } else {
                            dot.x = buf.parse().unwrap();
                        }
                        buf.clear();
                    }
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '-' => {
                        buf.push(c);
                    }
                    _ => continue,
                }
            }

            dots.push(dot);
        });

    return dots;
}

fn main() {
    let mut dots = parse_file();

    let (mut largest_x, mut largest_y) = get_max(&dots);
    let mut ticks = 0;

    loop {
        let mut new_largest_x = 0;
        let mut new_largest_y = 0;

        for dot in &mut dots {
            dot.x += dot.dx;
            dot.y += dot.dy;
            new_largest_x = std::cmp::max(new_largest_x, i32::abs(dot.x));
            new_largest_y = std::cmp::max(new_largest_y, i32::abs(dot.y));
        }

        if new_largest_x <= largest_x && new_largest_y <= largest_y {
            largest_x = new_largest_x;
            largest_y = new_largest_y;
        } else {
            println!("Ticks: {}", ticks);
            break;
        }
        ticks += 1;
    }
}
