use std::fs;
use std::env;
use std::cmp;

fn main()
{
    // Optional filename as a command-line argument
    let file = env::args().nth(1).unwrap_or("input2.txt".to_string());

    let mut sum = 0;

    if let Ok(content) = fs::read_to_string(file.clone())
    {
        for line in content.lines()
        {
            if !line.is_empty()
            {
                let mut first_index = line.len() as i32;
                let mut last_index = -1;
                let mut first_n = -1;
                let mut last_n = -1;
                for (n, digit, word) in [//(0, "0", "\x00"),
                                         (1, "1", "one"),
                                         (2, "2", "two"),
                                         (3, "3", "three"),
                                         (4, "4", "four"),
                                         (5, "5", "five"),
                                         (6, "6", "six"),
                                         (7, "7", "seven"),
                                         (8, "8", "eight"),
                                         (9, "9", "nine")
                                        ]
                {
                    let i = cmp::min(line.find(digit).map(|n| n as i32).unwrap_or(first_index),
                                     line.find(word).map(|n| n as i32).unwrap_or(first_index));
                    if first_index > i
                    {
                        first_index = i;
                        first_n = n;
                    }

                    let j = cmp::max(line.rfind(digit).map(|n| n as i32).unwrap_or(last_index),
                                     line.rfind(word).map(|n| n as i32).unwrap_or(last_index));
                    if last_index < j
                    {
                        last_index = j;
                        last_n = n;
                    }
                }

                let line_n = first_n * 10 + last_n;
                println!("N == {}", line_n);
                sum += line_n;
            }
        }
        println!("Sum == {}", sum);
    }
    else
    {
        println!("Cannot read '{}'", file);
    }
}
