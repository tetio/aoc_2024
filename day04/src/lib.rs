
const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

pub fn part_01(input: &str) -> i32 {
    
    let letters: Vec<char> = input.split("\n").flat_map(|l| l.chars()).collect();
    // (0..width).for_each(|x| );
    // for (x, y) in (0..width).zip(0..width) {
    //     println!("x: {}, {}", x, y);
    // }
    let width = input.split("\n").count();
    let mut res = 0;
    for x in 0..width {
        for y in 0..width {
            for m in make_moves() {
                if is_valid(&m, x, y, &letters, width) {
                    println!("OK ({x}, {y}) -A>({:?})", &m);
                    res += 1;
                } 
            }
        }
    }
    res
}

fn is_valid(moves: &Vec<(i32, i32)>, x: usize, y: usize, letters: &Vec<char>, width: usize) -> bool {
    let mut i: usize = 0;
    for p in moves {
        let l = get_letter(&letters, x as i32 + p.0, y as i32 + p.1, width);
        println!("({x}, {y}) + ({0}, {1})-> {l}", p.0, p.1);
        if l == XMAS[i] {
            i += 1;
        } else {
            break;
        }
    }
    i == 4
}

fn get_letter(letters: &Vec<char>, x: i32, y: i32, width: usize) -> char {
    if x < 0 || x >= width as i32 || y < 0 || y >= width as i32 {
        return ' ';
    }
    let res = letters
        .get(x as usize + y as usize * width)
        .expect("There should be a char in letters at ({x},{y})");
    *res
}

fn make_moves() -> Vec<Vec<(i32, i32)>> {
    vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],        
        vec![(0, 0), (1, 1), (2, 2), (3, 3)],

        vec![(0, 0), (0, -1), (0, -2), (0, -3)],
        vec![(0, 0), (-1, -1), (-2, -2), (-3, -3)],
        vec![(0, 0), (-1, 0), (-2, 0), (-3, 0)],
        vec![(0, 0), (-1, 1), (-2, 2), (-3, 3)],
        vec![(0, 0), (1, -1), (2, -2), (3, -3)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
    ]
}

#[cfg(test)]
mod tests {
    use crate::*;
    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

const INPUT_2: &str = "XMAS
XMAS
XMAS
XMAS";

    #[test]
    fn test_01_001() {
        let res = part_01(INPUT);
        assert_eq!(res, 18);
    }

    #[test]
    fn test_01_002() {
        let res = part_01(INPUT_2);
        assert_eq!(res, 6);
    }
}
