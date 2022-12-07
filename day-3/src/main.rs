use std::fs;

fn main() {
    let char_to_value = |char: char| -> usize {
        let chars = "0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        return chars.find(char).unwrap();
    };

    let input = fs::read_to_string("input").unwrap();

    let part1: usize = input
        .trim()
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            (
                left.chars().map(char_to_value).collect::<Vec<usize>>(),
                right.chars().map(char_to_value).collect::<Vec<usize>>(),
            )
        })
        .map(|(left, right)| {
            let in_both = left
                .into_iter()
                .filter(|x| right.contains(x))
                .collect::<Vec<usize>>();

            let mut uniq: Vec<usize> = vec![];

            for value in in_both.iter() {
                if !uniq.contains(value) {
                    uniq.push(*value);
                }
            }

            uniq.iter().sum::<usize>()
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum();

    let val_vect = input
        .trim()
        .lines()
        .map(|line| line.chars().map(char_to_value).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    let part2 = val_vect
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            let mut group: Vec<&Vec<usize>> = vec![];

            if idx % 3 == 0 {
                group.push(&val_vect[idx]);
                group.push(&val_vect[idx + 1]);
                group.push(&val_vect[idx + 2]);
            }
            group
        })
        .filter(|item| item.len() == 3)
        .map(|group| {
            let mut uniq = vec![];

            for value in group[0].iter() {
                if group[1].contains(value) && group[2].contains(value) && !uniq.contains(value) {
                    uniq.push(*value);
                }
            }

            uniq.iter().sum::<usize>()
        })
        .sum::<usize>();

    println!("{}", part1);
    println!("{}", part2);
}
