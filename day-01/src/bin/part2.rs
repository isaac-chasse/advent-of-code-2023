fn main() {
    let input = include_str!("./input1.txt");
    let solution = sum_calibration_values(input);

    println!("Sum of a calibration values: {}", solution);
}

const NUMBERS: [(&str, u32); 20] = [
    ("zero", 0), 
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn parse_all_digits_from_str(line: &str) -> u32 {
    let mut digits = (0..line.len()).filter_map(|i| {
        NUMBERS
            .iter()
            .find(|(number, _val)| line[i..].starts_with(number))
            .map(|(_number, val)| val)
    });

    let first = digits.next().unwrap();
    first * 10 + digits.last().unwrap_or(first)
}


fn sum_calibration_values(input: &str) -> u32 {
    let sum_of_calibration_values = input
        .lines()
        .map(parse_all_digits_from_str)
        .sum();

    return sum_of_calibration_values;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let solution = sum_calibration_values(input);
        assert_eq!(solution, 281);
    }
}   
