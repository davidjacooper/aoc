use std::fs;
use std::env;
use std::collections::HashSet;

fn main()
{
    let file = env::args().nth(1).unwrap_or("input1.txt".to_string());
    match fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) => gear_ratios_part1(&content)
    }
}

fn gear_ratios_part1(content: &str)
{
    let mut symbols = HashSet::new();
    for (row_n, row) in content.lines().enumerate()
    {
        for (col_n, ch) in row.chars().enumerate()
        {
            if !ch.is_digit(10) && ch != '.'
            {
                for i in -1..=1
                {
                    for j in -1..=1
                    {
                        symbols.insert((row_n as i32 + i, col_n as i32 + j));
                    }
                }
            }
        }
    }

    let mut sum = 0;
    let mut n: i32 = 0;
    let mut part_number = false;

    for (row_n, row) in content.lines().enumerate()
    {
        for (col_n, ch) in (row.to_string() + ".").chars().enumerate()
        {
            if let Some(digit) = ch.to_digit(10)
            {
                n = (n * 10) + (digit as i32);
                part_number |= symbols.contains(&(row_n as i32, col_n as i32));
            }
            else
            {
                if part_number
                {
                    sum += n;
                    println!("{n} - part number");
                    part_number = false;
                }
                n = 0;
            }
        }
    }

    println!("Sum == {}", sum);
}
