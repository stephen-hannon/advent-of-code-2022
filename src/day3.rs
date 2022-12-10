use std::{collections::HashSet};

type Rucksack = Vec<char>;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect()
}

fn get_priority(chr: char) -> Option<u32> {
    match chr {
        'a'..='z' => Some((chr as u8 - 'a' as u8 + 1) as u32),
        'A'..='Z' => Some((chr as u8 - 'A' as u8 + 27) as u32),
        _ => None
    }
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<Rucksack>) -> u32 {
    input
        .iter()
        .map(|line| {
            let midpoint = line.len() / 2;
            let slice1 = &line[..midpoint];
            let slice2 = &line[midpoint..];
            let compartment1: HashSet<&char> = slice1.iter().collect();
            let compartment2: HashSet<&char> = slice2.iter().collect();

            let intersection = compartment1.intersection(&compartment2);
            let common_chars = Vec::from_iter(intersection);
            let common_char = **common_chars[0];
            get_priority(common_char).unwrap()
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<Rucksack>) -> u32 {
    input
        .iter()
        .map(|line| {
            HashSet::from_iter(line.iter().cloned())
        })
        .collect::<Vec<HashSet<char>>>()
        .chunks(3)
        .map(|group| {
            let intersection = group[0]
                .intersection(&group[1])
                .find(|it| group[2].contains(it));

            let common_chars = Vec::from_iter(intersection);
            let common_char = *common_chars[0];
            get_priority(common_char).unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input_str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let input = input_generator(input_str);
        assert_eq!(solve_part1(&input), 157);
        assert_eq!(solve_part2(&input), 70);
    }
}