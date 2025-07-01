use std::fs;

use day05::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File input.txt was not found");
    let res_01 = part_01(input.as_str());
    println!("Res_01 0 {res_01}");
}
