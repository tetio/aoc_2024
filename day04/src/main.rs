use std::fs;

use day04::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File 'input.txt' not found");
    let res_01 = part_01(input.as_str());
    println!("Part_01 is {res_01}");

    let res_02 = part_02(input.as_str());
    println!("Part_02 is {res_02}");
}
