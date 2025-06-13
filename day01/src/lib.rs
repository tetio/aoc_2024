pub fn part01(input: &str) -> i32 {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    input.split("\n").for_each(|l| {
        let row = l.split_whitespace().collect::<Vec<&str>>();
        left.push(row[0].parse().expect("left should be a number"));
        right.push(row[1].parse().expect("right should be a number"));
    });
    left.sort();
    right.sort();
    let mut distance = 0;
    for i in 0..left.len() {
        distance += (left[i] - right[i]).abs();
    }
    return distance;
}

pub fn part02(input: &String) -> i32 {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    input.split("\n").for_each(|l| {
        let row = l.split_whitespace().collect::<Vec<&str>>();
        left.push(row[0].parse().expect("left should be a number"));
        right.push(row[1].parse().expect("right should be a number"));
    });
    return left
        .iter()
        .map(|l| l * right.iter().filter(|r| *r == l).count() as i32)
        .sum();
}
