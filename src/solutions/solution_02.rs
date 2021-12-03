use crate::input;

pub fn a(lines: input::InputLines) {
    let instructions = parse_instructions(lines);

    let mut horizontal_position = 0;
    let mut depth = 0;

    for instruction in instructions {
        let dir = instruction.0;
        let val = instruction.1;
        match dir {
            "forward" => horizontal_position += val,
            "up" => depth -= val,
            "down" => depth += val,
            &_ => {}
        }
    }

    println!("{}", horizontal_position * depth);
}

pub fn b(lines: input::InputLines) {
    let instructions = parse_instructions(lines);

    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;


    for instruction in instructions {
        let dir = instruction.0;
        let val = instruction.1;
        match dir {
            "forward" => { horizontal_position += val; depth += aim * val },
            "up" => aim -= val,
            "down" => aim += val,
            &_ => {}
        }
    }

    println!("{}", horizontal_position * depth);
}

type Instruction<'a> = (&'a str, i32);

fn parse_instructions(lines: input::InputLines) -> Vec<Instruction> {
    lines
        .map(|s| {
            let mut v = s.split(" ");
            (v.next().unwrap(), v.next().unwrap().parse::<i32>().unwrap())
        })
        .collect()
}
