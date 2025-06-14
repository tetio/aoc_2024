use regex::Regex;

pub fn part_01(input: &str) -> i32 {

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut res = 0;
    for caps in re.captures_iter(input) {
        //let full_match = caps.get(0).map_or("", |m| m.as_str());
        let arg1: i32 = caps.get(1).map_or("", |m| m.as_str()).parse().expect("arg1 is not a number");
        let arg2: i32 = caps.get(2).map_or("", |m| m.as_str()).parse().expect("arg2 is not a number");
        res += arg1 * arg2;
    }
    res
}

pub fn part_02(input: &str) -> i32 {
    let re = Regex::new(r"(mul\((\d{1,2}),(\d{1,2})\))|(do\(\))|(don't\(\))").unwrap();
    let mut do_op = true;
    for caps in re.captures_iter(input) {
        // `caps.get(0)` is always the full match
        let full_match = caps.get(0).map_or("", |m| m.as_str());
        println!("Full match found: '{}'", full_match);
        let mut do =true;
        match full_match.chars().take(2).collect() {
            "don"  => do_op = false,
            "do(" => do_op = true,  
            _ => {

            }
        }
        //
        // // Check which specific group matched
        // if let Some(mul_match) = caps.get(1) {
        //     // This is a 'mul' sequence
        //     let arg1_str = caps.get(2).map_or("", |m| m.as_str());
        //     let arg2_str = caps.get(3).map_or("", |m| m.as_str());
        //
        //     if let (Ok(n1), Ok(n2)) = (arg1_str.parse::<i32>(), arg2_str.parse::<i32>()) {
        //         println!("  Type: MUL, Args: ({}, {}) -> Product: {}", n1, n2, n1 * n2);
        //     } else {
        //         println!("  Type: MUL, Args: ({}, {}) -> Error parsing numbers", arg1_str, arg2_str);
        //     }
        // } else if let Some(do_match) = caps.get(4) {
        //     // This is a 'do()' sequence
        //     println!("  Type: DO");
        // } else if let Some(dont_match) = caps.get(5) {
        //     // This is a 'don't()' sequence
        //     println!("  Type: DON'T");
        // } else {
        //     println!("  Unknown match type (should not happen with this regex)");
        // }
        // println!("---");
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    pub fn test_01() {
        let res = part_01(INPUT);
        assert_eq!(res, 161);
    }


    #[test]
    pub fn test_01_001() {
        let res = part_01("mul(558,514)");
        assert_eq!(res, 286812);
    }

    #[test]
    pub fn test_02() {
        let res = part_02("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(res, 22);
    }

}