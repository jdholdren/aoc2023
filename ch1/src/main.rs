use std::fs::File;
use std::io::BufRead;

fn main() {
    // Open the file
    let f = File::open("./ch1/input.txt").unwrap();

    // Process line by line
    let mut sum = 0u32;
    for line in std::io::BufReader::new(f).lines() {
        sum += process_line(line.unwrap());
    }

    print!("sum: {}", sum)
}

// Figures out the value of a line.
fn process_line(s: String) -> u32 {
    let digits = digits(s);

    format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
        .parse()
        .unwrap()
}

// Finds all the digits in a string
fn digits(s: String) -> Vec<u32> {
    let mut nums = vec![];

    // Keep a moving window over the characters. This gets reset every new digit that
    // is found, either in alphabetical or numeric.
    let mut window = String::new();
    for c in s.chars() {
        if c.is_numeric() {
            // A numeric character is automatically a digit
            nums.push(c.to_digit(10).unwrap());
            continue;
        }

        // Add to the window and see if it has a numerical word at the end.
        window.push(c);
        if let Some(digit) = spelled_out(&window) {
            nums.push(digit);
        }
    }

    nums
}

// Checks if there's a spelled out digit in the string.
fn spelled_out(s: &str) -> Option<u32> {
    let digit_words: Vec<(&str, u32)> = vec![
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
    ];

    for (word, digit) in digit_words {
        if !s.ends_with(word) {
            continue;
        }

        return Some(digit);
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn spelled_out_present() {
        let res = spelled_out("abcone");
        assert_eq!(Some(1), res);
    }

    #[test]
    fn find_digits() {
        let digits = digits(String::from("6bjztkxhsixkgnkroneightht"));
        assert_eq!(vec![6, 6, 1, 8], digits);
    }
}
