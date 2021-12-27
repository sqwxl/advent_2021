struct Grid {
    values: Vec<u8>,
    width: i32,
    height: i32,
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Grid {
    fn at(&self, point: Point) -> Option<u8> {
        let Point { x, y } = point;
        let in_bounds = 0 <= x && x < self.width && 0 <= y && y < self.height;
        in_bounds.then(|| self.values[(x + y * self.width) as usize])
    }
}

fn neighbors(grid: &Grid, point: Point) -> Vec<Point> {
    let Point { x, y } = point;
    [
        Point { x, y: y - 1 },
        Point { x, y: y + 1 },
        Point { x: x - 1, y },
        Point { x: x + 1, y },
    ]
    .iter()
    .filter_map(|p| {
        if grid.at(*p).is_some() {
            Some(*p)
        } else {
            None
        }
    })
    .collect()
}

fn neighbor_values(grid: &Grid, point: Point) -> Vec<u8> {
    neighbors(grid, point)
        .iter()
        .filter_map(|xy| grid.at(*xy))
        .collect()
}

fn get_low_points(grid: &Grid) -> Vec<Point> {
    let mut lows = Vec::new();
    (0..grid.height).for_each(|y| {
        (0..grid.width).for_each(|x| {
            if let Some(p) = grid.at(Point { x, y }) {
                if neighbor_values(grid, Point { x, y }).iter().all(|&v| p < v) {
                    lows.push(Point { x, y })
                }
            }
        });
    });

    lows
}

fn bassin(grid: &Grid, point: Point) -> Vec<Point> {
    let mut visited = vec![point];
    let mut next: Vec<Point> = neighbors(grid, point)
        .iter()
        .filter(|&p| grid.at(*p).unwrap() != 9)
        .copied()
        .collect();

    while !next.is_empty() {
        visited.append(&mut next.clone());

        next = next
            .iter()
            .scan(Vec::new(), |state: &mut Vec<Point>, &p| {
                neighbors(grid, p).iter().for_each(|&n| {
                    if grid.at(n) != Some(9)
                        && !state.iter().any(|&v| n.x == v.x && n.y == v.y)
                        && !visited.iter().any(|&v| n.x == v.x && n.y == v.y)
                    {
                        state.push(n);
                    }
                });
                Some(state.clone())
            })
            .last()
            .unwrap()
            .to_vec();
    }

    visited
}

fn parse_grid(input: &str) -> Grid {
    Grid {
        values: input
            .lines()
            .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8))
            .collect::<Vec<_>>(),
        width: input.lines().next().unwrap().len() as i32,
        height: input.lines().count() as i32,
    }
}

pub fn a(input: &str) {
    let grid = parse_grid(input);

    let lows = get_low_points(&grid);

    let risk_levels = lows.iter().map(|&l| grid.at(l).unwrap() as u32 + 1);

    println!("{}", risk_levels.sum::<u32>())
}

pub fn b(input: &str) {
    let grid = parse_grid(input);

    let lows = get_low_points(&grid);

    let mut bassins = lows
        .iter()
        .map(|&l| bassin(&grid, l).len() as u32)
        .collect::<Vec<u32>>();

    bassins.sort_unstable();
    bassins.reverse();

    let result: u32 = bassins[0..3].iter().product();

    println!("{}", result);
}
