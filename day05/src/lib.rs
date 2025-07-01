use std::collections::HashMap;

pub fn part_01(input: &str) -> i32 {
    let mut order: HashMap<String, Vec<String>> = HashMap::new();
    let spec: Vec<_> = input.split("\n\n").collect();
    spec[0].split("\n").for_each(|s| {
        let values: Vec<_> = s.split("|").collect();
        order.entry(values[0].to_string()).or_insert_with(Vec::new).push(values[1].to_string());
    });
    let _z = order.get("75").unwrap();
    let data: Vec<Vec<i32>> = Vec::new();
    let _ = spec[1].split("\n").map(|l| { 
        let a: Vec<_> = l.split(",").collect();
        a.count();
    });
    
    
    0
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT1: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    pub fn test_01_001() {
        let res = part_01(INPUT1);
        assert_eq!(res, 112);
    }
}