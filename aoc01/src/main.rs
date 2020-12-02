use std::error::Error;

const INPUT: &'static str = include_str!("../input.txt");
const SEARCH_TOTAL: i32 = 2020;

fn main() {
    let mut input_nums = parse_input().collect::<Vec<_>>();
    input_nums.sort();

    let p1 = part_1(&input_nums).unwrap();
    println!("Part 1: {}", p1);

    let p2 = part_2(&input_nums).unwrap();
    println!("Part 2: {}", p2);
}

fn part_1(input: &[i32]) -> Result<i32, Box<dyn Error>> {
    for i in input {
        let search_current = SEARCH_TOTAL - i;
        let search_result = input.binary_search(&search_current);

        if let Ok(idx) = search_result {
            let j = input[idx];
            return Ok(i * j);
        }
    }

    Err("Did not find a pair of numbers.".into())
}

fn part_2(input: &[i32]) -> Result<i32, Box<dyn Error>> {
    for i in 0..input.len() - 2 {
        if input[i] + input[i + 1] <= SEARCH_TOTAL {
            for j in (i + 2)..input.len() {
                if input[i] + input[i + 1] + input[j] == SEARCH_TOTAL {
                    return Ok(input[i] * input[i + 1] * input[j]);
                }
            }
        }
    }

    Err("Did not find a trio of numbers.".into())
}

fn parse_input() -> impl Iterator<Item = i32> {
    INPUT.lines().map(|i| i.parse::<i32>().unwrap())
}
