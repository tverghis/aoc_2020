const INPUT: &'static str = include_str!("../input.txt");
const TREE: char = '#';

struct Slope {
    right: usize,
    down: usize,
}

impl Slope {
    fn new(right: usize, down: usize) -> Self {
        Slope { right, down }
    }
}

fn main() {
    let lines = INPUT
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let p1 = part_1(&lines);
    println!("Part 1: {} trees", p1);

    let p2 = part_2(&lines);
    println!("Part 2: {}", p2);
}

fn part_1(lines: &[Vec<char>]) -> usize {
    count_trees_in_slope(lines, &Slope::new(3, 1))
}

fn part_2(lines: &[Vec<char>]) -> usize {
    let slopes = vec![
        Slope::new(1, 1),
        Slope::new(3, 1),
        Slope::new(5, 1),
        Slope::new(7, 1),
        Slope::new(1, 2),
    ];

    slopes
        .iter()
        .map(|slope| count_trees_in_slope(lines, slope))
        .fold(1, |acc, trees| acc * trees)
}

fn count_trees_in_slope(lines: &[Vec<char>], slope: &Slope) -> usize {
    let mut row = 0;
    let mut col = 0;

    let mut tree_count = 0;

    // Assume all lines have the same length
    let max_col_pos = lines[0].len();

    loop {
        col = (col + slope.right) % max_col_pos;

        row += slope.down;

        if row >= lines.len() {
            break;
        }

        if lines[row][col] == TREE {
            tree_count += 1;
        }
    }

    tree_count
}
