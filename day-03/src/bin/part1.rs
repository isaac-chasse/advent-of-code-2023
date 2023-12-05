use std::collections::HashMap;

// Special characters in manual
const SPECIAL_CHARS: [char; 10] = ['@', '#', '$', '%', '&', '*', '-', '=', '+', '/'];

// Gets a grid of text input
fn construct_grid(text: &str) -> Vec<Vec<char>> {
    text.lines()
        .map(|line| line.chars().collect())
        .collect()
}

// Gets the part number and stores a Vec with the positions of each number
fn get_part_numbers_and_positions(grid: &Vec<Vec<char>>) -> HashMap<u32, Vec<(usize, usize)>> {
    let mut part_numbers_map: HashMap<u32, Vec<(usize, usize)>> = HashMap::new();

    for (row_idx, row) in grid.iter().enumerate() {
        let mut current_part_number: Option<u32> = None;
        let mut current_positions: Vec<(usize, usize)> = Vec::new();

        for (col_idx, &cell) in row.iter().enumerate() {
            if cell.is_digit(10) {
                // If a digit is encountered, update the current part number and positions
                let digit = cell.to_digit(10).unwrap();
                current_part_number = Some(current_part_number.unwrap_or(0) * 10 + digit);
                current_positions.push((row_idx, col_idx));
            } else {
                // If a non-digit is encountered, check if there was a current part number
                if let Some(part_number) = current_part_number {
                    let entry = part_numbers_map.entry(part_number).or_insert(vec![]);
                    entry.extend_from_slice(&current_positions);
                }

                // Reset for the next sequence
                current_part_number = None;
                current_positions.clear();
            }
        }

        // Check if there is a current part number after the last symbol or period
        if let Some(part_number) = current_part_number {
            let entry = part_numbers_map.entry(part_number).or_insert(vec![]);
            entry.extend_from_slice(&current_positions);
        }
    }

    part_numbers_map
}

fn get_valid_neighbors(
    part_numbers_map: &HashMap<u32, Vec<(usize, usize)>>,
    position: (usize, usize),
) -> HashMap<u32, Vec<(usize, usize)>> {
    let mut valid_positions_map: HashMap<u32, Vec<(usize, usize)>> = HashMap::new();
    let (row, col) = position;

    // Set neighbors in i, j space
    let neighbors: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    for (part_number, positions) in part_numbers_map.iter() {
        for position_set in positions {
            for (i, j) in neighbors.iter() {
                let neighbor_position = (
                    position_set.0 as isize + i,
                    position_set.1 as isize + j,
                );
                if neighbor_position == (row as isize, col as isize) {
                    let entry = valid_positions_map.entry(*part_number).or_insert(vec![]);
                    entry.push(*position_set);
                }
            }
        }
    }

    valid_positions_map
}

fn get_special_char_positions(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut positions_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for special_char in SPECIAL_CHARS {
        positions_map.insert(special_char, Vec::new());
    }

    for (row, line) in grid.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if SPECIAL_CHARS.contains(&c) {
                if let Some(positions) = positions_map.get_mut(&c) {
                    positions.push((row, col));
                }
            }
        }
    }

    positions_map.values().flat_map(|positions| positions.iter().cloned()).collect()
}

fn main() {
    let input = include_str!("./input.txt");
    let input_grid = construct_grid(input);

    // Get part numbers as a HashMap of positions
    let part_numbers_map = get_part_numbers_and_positions(&input_grid);
    
    // Get the special character positions 
    let special_positions = get_special_char_positions(&input_grid);
    
    let mut part_numbers: Vec<u32> = Vec::new();

    for special_position in special_positions {
        let valid_part_positions_map = get_valid_neighbors(&part_numbers_map, special_position);
        let valid_parts: Vec<u32> = valid_part_positions_map.keys().cloned().collect();
        for part in valid_parts {
            part_numbers.push(part);
        }
    }
    
    let part_sum: u32 = part_numbers.iter().sum();
    println!("Sum of valid part numbers: {}", part_sum);
}
