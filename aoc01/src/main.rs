use std::error::Error;

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let p1 = part_1().unwrap();

    println!("Part 1: {}", p1);
}

fn part_1() -> Result<i32, Box<dyn Error>> {
    let mut input_nums = INPUT
        .lines()
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    input_nums.sort();

    let search_total = 2020;

    for i in &input_nums {
        let search_current = search_total - i;
        let search_result = input_nums.binary_search(&search_current);

        if let Ok(idx) = search_result {
            let j = input_nums[idx];
            return Ok(i * j);
        }
    }

    Err("Did not find a pair of numbers.".into())
}
