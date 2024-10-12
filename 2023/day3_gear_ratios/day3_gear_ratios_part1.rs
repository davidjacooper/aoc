use std::fs;
use std::env;
use std::collections::HashSet;

fn main()
{
    let file = env::args().nth(1).unwrap_or("input1.txt".to_string());

    if let Ok(content) = fs::read_to_string(file.clone())
    {
        let mut symbols = HashSet::new();
        for (row_n, row) in content.lines().enumerate()
        {
            for (col_n, ch) in row.chars().enumerate()
            {
                if !ch.is_digit(10) && ch != '.'
                {
                    symbols.insert((row_n + 1, col_n + 1));
                }
            }
        }

        let mut sum = 0;
        let mut n: i32 = -1;
        let mut number = false;
        let mut part_number = false;

        for (row_n, row) in content.lines().enumerate()
        {
            for (col_n, ch) in (row.to_string() + ".").chars().enumerate()
            {
                if let Some(digit) = ch.to_digit(10)
                {
                    if !number
                    {
                        number = true;
                        n = 0;
                        part_number = symbols.contains(&(row_n,     col_n)) ||
                                      symbols.contains(&(row_n + 1, col_n)) ||
                                      symbols.contains(&(row_n + 2, col_n));
                    }
                    n = (n * 10) + (digit as i32);
                    part_number |= symbols.contains(&(row_n,     col_n + 1)) ||
                                   symbols.contains(&(row_n + 2, col_n + 1));
                }
                else if number
                {
                    // First non-numeric char after a number; we guarantee there will be at least one (+ "." above).

                    part_number |= symbols.contains(&(row_n,     col_n + 1)) ||
                                   symbols.contains(&(row_n + 1, col_n + 1)) ||
                                   symbols.contains(&(row_n + 2, col_n + 1));
                    if part_number
                    {
                        sum += n;
                        println!("{n} - part number");
                    }
                    else
                    {
                        println!("{n} - ignore");
                    }
                    number = false;
                    part_number = false;
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
