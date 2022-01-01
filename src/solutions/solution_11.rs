use std::collections::HashSet;

const W: usize = 10;
const H: usize = 10;

type Levels = [u8; W * H];

#[derive(Clone, Copy)]
struct Grid {
    levels: Levels,
}

#[derive(Clone, Copy)]
struct Octopus {
    x: i8,
    y: i8,
}

const NEIGHBORS: [[i8; 2]; 8] = [
    [-1, -1],
    [0, -1],
    [1, -1],
    [-1, 0],
    [1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
];

impl Grid {
    fn at(&self, point: Octopus) -> Option<u8> {
        let Octopus { x, y } = point;
        let in_bounds = 0 <= x && x < W as i8 && 0 <= y && y < H as i8;
        if in_bounds {
            Some(self.levels[x as usize + y as usize * W])
        } else {
            None
        }
    }

    fn set_at(&mut self, octopus: Octopus, value: u8) {
        if self.at(octopus).is_some() {
            self.levels[octopus.x as usize + octopus.y as usize * W] = value;
        }
    }

    fn step(self, counter: &mut u32) -> Grid {
        let mut next = Grid {
            levels: self.levels,
        };
        // increase all energy Levels
        for v in next.levels.iter_mut() {
            *v += 1;
        }
        // do the flash thing
        let flashed = next.flash(counter);
        //
        for octopus in flashed.iter() {
            next.set_at(*octopus, 0);
        }

        next
    }

    fn flash(&mut self, counter: &mut u32) -> Vec<Octopus> {
        let mut flashed = HashSet::new();

        loop {
            let mut was_flash = false;
            for y in 0..H {
                for x in 0..W {
                    let octo = Octopus {
                        x: x as i8,
                        y: y as i8,
                    };
                    let v = self.at(octo).unwrap();
                    if v > 9 && flashed.insert((x, y)) {
                        neighbors(self, octo)
                            .iter_mut()
                            .for_each(|o| self.set_at(o.0, o.1 + 1));
                        *counter += 1;
                        was_flash = true;
                    }
                }
            }
            if !was_flash {
                break;
            }
        }

        flashed
            .into_iter()
            .map(|a| Octopus {
                x: a.0 as i8,
                y: a.1 as i8,
            })
            .collect()
    }
}

fn parse_input(input: &str) -> Grid {
    let values: Levels = input
        .lines()
        .enumerate()
        .fold([0; W * H], |a: Levels, (y, v)| {
            let mut result = a;
            v.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .enumerate()
                .for_each(|(x, d)| result[x + y * W] = d);
            result
        });
    Grid { levels: values }
}

fn neighbors(grid: &Grid, point: Octopus) -> Vec<(Octopus, u8)> {
    let Octopus { x, y } = point;
    let mut result = Vec::new();
    for &[xo, yo] in NEIGHBORS.iter() {
        let p = Octopus {
            x: x + xo,
            y: y + yo,
        };
        if let Some(e) = grid.at(p) {
            result.push((p, e));
        }
    }

    result
}

fn step_til_simulatneous(grid: &mut Grid, counter: &mut u32) {
    let mut dummy_counter = 0;
    loop {
        *grid = grid.step(&mut dummy_counter);
        *counter += 1;
        if grid.levels.iter().all(|v| *v == 0) {
            break;
        }
    }
}

pub fn a(input: &str) {
    let mut grid = parse_input(input);
    let mut flashes = 0;
    for _ in 0..100 {
        grid = grid.step(&mut flashes);
    }

    println!("{}", flashes);
}
pub fn b(input: &str) {
    let mut grid = parse_input(input);
    let mut steps = 0;
    step_til_simulatneous(&mut grid, &mut steps);

    println!("{}", steps);
}
