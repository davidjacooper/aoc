use std::fs;
use std::env;

fn main()
{
    // Optional filename as a command-line argument
    let file = env::args().nth(1).unwrap_or("input1.txt".to_string());

    let mut sum = 0;

    if let Ok(content) = fs::read_to_string(file.clone())
    {
        for line in content.lines()
        {
            if !line.is_empty()
            {
                let digits: Vec<char> = line.chars().filter(|c: &char| c.is_ascii_digit()).collect();
                let n = digits.first().unwrap().to_digit(10).unwrap() * 10 + digits.last().unwrap().to_digit(10).unwrap();
                println!("N == {}", n);
                sum += n;
            }
        }
        println!("Sum == {}", sum);
    }
    else
    {
        println!("Cannot read '{}'", file);
    }
}
