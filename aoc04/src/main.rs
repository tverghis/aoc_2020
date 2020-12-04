use passport::Passport;

const INPUT: &'static str = include_str!("../input.txt");

mod passport;

fn main() {
    let passports = INPUT
        .split("\n\n")
        .map(|s| s.lines().collect::<Vec<_>>())
        .map(Passport::from)
        .collect::<Vec<_>>();

    let p1 = part_1(&passports);
    println!("Part 1: {} valid passports", p1);
}

fn part_1(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.has_all_req_fields()).count()
}
