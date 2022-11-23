#![allow(dead_code, unused)]
use std::fs;

mod solutions;

pub mod input {
    pub type InputLines<'a> = std::str::Lines<'a>;
}

fn main() {
    let puzzle = std::env::args()
        .nth(1)
        .expect("forgot puzzle id, ie: input/01A");
    let file = &puzzle[0..puzzle.len() - 1];
    let input = fs::read(&file).unwrap();
    let data = String::from_utf8(input).unwrap();
    let lines = data.lines();
    
    let input = fs::read_to_string(file).unwrap();

    println!("{}", puzzle);
    match puzzle.as_str() {
        "inputs/01A" => solutions::solution_01::a(lines),
        "inputs/01B" => solutions::solution_01::b(lines),
        "inputs/02A" => solutions::solution_02::a(lines),
        "inputs/02B" => solutions::solution_02::b(lines),
        "inputs/03A" => solutions::solution_03::a(lines),
        "inputs/03B" => solutions::solution_03::b(lines),
        "inputs/04A" => solutions::solution_04::a(lines),
        "inputs/04B" => solutions::solution_04::b(lines),
        "inputs/05A" => solutions::solution_05::a(lines),
        "inputs/05B" => solutions::solution_05::b(lines),
        "inputs/06A" => solutions::solution_06::a(lines),
        "inputs/06B" => solutions::solution_06::b(lines),
        "inputs/07A" => solutions::solution_07::a(lines),
        "inputs/07B" => solutions::solution_07::b(lines),
        "inputs/08A" => solutions::solution_08::a(lines),
        "inputs/08B" => solutions::solution_08::b(lines),
        "inputs/09A" => solutions::solution_09::a(&input),
        "inputs/09B" => solutions::solution_09::b(&input),
        "inputs/10A" => solutions::solution_10::a(&input),
        "inputs/10B" => solutions::solution_10::b(&input),
        "inputs/11A" => solutions::solution_11::a(&input),
        "inputs/11B" => solutions::solution_11::b(&input),
        "inputs/12A" => solutions::solution_12::a(&input),
        "inputs/12B" => solutions::solution_12::b(&input),
        "inputs/13A" => solutions::solution_13::a(&input),
        "inputs/13B" => solutions::solution_13::b(&input),
        "inputs/14A" => solutions::solution_14::a(&input),
        "inputs/14B" => solutions::solution_14::b(&input),
        "inputs/15A" => solutions::solution_15::a(&input),
        "inputs/15B" => solutions::solution_15::b(&input),
        "inputs/16A" => solutions::solution_16::a(&input),
        "inputs/16B" => solutions::solution_16::b(&input),
        // "inputs/17A" => solutions::solution_17::a(&input),
        // "inputs/17B" => solutions::solution_17::b(&input),
        // "inputs/18A" => solutions::solution_18::a(&input),
        // "inputs/18B" => solutions::solution_18::b(&input),
        // "inputs/19A" => solutions::solution_19::a(&input),
        // "inputs/19B" => solutions::solution_19::b(&input),
        // "inputs/20A" => solutions::solution_20::a(&input),
        // "inputs/20B" => solutions::solution_20::b(&input),
        // "inputs/21A" => solutions::solution_21::a(&input),
        // "inputs/21B" => solutions::solution_21::b(&input),
        // "inputs/22A" => solutions::solution_22::a(&input),
        // "inputs/22B" => solutions::solution_22::b(&input),
        // "inputs/23A" => solutions::solution_23::a(&input),
        // "inputs/23B" => solutions::solution_23::b(&input),
        // "inputs/24A" => solutions::solution_24::a(&input),
        // "inputs/24B" => solutions::solution_24::b(&input),
        // "inputs/25A" => solutions::solution_25::a(&input),
        // "inputs/25B" => solutions::solution_25::b(&input),
        _ => println!("input not recognized. use 'inputs/01A'"),
    }
}
