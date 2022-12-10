use std::fs;

fn main() {
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

struct Hand<'a> {
    name: (&'a str, &'a str),
    value: u32,
    wins: u32,
    cheat: u8,
}

impl Hand<'_> {
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

    fn get_loser(winner: u32) -> Self {
        if winner == ROCK.value {
            return ROCK;
        }
        if winner == PAPER.value {
            return PAPER;
        }
        return SCISSORS;
    }

    fn get_winner(value: u32) -> Self {
        if value == ROCK.wins {
            return ROCK;
        }
        if value == PAPER.wins {
            return PAPER;
        }
        return SCISSORS;
    }
}

const LOSE: u8 = 0;
const DRAW: u8 = 1;
const WIN: u8 = 2;

const ROCK: Hand = Hand {
    name: ("X", "A"),
    value: 1,
    wins: 3,
    cheat: LOSE,
};

const PAPER: Hand = Hand {
    name: ("Y", "B"),
    value: 2,
    wins: 1,
    cheat: DRAW,
};

const SCISSORS: Hand = Hand {
    name: ("Z", "C"),
    value: 3,
    wins: 2,
    cheat: WIN,
};

struct Play<'a> {
    player: Hand<'a>,
    enemy: Hand<'a>,
}

impl Play<'_> {
    fn new(line: &str) -> Play<'_> {
        let (enemy_input, player_input) = line.split_at(1);

        let player = Hand::from_input(player_input.trim());

        let enemy = Hand::from_input(enemy_input.trim());

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
        if self.player.cheat == LOSE {
            let loses_to = Hand::get_loser(self.enemy.wins);
            return loses_to.value;
        }

        if self.player.cheat == DRAW {
            return self.enemy.value + 3;
        }

        let wins_to = Hand::get_winner(self.enemy.value);
        return wins_to.value + 6;
    }
}
