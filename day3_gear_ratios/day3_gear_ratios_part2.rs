use std::fs;
use std::env;
use std::collections::HashMap;

fn main()
{
    let file = env::args().nth(1).unwrap_or("input1.txt".to_string());

    if let Ok(content) = fs::read_to_string(file.clone())
    {
        let mut index: usize = 0;
        let mut asterisks = HashMap::new();
        for (row_n, row) in content.lines().enumerate()
        {
            for (col_n, ch) in row.chars().enumerate()
            {
                if ch == '*'
                {
                    for i in row_n..(row_n + 3)
                    {
                        for j in col_n..(col_n + 3)
                        {
                            asterisks.insert((i, j), index);
                        }
                    }
                    index += 1;
                }
            }
        }

        let mut sum = 0;
        let mut n: i32 = -1;
        let mut number = false;
        let mut start_col_n = 0;
        let mut gear_values = vec![-1; asterisks.len()];

        for (row_n, row) in content.lines().enumerate()
        {
            for (col_n, ch) in (row.to_string() + ".").chars().enumerate()
            {
                if let Some(digit) = ch.to_digit(10)
                {
                    if !number
                    {
                        number = true;
                        start_col_n = col_n;
                        n = 0;
                    }
                    n = (n * 10) + (digit as i32);
                }
                else if number
                {
                    // First non-numeric char after a number; we guarantee there will be at least one (+ "." above).

                    for j in start_col_n..col_n
                    {
                        if let Some(index) = asterisks.get(&(row_n + 1, j + 1))
                        {
                            let prev_n = gear_values[*index];
                            if prev_n == -1
                            {
                                gear_values[*index] = n;
                            }
                            else
                            {
                                println!("Gear ratio: {n} * {prev_n} == {}", n * prev_n);
                                sum += n * prev_n;
                            }
                            break;
                        }
                    }
                    number = false;
                }
            }
        }

        println!("Sum == {}", sum);
    }
    else
    {
        println!("Cannot read '{}'", file);
    }
}

