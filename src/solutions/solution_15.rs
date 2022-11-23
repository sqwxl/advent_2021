use std::cmp::Reverse;
use std::ops::Index;
use std::collections::BinaryHeap;

pub fn a(input: &str) {
    let map = parse_input(input);
    print!("{}", lowest_risk(map));
}

pub fn b(input: &str) {
    let map = parse_input(input);
    let w = map[0].len();
    let h = map.len();
    let expanded = (0..(5*w)).map(|x| {
        (0..(5*h)).map(|y| {
            let n = map[x % w][y % h]
             + (x / w) as u32
             + (y /h) as u32;
            if n > 9 { n - 9 } else { n }
        }).collect()
    }).collect();

    print!("{}", lowest_risk(expanded));

}

pub fn lowest_risk(map: Vec<Vec<u32>>) -> u32 {
    let w = map[0].len();
    let h = map.len();
    
    let goal = (w-1, h-1);

    let mut dist = vec![vec![u32::MAX; w]; h];

    let mut q = BinaryHeap::new();
    q.push(Reverse((0,0,0)));

    while let Some(Reverse((cost,x,y))) = q.pop() {
        if (x,y) == goal { return cost; }

        if cost > dist[x][y] { continue; }

        for (xx, yy) in [(x-1, y), (x+1, y), (x, y-1), (x, y+1)] {
            let next_cost = match map.get(xx).and_then(|r| r.get(yy)) {
                Some(c) => cost+c,
                None => continue,
            };
            if next_cost < dist[xx][yy] {
                dist[xx][yy] = next_cost;
                q.push(Reverse((next_cost, xx, yy)));
            }
        }
    }
    unreachable!()
}


fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect()
}
