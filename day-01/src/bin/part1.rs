fn main() {
    let input = include_str!("./input1.txt");
    let solution = sum_calibration_values(input);

    println!("Sum of a calibration values: {}", solution);
}

fn sum_calibration_values(input: &str) -> u32 {
    let sum_of_calibration_values = input
        .lines()
        .filter_map(|line| extract_first_and_last_digit_as_int(line))
        .sum();

    return sum_of_calibration_values;
}

fn extract_first_and_last_digit_as_int(line: &str) -> Option<u32> {
    let digits: Vec<u32> = line
        .chars()
        .filter(|c| c.is_digit(10))
        .flat_map(|c| c.to_digit(10))
        .collect();

    Some(digits[0] * 10 + digits[digits.len() - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_numbers() {
        let test_case = "laksr9juh4gbejb";
        let result = extract_first_and_last_digit_as_int(test_case).unwrap();
        assert_eq!(result, 94);
    }

    #[test]
    fn test_single_number() {
        let test_case = "laksr9juhgbejb";
        let result = extract_first_and_last_digit_as_int(test_case).unwrap();
        assert_eq!(result, 99);
    }

    #[test]
    fn test_n_numbers() {
        let test_case = "laksr349juh49gbe73413jb";
        let result = extract_first_and_last_digit_as_int(test_case).unwrap();
        assert_eq!(result, 33);
    }
}
