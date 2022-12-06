use std::fs;

fn main() {
    println!("Hello, world!");

    let input = fs::read_to_string("input").unwrap();

    let lines = input.lines().collect::<Vec<&str>>();

    let mut scores: Vec<u32> = vec![];

    let mut cheat_scores: Vec<u32> = vec![];

    for line in lines {
        let play = Play::new(line);

        scores.push(play.get_score());

        cheat_scores.push(play.get_cheat_score());
    }

    let max_score = scores.iter().sum::<u32>();

    // Part 1: Total Score
    println!("Max score: {}", max_score);

    // Part 2: Cheat score

    let max_cheat_score = cheat_scores.iter().sum::<u32>();
    println!("Cheat score: {}", max_cheat_score);
}

struct Item<'a> {
    name: (&'a str, &'a str),
    value: u32,
    wins: u32,
    cheat: u32,
}

impl Item<'_> {
    fn from_input(input: &str) -> Self {
        if input == ROCK.name.1 || input == ROCK.name.0 {
            ROCK
        } else if input == PAPER.name.1 || input == PAPER.name.0 {
            PAPER
        } else if input == SCISSORS.name.1 || input == SCISSORS.name.0 {
            SCISSORS
        } else {
            panic!("Invalid input: {}", input);
        }
    }

    fn get_loser(loses_to: u32) -> Self {
        if loses_to == ROCK.value {
            return ROCK;
        }
        if loses_to == PAPER.value {
            return PAPER;
        }
        return SCISSORS;
    }

    fn get_winner(wins_to: u32) -> Self {
        if wins_to == ROCK.wins {
            return ROCK;
        }
        if wins_to == PAPER.wins {
            return PAPER;
        }
        return SCISSORS;
    }
}

const ROCK: Item = Item {
    name: ("X", "A"),
    value: 1,
    wins: 3,
    cheat: 0,
};

const PAPER: Item = Item {
    name: ("Y", "B"),
    value: 2,
    wins: 1,
    cheat: 1,
};

const SCISSORS: Item = Item {
    name: ("Z", "C"),
    value: 3,
    wins: 2,
    cheat: 2,
};

struct Play<'a> {
    player: Item<'a>,
    enemy: Item<'a>,
}

impl Play<'_> {
    fn new(line: &str) -> Play<'_> {
        let (enemy_input, player_input) = line.split_at(1);

        let player = Item::from_input(player_input.trim());

        let enemy = Item::from_input(enemy_input.trim());

        Play { player, enemy }
    }

    pub fn get_score(&self) -> u32 {
        if self.player.wins == self.enemy.value {
            return self.player.value + 6;
        }
        if self.enemy.wins == self.player.value {
            return self.player.value;
        }

        return self.player.value + 3;
    }

    pub fn get_cheat_score(&self) -> u32 {
        println!(
            "Player cheat: {}, Enemy pick is {}",
            self.player.cheat, self.enemy.name.1
        );
        if self.player.cheat == 0 {
            let loses_to = Item::get_loser(self.enemy.wins);
            println!("So I LOSE with: {}", loses_to.name.1);
            return loses_to.value;
        }

        if self.player.cheat == 1 {
            println!("I DRAW with: {}", self.enemy.name.1);
            return self.enemy.value + 3;
        }

        let wins_to = Item::get_winner(self.enemy.value);
        println!("So i WIN with: {}", wins_to.name.1);
        return wins_to.value + 6;
    }
}
