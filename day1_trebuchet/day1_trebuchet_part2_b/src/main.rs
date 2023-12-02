use std::fs;
use std::env;
use regex::Regex;

const WORDS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn to_digit(s: &str) -> i32
{
    return WORDS.iter().position(|test_s| test_s == &s)
                       .map(|i| (i as i32) + 1)
                       .unwrap_or_else(|| (s.bytes().nth(0).unwrap() - b'0') as i32)
}

fn main()
{
    // Optional filename as a command-line argument
    let file = env::args().nth(1).unwrap_or("input2.txt".to_string());

    if let Ok(content) = fs::read_to_string(file.clone())
    {
        let regex_str = format!("[1-9]|{}", WORDS.join("|"));
        let first_regex = Regex::new(&regex_str).unwrap();
        let last_regex  = Regex::new(&format!(".*({regex_str})")).unwrap();
        let mut sum = 0;

        for line in content.lines()
        {
            if !line.is_empty()
            {
                let first = to_digit(first_regex.find(line).unwrap().as_str());
                let last = to_digit(&last_regex.captures(line).unwrap()[1]);

                println!("n={first}{last}");
                sum += first * 10 + last;
            }
        }
        println!("Sum == {}", sum);
    }
    else
    {
        println!("Cannot read '{}'", file);
    }
}
