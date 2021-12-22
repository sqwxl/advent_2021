use crate::input;

pub fn a(lines: input::InputLines) {
    let lines = parse_input(lines);

    let verticals = verticals(&lines);
    let horizontals = horizontals(&lines);

    let mut grid = make_grid(&lines);
    walk_v(&verticals, &mut grid);
    walk_h(&horizontals, &mut grid);

    let mut count = 0;
    for row in grid.iter() {
        for n in row.iter() {
            if *n > 1 {
                count += 1
            }
        }
    }

    println!("{}", count);
}

pub fn b(lines: input::InputLines) {
    let lines = parse_input(lines);

    let verticals = verticals(&lines);
    let horizontals = horizontals(&lines);
    let diagonals = diagonals(&lines);

    let mut grid = make_grid(&lines);
    walk_v(&verticals, &mut grid);
    walk_h(&horizontals, &mut grid);
    walk_d(&diagonals, &mut grid);

    let mut count = 0;
    for row in grid.iter() {
        for n in row.iter() {
            if *n > 1 {
                count += 1
            }
        }
    }
    println!("{}", count);
}

#[derive(Clone, Copy)]
struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

fn make_grid(lines: &Vec<Line>) -> Vec<Vec<u32>> {
    let h: u32 = lines.iter().fold(0, |h, &l| {
        let m = u32::max(l.y1, l.y2);
        if m > h {
            m
        } else {
            h
        }
    });

    let w: u32 = lines.iter().fold(0, |w, &l| {
        let m = u32::max(l.x1, l.x2);
        if m > w {
            m
        } else {
            w
        }
    });

    let mut grid = Vec::new();

    for _ in 0..=h {
        let mut row = Vec::new();
        for _ in 0..=w {
            row.push(0);
        }
        grid.push(row);
    }

    grid
}

fn parse_input(lines: input::InputLines) -> Vec<Line> {
    lines
        .map(|l| {
            let (left, right) = l.split_once(" -> ").unwrap();
            let (x1, y1) = left.split_once(",").unwrap();
            let (x2, y2) = right.split_once(",").unwrap();
            Line {
                x1: x1.parse().unwrap(),
                y1: y1.parse().unwrap(),
                x2: x2.parse().unwrap(),
                y2: y2.parse().unwrap(),
            }
        })
        .collect()
}

fn verticals(lines: &Vec<Line>) -> Vec<Line> {
    lines
        .into_iter()
        .filter(|l| l.x1 == l.x2)
        .map(|m| *m)
        .collect()
}

fn horizontals(lines: &Vec<Line>) -> Vec<Line> {
    lines
        .into_iter()
        .filter(|l| l.y1 == l.y2)
        .map(|m| *m)
        .collect()
}

fn diagonals(lines: &Vec<Line>) -> Vec<Line> {
    lines
        .into_iter()
        .filter(|l| l.x1 != l.x2 && l.y1 != l.y2)
        .map(|m| *m)
        .collect()
}

fn walk_v(lines: &Vec<Line>, board: &mut Vec<Vec<u32>>) {
    for line in lines.iter() {
        let mut min_max = [line.y1, line.y2];
        min_max.sort();
        for y in min_max[0]..=min_max[1] {
            board[line.x1 as usize][y as usize] += 1;
        }
    }
}

fn walk_h(lines: &Vec<Line>, board: &mut Vec<Vec<u32>>) {
    for line in lines.iter() {
        let mut min_max = [line.x1, line.x2];
        min_max.sort();
        for x in min_max[0]..=min_max[1] {
            board[x as usize][line.y1 as usize] += 1;
        }
    }
}

fn walk_d(lines: &Vec<Line>, board: &mut Vec<Vec<u32>>) {
    for line in lines.iter() {
        let dx = if line.x1 < line.x2 { 1 } else { -1 };
        let dy = if line.y1 < line.y2 { 1 } else { -1 };
        let mut x = line.x1 as i32;
        let mut y = line.y1 as i32;
        loop {
            board[x as usize][y as usize] += 1;
            if x == line.x2 as i32 {
                break;
            }
            x += dx;
            y += dy;
        }
    }
}
