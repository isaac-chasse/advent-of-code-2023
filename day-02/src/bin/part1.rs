use std::str::FromStr;

#[derive(Debug)]
struct Game {
    game_number: u32,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    blue: u32,
    red: u32,
    green: u32,
}

impl Default for Round {
    fn default() -> Self {
        Round { blue: 0, red: 0, green: 0 }
    }
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(": ");
        let game_number = iter.next().ok_or("Invalid input")?.trim().parse().map_err(|_| "Invalid game number")?;
        let rounds_str = iter.next().ok_or("Invalid input")?.trim();
        
        let rounds = rounds_str.split("; ")
            .map(|round| {
                let mut round_iter = round.split(", ");

                let mut blue = 0;
                let mut red = 0;
                let mut green = 0;

                while let Some(part) = round_iter.next() {
                    let count = part.split_whitespace().next().unwrap_or("0").parse().map_err(|_| "Invalid count")?;
                    if part.contains("blue") {
                        blue = count;
                    } else if part.contains("red") {
                        red = count;
                    } else if part.contains("green") {
                        green = count;
                    }
                }

                Ok(Round { blue, red, green })
            })
            .collect::<Result<Vec<Round>, &'static str>>()?;

        Ok(Game { game_number, rounds })
    }
}

fn get_valid_games(input: &str, constraint: Round) -> Vec<Game> {
    let games: Vec<Game> = input.split("Game ").skip(1)
        .filter_map(|game_str| game_str.parse::<Game>().ok())
        .filter(|game| game.rounds.iter().all(|round| round.red <= constraint.red && round.blue <= constraint.blue && round.green <= constraint.green))
        .collect();

    games
}

fn main() {
    let input = include_str!("./input.txt");

    let constraint: Round = Round { red: 12, blue: 14, green: 13 };

    let games: Vec<Game> = get_valid_games(input, constraint);

    let game_sum: u32 = games
        .iter()
        .map(|game| game.game_number)
        .sum();

    println!("{:?}", game_sum);
}

