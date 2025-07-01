
const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

pub fn part_01(input: &str) -> usize {
    let width = input.split("\n").count();    
    let letters: Vec<char> = input.split("\n").flat_map(|l| l.chars()).collect();
    (0..width)
        .flat_map(|x| (0..width).map(move |y| (x, y)))
        .flat_map(|p| make_moves().into_iter().map(move |m| (p, m)))
        .filter(|param| is_valid(&param.1, param.0.0, param.0.1, &letters, width))
        .count()
}


pub fn part_02(input: &str) -> usize {
    let width = input.split("\n").count();    
    let letters: Vec<char> = input.split("\n").flat_map(|l| l.chars()).collect();
    (0..width)
        .flat_map(|x| (0..width).map(move |y| (x, y)))
        .filter(|p| get_letter(&letters, p.0 as i32, p.1 as i32, width) == 'A')
        .flat_map(|p| make_moves_02().into_iter().map(move |m| (p, m)))
        .filter(|param| is_valid_02(&param.1, param.0.0, param.0.1, &letters, width))
        .count()
}

fn is_valid(moves: &Vec<(i32, i32)>, x: usize, y: usize, letters: &Vec<char>, width: usize) -> bool {
    let mut i: usize = 0;
    for p in moves {
        let l = get_letter(&letters, x as i32 + p.0, y as i32 + p.1, width);
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

fn is_valid_02(moves: &Vec<(i32, i32)>, x: usize, y: usize, letters: &Vec<char>, width: usize) -> bool {
    let l0 = get_letter(&letters, x as i32 + moves[0].0, y as i32 + moves[0].1, width);
    let l1 = get_letter(&letters, x as i32 + moves[1].0, y as i32 + moves[1].1, width);
    let l2 = get_letter(&letters, x as i32 + moves[2].0, y as i32 + moves[2].1, width);
    let l3 = get_letter(&letters, x as i32 + moves[3].0, y as i32 + moves[3].1, width);
    // l0 == 'S' && l1 == 'M' && l2 == 'S' && l3 == 'M' 
    // || l0 == 'M' && l1 == 'S' && l2 == 'M' && l3 == 'S' 
    (l0 == 'S' && l1 == 'M' || l0 == 'M' && l1 == 'S') && (l2 == 'S' && l3 == 'M' || l2 == 'M' && l3 == 'S')
}

fn make_moves_02() -> Vec<Vec<(i32, i32)>> {
    vec![vec![(-1,1), (1,-1), (-1,-1), (1,1)]]
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

const INPUT_1_2: &str = "XMAS
XMAS
XMAS
XMAS";

const INPUT_2_0: &str = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

    #[test]
    fn test_01_001() {
        let res = part_01(INPUT);
        assert_eq!(res, 18);
    }

    #[test]
    fn test_01_002() {
        let res = part_01(INPUT_1_2);
        assert_eq!(res, 6);
    }

        #[test]
    fn test_02_001() {
        let res = part_02(INPUT_2_0);
        assert_eq!(res, 9);
    }
}
