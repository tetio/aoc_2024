use std::fs;
use day03::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt file not found");
    let res_01 = part_01(input.as_str());
    println!("Res01 = {res_01}");

    let res_02 = part_02(input.as_str());
    println!("Res02 = {res_02}");

}
