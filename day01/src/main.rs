use day01::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("(^)(^)(^)file input.txt not found");
    let res01 = part01(&input);
    println!("Res01 = {}", res01);
    let res02 = part02(&input);
    println!("Res02 = {}", res02)

}
