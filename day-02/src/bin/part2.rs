use std::{str::FromStr, process::id};

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

fn get_games(input: &str) -> Vec<Game> {
    let games: Vec<Game> = input.split("Game ").skip(1)
        .filter_map(|game_str| game_str.parse::<Game>().ok())
        .collect();

    games
}

fn main() {
    let input = include_str!("./input.txt");
    let games: Vec<Game> = get_games(input);

    let sum_products: u32 = games.into_iter().map(|game: Game| {
        // Get smallest value from each round 
        let fewest_possible_red = game.rounds.iter().map(|round| round.red).max().unwrap();
        let fewest_possible_blue = game.rounds.iter().map(|round| round.blue).max().unwrap();
        let fewest_possible_green = game.rounds.iter().map(|round| round.green).max().unwrap();

        // Calculate the product if there is at least 1 
        let product_red = if fewest_possible_red > 0 { fewest_possible_red } else { 1 };
        let product_blue = if fewest_possible_blue > 0 { fewest_possible_blue } else { 1 };
        let product_green = if fewest_possible_green > 0 { fewest_possible_green } else { 1 };
        
        // Return the product 
        product_red * product_blue * product_green
    }).sum();

    println!("Sum of color products: {}", sum_products);
}

