fn vec_intersection(v1: &[u32], v2: &[u32]) -> Vec<u32> {
    v1.iter()
        .filter(|&e1| v2.iter().any(|&e2| *e1 == e2))
        .cloned()
        .collect()
}

fn parse_card_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    let parts: Vec<&str> = line.split('|').collect();

    let left_vec: Vec<u32> = parts[0].trim().split_whitespace().filter_map(|s| s.parse().ok()).collect();
    let right_vec: Vec<u32> = parts[1].trim().split_whitespace().filter_map(|s| s.parse().ok()).collect();

    (left_vec, right_vec)
}

fn main() {
    // Read data 
    let input = include_str!("./input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let card_vec: Vec<(Vec<u32>, Vec<u32>)> = lines.iter().map(|line| parse_card_line(line)).collect();

    let mut points: Vec<u32> = Vec::new();

    for (left, right) in card_vec.iter() {
        let winners = vec_intersection(left, right);
        if winners.len() > 0 {
            let base: u32 = 2;
            let score: u32 = base.pow(winners.len() as u32 - 1);
            points.push(score);
        } 
    }

    let total_score: u32 = points.iter().sum();
    println!("Total: {}", total_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let lines: Vec<&str> = input.lines().collect();
        let card_vec: Vec<(Vec<u32>, Vec<u32>)> = lines.iter().map(|line| parse_card_line(line)).collect();

        let mut points: Vec<u32> = Vec::new();

        for (left, right) in card_vec.iter() {
            let winners = vec_intersection(left, right);
            if winners.len() > 0 {
                let base: u32 = 2;
                let score: u32 = base.pow(winners.len() as u32 - 1);
                points.push(score);
            } 
        }

        let total_score: u32 = points.iter().sum();
        assert_eq!(total_score, 13);

    }
}
