pub fn part_01(input: &str) -> u32 {
    return input
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .map(|r| r.parse::<i32>().expect("Invalid number in report"))
                .collect::<Vec<i32>>()
        })
        .filter(|r| is_safe(&r))
        .count() as u32;
}
fn is_safe(report: &Vec<i32>) -> bool {
    let aux: Vec<i32> = report.windows(2).map(|pair| pair[1] - pair[0]).collect();
    let sign = aux[0].signum();
    aux.iter()
        .filter(|e| e.signum() == 0 || e.signum() != sign || e.abs() >= 4)
        .count() == 0
}
pub fn part_02(input: &str) -> u32 { 
    input
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .map(|r| r.parse::<i32>().expect("Invalid number in report"))
                .collect::<Vec<i32>>()
        })
        .filter(|r| {
            if is_safe(&r) {
                true
            } else {
                let reports = report_generator(&r);
                reports.iter().any(|r| is_safe(&r))
            }
        })
        .count() as u32
}


fn report_generator(report: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    for i in 0..report.len() {
        let mut a: Vec<i32> = Vec::new();
        for j in 0..report.len() {
            if j != i {
                a.push(report[j].clone());
            }
        }
        res.push(a);
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::*;
    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn process_part_01() {
        let res = part_01(INPUT);
        assert_eq!(res, 2);
    }

    #[test]
    fn process_part_02() {
        let res = part_02(INPUT);
        assert_eq!(res, 4);
    }
}
