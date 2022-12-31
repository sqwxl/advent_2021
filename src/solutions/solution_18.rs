pub fn a(input: &str) {
    let nums = parse_input(input);
    let mut sum = nums.into_iter().reduce(|a, n| add(&a, &n)).unwrap();
    let mag = magnitude(sum);

    println!("{}", mag);
}
pub fn b(input: &str) {
    let nums = parse_input(input);
    let mut max = 0;
    for a in nums.iter() {
        for b in nums.iter() {
            if a != b {
                let mag = magnitude(add(a, b));
                if mag > max {
                    max = mag;
                }
            }
        }
    }

    println!("{}", max);
}

fn add(n1: &[(u32, u32)], n2: &[(u32, u32)]) -> Vec<(u32, u32)> {
    let mut res = n1.to_owned();

    res.append(&mut n2.to_owned());
    res.iter_mut().for_each(|(_, d)| {
        *d += 1;
    });

    reduce(&mut res);

    res
}

fn reduce(num: &mut Vec<(u32, u32)>) {
    loop {
        if explode(num).is_some() {
            continue;
        }
        if split(num).is_some() {
            continue;
        }
        break;
    }
}

fn explode(num: &mut Vec<(u32, u32)>) -> Option<()> {
    // find i is index of first member of leftmost pair that is at least four levels deep
    if let Some((i, _)) = num.iter().enumerate().find(|(_, &n)| n.1 > 4) {
        // left
        if i > 0 {
            num[i - 1].0 += num[i].0;
        }
        // right
        if i < num.len() - 2 {
            num[i + 2].0 += num[i + 1].0;
        }
        // replace
        num.remove(i + 1);
        num[i] = (0, num[i].1 - 1);

        Some(())
    } else {
        None
    }
}

fn split(num: &mut Vec<(u32, u32)>) -> Option<()> {
    if let Some((i, &(a, d))) = num.iter().enumerate().find(|(_, &n)| n.0 > 9) {
        let round_up = |x: u32| -> u32 {
            if x % 2 == 0 {
                x >> 1
            } else {
                (x >> 1) + 1
            }
        };

        num[i] = (a / 2, d + 1);
        num.insert(i + 1, (round_up(a), d + 1));

        Some(())
    } else {
        None
    }
}

fn max_depth_idx(num: &[(u32, u32)]) -> usize {
    let (mut max_depth, mut max_idx) = (0, 0);
    for i in 0..num.len() {
        if num[i].1 > max_depth {
            max_depth = num[i].1;
            max_idx = i;
        }
    }
    max_idx as usize
}

fn magnitude(mut num: Vec<(u32, u32)>) -> u32 {
    while num.len() > 1 {
        let i = max_depth_idx(&num);
        let ((a, d), b) = (num[i], num[i + 1].0);
        num[i] = (3 * a + 2 * b, d - 1);
        num.remove(i + 1);
    }

    num[0].0
}

fn parse_line(line: &str) -> Vec<(u32, u32)> {
    let (mut depth, mut num) = (0, Vec::new());
    for c in line.chars() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' => {}
            _ => num.push((c.to_digit(10).unwrap(), depth)),
        }
    }

    num
}

fn parse_input(input: &str) -> Vec<Vec<(u32, u32)>> {
    let nums = input.lines().map(parse_line).collect::<Vec<_>>();
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let equal = |a: &[(u32, u32)], b: &[(u32, u32)]| {
            let match_count = a
                .iter()
                .zip(b)
                .filter(|((a, b), (c, d))| a == c && b == d)
                .count();
            match_count == a.len()
        };

        let n = parse_line("[7,9]");
        assert!(equal(&n, &[(7, 1), (9, 1)]));

        let n = parse_line("[[5,4],[[1,3],[5,[2,8]]]]");
        assert!(equal(
            &n,
            &[(5, 2), (4, 2), (1, 3), (3, 3), (5, 3), (2, 4), (8, 4)]
        ));
    }

    #[test]
    fn test_max_depth_idx() {
        let num = parse_line("[9,1]");
        assert_eq!(max_depth_idx(&num), 0);

        let num = parse_line("[[9,1],[1,9]]");
        assert_eq!(max_depth_idx(&num), 0);

        let num = parse_line("[[1,2],[[3,4],5]]");
        assert_eq!(max_depth_idx(&num), 2);

        let num = parse_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(max_depth_idx(&num), 0);

        let num = parse_line("[[[[1,5],[5,0]],[9,4]],[[[8,3],3],[8,[3,6]]]]");
        assert_eq!(max_depth_idx(&num), 0);
    }

    #[test]
    fn test_magnitude() {
        let num = parse_line("[9,1]");
        assert_eq!(magnitude(num), 29);

        let num = parse_line("[1,9]");
        assert_eq!(magnitude(num), 21);

        let num = parse_line("[[9,1],[1,9]]");
        assert_eq!(magnitude(num), 129);

        let num = parse_line("[[1,2],[[3,4],5]]");
        assert_eq!(magnitude(num), 143);

        let num = parse_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(magnitude(num), 1384);
    }

    #[test]
    fn test_explode() {
        let cases = [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ];
        for &(input, expected) in cases.iter() {
            let mut i = parse_line(input);
            explode(&mut i);
            assert_eq!(i, parse_line(expected));
        }
    }

    #[test]
    fn test_split() {
        let cases: [(&[_], &[_]); 2] = [
            (
                &[
                    (0, 4),
                    (7, 4),
                    (4, 3),
                    (15, 3),
                    (0, 4),
                    (13, 4),
                    (1, 2),
                    (1, 2),
                ],
                &[
                    (0, 4),
                    (7, 4),
                    (4, 3),
                    (7, 4),
                    (8, 4),
                    (0, 4),
                    (13, 4),
                    (1, 2),
                    (1, 2),
                ],
            ),
            (
                &[
                    (0, 4),
                    (7, 4),
                    (4, 3),
                    (7, 4),
                    (8, 4),
                    (0, 4),
                    (13, 4),
                    (1, 2),
                    (1, 2),
                ],
                &[
                    (0, 4),
                    (7, 4),
                    (4, 3),
                    (7, 4),
                    (8, 4),
                    (0, 4),
                    (6, 5),
                    (7, 5),
                    (1, 2),
                    (1, 2),
                ],
            ),
        ];

        for &(input, expected) in cases.iter() {
            let mut i = input.to_owned();
            split(&mut i);
            assert_eq!(i, expected);
        }
    }

    #[test]
    fn test_reduce() {
        let mut n = parse_line("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        reduce(&mut n);

        let expected = parse_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(n, expected);
    }

    #[test]
    fn test_add() {
        let cases: &[(&[_], &str)] = &[
            (
                &["[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]"],
                "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            ),
            (
                &["[1,1]", "[2,2]", "[3,3]", "[4,4]"],
                "[[[[1,1],[2,2]],[3,3]],[4,4]]",
            ),
            (
                &["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"],
                "[[[[3,0],[5,3]],[4,4]],[5,5]]",
            ),
            (
                &["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"],
                "[[[[5,0],[7,4]],[5,5]],[6,6]]",
            ),
            (
                &[
                    "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                    "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                    "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                    "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                    "[7,[5,[[3,8],[1,4]]]]",
                    "[[2,[2,2]],[8,[8,1]]]",
                    "[2,9]",
                    "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                    "[[[5,[7,4]],7],1]",
                    "[[[[4,2],2],6],[8,7]]",
                ],
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            ),
        ];

        for &(nums, expected) in cases.iter() {
            let sum = nums
                .iter()
                .map(|&n| parse_line(n))
                .reduce(|a, n| add(&a, &n))
                .unwrap();
            assert_eq!(sum, parse_line(expected));
        }
    }
}

//
// Data flow
//
// input: &str
//   |
// Vec<SnailNumbers>
//   |
// ADD SnailNumbers
//   |
// Sum: SnailNumber
//   |
// MAGNITUDE: u32
//
//
// PROCESS:
//
// 1. Add first two SnailNumbers.
// 2. Reduce the result
//
