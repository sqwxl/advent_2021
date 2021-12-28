fn score(ch: char) -> u32 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        other => panic!("can't score '{}'!", other),
    }
}

const OPEN: [char; 4] = ['(', '[', '{', '<'];

fn validate_pair(pair: (char, char)) -> bool {
    matches!(pair, ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>'))
}

enum LineStatus {
    Incomplete(u64),
    Corrupt(u32),
}

use LineStatus::*;

fn validate_line(line: &str) -> LineStatus {
    let mut stack: Vec<char> = Vec::new();

    for c in line.chars() {
        if OPEN.contains(&c) {
            stack.push(c);
        } else if let Some(o) = stack.pop() {
            if !validate_pair((o, c)) {
                return Corrupt(score(c));
            }
        }
    }

    let score = stack.iter().rev().fold(0, |score, c| {
        score * 5
            + match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("not an opening char?!"),
            }
    });
    Incomplete(score)
}

pub fn a(input: &str) {
    let syntax_error = input.lines().fold(0, |score, l| {
        if let Corrupt(s) = validate_line(l) {
            score + s
        } else {
            score
        }
    });

    println!("{}", syntax_error);
}
pub fn b(input: &str) {
    let mut scores = input
        .lines()
        .filter_map(|l| match validate_line(l) {
            Incomplete(score) => Some(score),
            Corrupt(_) => None,
        })
        .collect::<Vec<_>>();

    scores.sort_unstable();

    println!("{}", scores[scores.len() / 2])
}
