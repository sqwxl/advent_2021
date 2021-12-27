use crate::input;

pub fn a(lines: input::InputLines) {
    let fish = get_fish(lines);

    let mut fishes = 0;
    for f in fish {
        fishes += descendants(f, 80)
    }

    println!("{}", fishes);
}

pub fn b(lines: input::InputLines) {
    let fish: Vec<usize> = get_fish(lines).iter().map(|f| *f as usize).collect();

    const NDAYS: usize = 256;

    let mut fish_calendar = vec![0u64; NDAYS + 9]; 

    let mut fish_count = fish.len() as u64;

    for f in &fish {
        fish_calendar[*f] += 1;
    }

    for d in 0..NDAYS {
        let todays = fish_calendar[d];
        fish_calendar[d + 7] += todays;
        fish_calendar[d + 9] += todays;
        fish_count += todays;
    }
    
    println!("{}", fish_count);
}

fn get_fish(lines: input::InputLines) -> Vec<u32> {
    lines
        .clone()
        .next()
        .unwrap()
        .split(',')
        .map(|a| a.parse().unwrap())
        .collect()
}

fn descendants(t: u32, gen: u32) -> u64 {
    match (gen > 0, t > 0) {
        (false, _) => 1,
        (true, false) => descendants(6, gen - 1) + descendants(8, gen - 1),
        (true, true) => descendants(t - 1, gen - 1),
    }
}
