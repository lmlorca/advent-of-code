fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut elf_bags: Vec<ElfBag> = input
        .trim()
        .split("\n\n")
        .map(|bag: &str| -> ElfBag {
            let snacks: Vec<u32> = bag
                .split("\n")
                .map(|snack: &str| -> u32 { snack.parse().unwrap() })
                .collect();

            let calories = snacks.iter().sum();

            ElfBag { snacks, calories }
        })
        .collect();

    // Part 1
    let max_calories = elf_bags.iter().map(|bag| bag.calories).max().unwrap();

    println!("Max calories: {}", max_calories);

    // Part 2 (top 3 total)
    elf_bags.sort_by_key(|bag| bag.calories);

    let mut total_top_3 = 0;

    for bag in elf_bags.iter().rev().take(3) {
        total_top_3 += bag.calories;
        println!("{:?}", bag);
    }

    println!("Total top 3: {}", total_top_3);
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ElfBag {
    snacks: Vec<u32>,
    calories: u32,
}
