type ElfCalories = Vec<u32>;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<ElfCalories> {
    input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|d| d.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<ElfCalories>) -> u32 {
    input
        .iter()
        .map(|items| {
            items.iter().sum()
        })
        .max().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<ElfCalories>) -> u32 {
    let mut elves = input
        .iter()
        .map(|items| {
            items.iter().sum()
        })
        .collect::<Vec<u32>>();
    elves.sort();
    elves.reverse();
    elves[0..3].iter().sum()
}