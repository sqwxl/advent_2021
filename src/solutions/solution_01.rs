pub fn b(lines: std::str::Lines<'_>) {
    let mut count = 0;
    let numbers: Vec<i32> = lines.map(|s: &str| s.parse::<i32>().unwrap()).collect();
    let mut last: i32 = numbers[0..3].iter().sum();
    for i in 1..(numbers.len() - 2) {
        let sum: i32 = numbers[i..i + 3].iter().sum();
        if sum > last {
            count += 1;
        }
        last = sum;
    }
    println!("{}", count);
}

pub fn a(mut lines: std::str::Lines<'_>) {
    let mut count = 0;
    let mut last = lines.next().unwrap().parse::<i32>().unwrap();
    for line in lines {
        let new = line.parse::<i32>().unwrap();
        if new > last {
            count += 1;
        }
        last = new;
    }
    println!("{}", count);
}
