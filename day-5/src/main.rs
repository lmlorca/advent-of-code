use std::fs;

const PART: u8 = 2;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let (initial_state, instructions) = input.split_once("\n\n").unwrap();

    let initial_state = initial_state.lines().collect::<Vec<&str>>();

    // Number of cols
    let cols_num = initial_state
        .last()
        .unwrap()
        .split(" ")
        .filter(|el| el.len() > 0)
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut initial_state = initial_state
        .iter()
        .map(|line| {
            let chars = line
                .chars()
                .enumerate()
                .filter(|(idx, _)| {
                    if (idx + 1).rem_euclid(4) == 0 && idx != &0 {
                        return false;
                    } else {
                        return true;
                    }
                })
                .map(|(_, c)| c)
                .collect::<Vec<char>>();

            let mut out = vec![];

            for (idx, c) in chars.iter().enumerate() {
                if (idx + 1).rem_euclid(3) == 0 {
                    if c == &']' {
                        out.push(chars[idx - 1]);
                    } else {
                        out.push('-');
                    }
                }
            }

            out
        })
        .collect::<Vec<Vec<char>>>();

    initial_state.remove(initial_state.len() - 1);

    let mut state: Vec<Vec<char>> = vec![];

    for col_idx in 1..cols_num + 1 {
        let mut col = vec![];
        for line in initial_state.iter() {
            if line[col_idx - 1] != '-' {
                col.push(line[col_idx - 1]);
            }
        }
        if PART == 1 {
            col.reverse();
        }
        state.push(col);
    }

    let parse_instructions = |instruction_line: &str| -> Instruction {
        let instruction = instruction_line.split(" ").collect::<Vec<&str>>();

        Instruction {
            mov: instruction[1].parse::<usize>().unwrap(),
            fro: instruction[3].parse::<usize>().unwrap(),
            ito: instruction[5].parse::<usize>().unwrap(),
        }
    };

    let instructions = instructions
        .lines()
        .map(parse_instructions)
        .collect::<Vec<Instruction>>();

    for instruction in instructions.iter() {
        if PART == 1 {
            // part_1
            for _ in 0..instruction.mov {
                let col = &mut state[instruction.fro - 1].pop().unwrap();
                state[instruction.ito - 1].push(*col)
            }
        } else {
            // part_2
            let from = &mut state[instruction.fro - 1].to_owned();
            let mut to = &mut state[instruction.ito - 1];

            let (taken, remainder) = from.split_at(instruction.mov);

            let mut taken = taken.to_vec().to_owned();

            let remainder = remainder.to_vec().to_owned();

            taken.append(&mut to);

            state[instruction.fro - 1] = remainder;
            state[instruction.ito - 1] = taken;
        }
    }

    let mut result = String::new();

    for col_idx in 1..cols_num + 1 {
        // let col = state_map.get(&col_idx).unwrap();
        let col = &state[col_idx - 1];

        if PART == 1 {
            result.push(*col.last().unwrap());
        } else {
            result.push(*col.first().unwrap());
        }
    }

    println!("Part {}: {:?}", PART, result);
}

#[derive(Debug)]
struct Instruction {
    mov: usize,
    fro: usize,
    ito: usize,
}
