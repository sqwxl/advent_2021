use crate::input;

pub fn a(lines: input::InputLines) {
    let mut crabs = parse_input(&lines);
    crabs.sort();
    let n = crabs.len();

    let med = match n % 2 == 0 {
        true => crabs[n / 2],
        false => (crabs[(n - 1) / 2] + crabs[(n + 1) / 2]) / 2,
    };

    let cost = crabs.iter().fold(0, |cost, pos| cost + (pos - med).abs());

    println!("{}", cost);
}

pub fn b(lines: input::InputLines) {
    let mut crabs = parse_input(&lines);
    crabs.sort();

    let max = crabs.last().copied().unwrap();

    let mut fuels = vec![0; max as usize + 1];

    for i in 0..=max {
        crabs.iter().for_each(|pos| {
            let fuel = fuel(i32::abs(pos - i));
            fuels[i as usize] += fuel;
        });
    }

    let min = fuels.iter().min().unwrap();

    println!("{}", min)
}

fn fuel(n: i32) -> i32 {
    let mut f = 0;
    for i in 1..=n {
        f += i;
    }
    f
}

fn parse_input(lines: &input::InputLines) -> Vec<i32> {
    lines
        .clone()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}
