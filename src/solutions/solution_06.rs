use crate::input;

pub fn a(lines: input::InputLines) {
    let mut fish = get_fish(lines);

    for _ in 0..80 {
        generation(&mut fish);
    }

    println!("{}", fish.len());
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

fn generation(fish: &mut Vec<u32>) {
    let len = fish.len();

    for i in 0..len {
        if fish[i] == 0 {
            fish[i] = 6;
            fish.push(8)
        } else {
            fish[i] -= 1;
        }
    }
}
