use std::collections::HashMap;

use crate::input::InputLines;

const ONE: usize = 2;
const FOUR: usize = 4;
const SEVEN: usize = 3;
const EIGHT: usize = 7;

pub fn a(lines: InputLines) {
    let input = parse_input(lines);

    let mut counts = vec![0; 8];

    input.1.iter().for_each(|j| {
        j.iter().for_each(|h| {
            counts[h.len()] += 1;
        })
    });

    const ONE: usize = 2;
    const FOUR: usize = 4;
    const SEVEN: usize = 3;
    const EIGHT: usize = 7;

    let uniques = counts[ONE] + counts[FOUR] + counts[SEVEN] + counts[EIGHT];

    println!("{}", uniques);
}

pub fn b(lines: InputLines) {
    let (signals, outputs) = parse_input(lines);

    let n = signals.len();

    // segments
    //
    //  00000
    // 5     1
    // 5     1
    //  66666
    // 4     2
    // 4     2
    //  33333
    //

    let zero = [0, 1, 2, 3, 4, 5];
    let one = [1, 2];
    let two = [0, 1, 3, 4, 6];
    let three = [0, 1, 2, 3, 6];
    let four = [1, 2, 5, 6];
    let five = [0, 2, 3, 5, 6];
    let six = [0, 2, 3, 4, 5, 6];
    let seven = [0, 1, 2];
    let eight = [0, 1, 2, 3, 4, 5, 6];
    let nine = [0, 1, 2, 3, 5, 6];

    let digit_segment_code = (zero, one, two, three, four, five, six, seven, eight, nine);

    let unique_lengths = [ONE, FOUR, SEVEN, EIGHT];

    for i in 0..n {
        let signal = &signals[i];
        let output = &outputs[i];

        let mut signal_map = HashMap::new();

        // Uniques
        for &s in signal.iter() {
            let len = s.len();
            if unique_lengths.contains(&len) {
                signal_map.insert(len, s);
            }
        }
        let group_0_6_9 = signal.iter().filter(|s| s.len() == 6).copied();
        let group_2_3_5 = signal.iter().filter(|s| s.len() == 5).copied();

        let sig_one = signal_map.get(&1).copied().unwrap();
        let sig_four = signal_map.get(&4).copied().unwrap();
        let sig_seven = signal_map.get(&7).copied().unwrap();
        let sig_eight = signal_map.get(&8).copied().unwrap();

        let mut segment_map = HashMap::new();

        // 1 and 7 give us segment 0
        let s0 = sig_seven.chars().filter(|c| sig_one.contains(*c)).last().unwrap();
        segment_map.insert(s0, 0);

        // we can find 9 and segment 3 by combining sigs one, four and seven
        
    }
}

fn parse_input(lines: InputLines) -> (Vec<Vec<&str>>, Vec<Vec<&str>>) {
    lines.clone().fold((vec![], vec![]), |mut res, line| {
        let (s, o) = line.split_once(" | ").unwrap();

        res.0.push(s.split(' ').collect::<Vec<_>>());
        res.1.push(o.split(' ').collect::<Vec<_>>());

        res
    })
}
