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
        _ => println!("input not recognized. use 'inputs/01A'"),
    }
}
