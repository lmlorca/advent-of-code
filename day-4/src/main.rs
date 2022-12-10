use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    fn parse_line(line: &str) -> (u32, u32, u32, u32) {
        let pair = line.split(",").collect::<Vec<&str>>();

        let s1 = pair[0];
        let s2 = pair.last().unwrap();

        let vec1 = s1.split("-").collect::<Vec<&str>>();
        let vec2 = s2.split("-").collect::<Vec<&str>>();

        let start_a = vec1[0].parse::<u32>().unwrap();
        let end_a = vec1.last().unwrap().parse::<u32>().unwrap();

        let start_b = vec2[0].parse::<u32>().unwrap();
        let end_b = vec2.last().unwrap().parse::<u32>().unwrap();

        (start_a, end_a, start_b, end_b)
    }

    let full_overlap = |line: &&str| {
        let (start_a, end_a, start_b, end_b) = parse_line(line);

        if (start_a >= start_b && end_a <= end_b) || (start_b >= start_a && end_b <= end_a) {
            return true;
        }

        return false;
    };

    let partial_overlap = |line: &&str| {
        let (start_a, end_a, start_b, end_b) = parse_line(line);

        if start_a <= end_b && start_b <= end_a {
            return true;
        }

        return false;
    };

    let part_1 = input.lines().filter(full_overlap).count();
    let part_2 = input.lines().filter(partial_overlap).count();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
