use std::collections::HashMap;

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

// Function to update cumulative map based on winners
fn update_cumulative_map(cumulative_map: &mut HashMap<usize, u32>, matches_len: usize, current_index: usize) {
    let current_value = cumulative_map.get(&current_index).cloned().unwrap_or(1);

    let next_index = current_index + 1;
    for idx in next_index..next_index+matches_len {
        let future_entry = cumulative_map.entry(idx).or_insert(0);
        *future_entry += current_value;
    }
}

fn main() {
    // Read data 
    let input = include_str!("./input.txt");

    let lines: Vec<&str> = input.lines().collect();
    let card_vec: Vec<(Vec<u32>, Vec<u32>)> = lines.iter().map(|line| parse_card_line(line)).collect();

    let mut cumulative_map: HashMap<usize, u32> = HashMap::new();

    // Populate hashmap 
    for (idx, _) in card_vec.iter().enumerate() {
        let current_index = idx + 1; // 1-based 

        cumulative_map.entry(current_index).or_insert(1);
    }

    for (idx, (left, right)) in card_vec.iter().enumerate() {
        let current_index = idx + 1; // 1-indexed

        let matches: Vec<u32> = vec_intersection(left, right);
        let n_matches = matches.len();

        // Only tally if there is more than one winner
        if !matches.is_empty() {
            update_cumulative_map(&mut cumulative_map, n_matches, current_index);
        }
    }

    let total_score: u32 = cumulative_map.values().sum();
    println!("Total cards: {}", total_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test_cumulative() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let lines: Vec<&str> = input.lines().collect();
        let card_vec: Vec<(Vec<u32>, Vec<u32>)> = lines.iter().map(|line| parse_card_line(line)).collect();

        let mut cumulative_map: HashMap<usize, u32> = HashMap::new();

        // Populate hashmap 
        for (idx, _) in card_vec.iter().enumerate() {
            let current_index = idx + 1; // 1-based 

            cumulative_map.entry(current_index).or_insert(1);
        }

        for (idx, (left, right)) in card_vec.iter().enumerate() {
            let current_index = idx + 1; // 1-indexed

            let matches: Vec<u32> = vec_intersection(left, right);
            let n_matches = matches.len();

            dbg!(&current_index, &matches);

            // Only tally if there is more than one winner
            if !matches.is_empty() {
                dbg!(&matches.len());
                update_cumulative_map(&mut cumulative_map, n_matches, current_index);
            }
        }

        dbg!(&cumulative_map);

        let total_score: u32 = cumulative_map.values().sum();
        assert_eq!(total_score, 30);
    }
}
