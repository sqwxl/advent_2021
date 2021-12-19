use crate::input;

pub fn a(lines: input::InputLines) {
    let gamma_str = parse_gamma(lines);
    let epsilon_str: String = gamma_str
        .clone()
        .chars()
        .map(|b| if b == '0' { '1' } else { '0' })
        .collect();

    let gamma = u32::from_str_radix(&gamma_str.to_string(), 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon_str.to_string(), 2).unwrap();

    println!("{}", gamma * epsilon);
}

fn parse_gamma(lines: input::InputLines) -> String {
    let line_count = lines.clone().count();
    let mut ones = Vec::new();
    for line in lines {
        for (i, b) in line.chars().enumerate() {
            if ones.len() <= i {
                ones.push(0);
            }
            if b == '1' {
                ones[i] += 1;
            }
        }
    }

    ones.into_iter()
        .map(|count| if count > line_count / 2 { '1' } else { '0' })
        .collect::<String>()
}

pub fn b(lines: input::InputLines) {
    let numbers = lines.clone()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect::<Vec<_>>();

    let bits = lines.clone().nth(0).unwrap().len();

    let (oxygen_generator_rating, co2_scrubber_rating) = compute_ratings(numbers, bits);

    let life_support_rating = oxygen_generator_rating * co2_scrubber_rating;

    println!("{}", life_support_rating);
}

fn compute_ratings(input_numbers: Vec<u32>, bits: usize) -> (u32, u32) {
    let oxygen_generator_rating = (0..bits)
        .rev()
        .scan(input_numbers.clone(), |oxy, pos| {
            let has_more_ones =
                oxy.iter().filter(|n| *n & 1 << pos > 0).count() >= (oxy.len() + 1) / 2;
            let mut i = 0;
            while i < oxy.len() {
                if (oxy[i] & 1 << pos > 0) != has_more_ones {
                    oxy.remove(i);
                } else {
                    i += 1;
                }
            }
            println!("{}", pos);
            oxy.first().copied()
        })
        .last()
        .unwrap();

    let co2_scrubber_rating = (0..bits)
        .rev()
        .scan(input_numbers.clone(), |oxy, pos| {
            let has_more_ones =
                oxy.iter().filter(|n| *n & 1 << pos > 0).count() >= (oxy.len() + 1) / 2;
            let mut i = 0;
            while i < oxy.len() {
                if (oxy[i] & 1 << pos > 0) == has_more_ones {
                    oxy.remove(i);
                } else {
                    i += 1;
                }
            }
            oxy.first().copied()
        })
        .last()
        .unwrap();

    (oxygen_generator_rating, co2_scrubber_rating)
}
