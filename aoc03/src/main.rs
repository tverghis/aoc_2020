const INPUT: &'static str = include_str!("../input.txt");
const TREE: char = '#';

fn main() {
    let lines = INPUT
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let p1 = part_1(&lines);
    println!("Part 1: {} trees", p1);
}

fn part_1(lines: &[Vec<char>]) -> u32 {
    let mut row = 0;
    let mut col = 0;

    let mut tree_count = 0;

    // Assume all lines have the same length
    let max_col_pos = lines[0].len();

    loop {
        col = (col + 3) % max_col_pos;

        row += 1;

        if row == lines.len() {
            break;
        }

        if lines[row][col] == TREE {
            tree_count += 1;
        }
    }

    tree_count
}
