use crate::input;

type Board<'a> = Vec<Vec<&'a str>>;
type Boards<'a> = Vec<Board<'a>>;
type Nums<'a> = Vec<&'a str>;

pub fn a(lines: input::InputLines) {
    let (boards, nums) = make_boards(lines);
    let mut remainders = boards.clone();
    let mut result = 0;
    'outer: for num in nums.iter() {
        for (i, board) in boards.iter().enumerate() {
            for (j, row) in board.iter().enumerate() {
                for (k, n) in row.iter().enumerate() {
                    if n == num {
                        remainders[i][j][k] = "X";
                    }
                }
            }
            if wins(&remainders[i]) {
                print_board(&remainders[i]);
                let n = num.parse::<u32>().unwrap();
                result = score(&remainders[i], n);
                break 'outer;
            }
        }
    }

    println!("{}", result);
}

pub fn b(lines: input::InputLines) {
    let (boards, nums) = make_boards(lines);
    let mut remainders = boards.clone();
    let mut result = 0;
    let mut won = vec![false; boards.len()];
    for num in nums.iter() {
        for (i, board) in boards.iter().enumerate() {
            if !won[i] {
            for (j, row) in board.iter().enumerate() {
                for (k, n) in row.iter().enumerate() {
                    if n == num {
                        remainders[i][j][k] = "X";
                    }
                }
            }
            if wins(&remainders[i]) {
                won[i] = true;
                let n = num.parse::<u32>().unwrap();
                result = score(&remainders[i], n);
            }
            }
        }
    }

    println!("{}", result);
}

fn make_boards(lines: input::InputLines) -> (Boards, Nums) {
    let nums = lines
        .clone()
        .next()
        .unwrap()
        .split(',')
        .collect::<Vec<&str>>();

    let lines = lines.clone().skip(2);

    // make boards
    let boards = lines.fold(vec![vec![]], |acc, line| {
        let mut result = acc.clone();
        if line.len() == 0 {
            result.push(vec![]);
        } else {
            let row = line.split_whitespace().collect::<Vec<&str>>();
            result.last_mut().unwrap().push(row);
        }
        result
    });

    (boards, nums)
}

fn print_board(board: &Vec<Vec<&str>>) {
    board.iter().for_each(|row| println!("{}", row.join(" - ")))
}

fn score(board: &Vec<Vec<&str>>, n: u32) -> u32 {
    let mut sum = 0;
    for i in 0..5 {
        for j in 0..5 {
            if let Ok(k) = board[i][j].parse::<u32>() {
                sum += k;
            }
        }
    }

    sum * n
}

fn wins(board: &Vec<Vec<&str>>) -> bool {
    // horizontal
    for row in board.iter() {
        if row.iter().all(|n| *n == "X") {
            return true;
        }
    }
    // vertical
    for j in 0..5 {
        let mut win = true;
        for i in 0..5 {
            if board[i][j] != "X" {
                win = false;
                break;
            }
        }
        if win {
            return true;
        }
    }
    // diagonal
    let right = vec![0, 1, 2, 3, 4];
    let left = vec![4, 3, 2, 1, 0];

    for dia in vec![right, left] {
        let mut win = true;
        for (i, a) in dia.iter().enumerate() {
            if board[i][*a as usize] != "X" {
                win = false;
                break;
            }
        }
        if win {
            return true;
        }
    }

    false
}

