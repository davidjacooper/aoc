use std::fs;
use std::env;
use std::collections::HashMap;

fn main()
{
    let file = env::args().nth(1).unwrap_or("input1.txt".to_string());
    match fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) => gear_ratios_part2(&content)
    }
}

fn gear_ratios_part2(content: &str)
{
    let mut index: usize = 0;
    let mut asterisks = HashMap::new();
    for (row_n, row) in content.lines().enumerate()
    {
        for (col_n, ch) in row.chars().enumerate()
        {
            if ch == '*'
            {
                for i in -1..=1
                {
                    for j in -1..=1
                    {
                        asterisks.insert((row_n as i32 + i, col_n as i32 + j), index);
                    }
                }
                index += 1;
            }
        }
    }

    let mut sum = 0;
    let mut n: i32 = 0;
    let mut gear_values = vec![-1; asterisks.len()];
    let mut gear_index = None;

    for (row_n, row) in content.lines().enumerate()
    {
        for (col_n, ch) in (row.to_string() + ".").chars().enumerate()
        {
            if let Some(digit) = ch.to_digit(10)
            {
                n = (n * 10) + (digit as i32);
                gear_index = gear_index.or_else(|| asterisks.get(&(row_n as i32, col_n as i32)));
            }
            else
            {
                if let Some(index) = gear_index
                {
                    let prev_n = gear_values[*index];
                    if prev_n == -1
                    {
                        gear_values[*index] = n;
                    }
                    else
                    {
                        println!("Gear ratio: {prev_n} * {n} == {}", n * prev_n);
                        sum += n * prev_n;
                    }
                    gear_index = None;
                }
                n = 0;
            }
        }
    }

    println!("Sum == {}", sum);
}
