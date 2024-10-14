use std::time::Instant;
// use std::collections::HashSet;
use std::iter::FromIterator;

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("../inputs/day03_input.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            if let Err(err) = solve(&content)
            {
                println!("ERROR: {}", err);
            }
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}


fn solve(content: &str) -> Result<(),&str>
{
    let values: Vec<i32> =
        content
        .trim_end()
        .lines()
        .map(|line|
            line
            .chars()
            .rev()
            .enumerate()
            .map(|(i, ch)| if ch == '1' { 1 << i } else { 0 })
            .sum())
        .collect();

    let max_bit = values.iter().max().unwrap().ilog2();
    let mut oxygen_values: Vec<i32> = Vec::from_iter(values.clone());
    let mut co2_values:    Vec<i32> = Vec::from_iter(values.clone());

    for bit in (0..=max_bit).rev()
    {
        if oxygen_values.len() > 1
        {
            let ones = oxygen_values
                .iter()
                .map(|i| ((i >> bit) & 1) * 2 - 1)
                .sum::<i32>() >= 0;

            oxygen_values.retain(|i| (i >> bit) & 1 == if ones {1} else {0});
        }
        if co2_values.len() > 1
        {
            let ones = co2_values
                .iter()
                .map(|i| ((i >> bit) & 1) * 2 - 1)
                .sum::<i32>() < 0;

            co2_values.retain(|i| (i >> bit) & 1 == if ones {1} else {0});
        }
    }

    let life_support_rating = oxygen_values[0] * co2_values[0];

    println!("max_bit={}, oxygen_values=={:?}, co2_values=={:?}, life_support_rating=={}", max_bit, oxygen_values, co2_values, life_support_rating);
    Ok(())
}
