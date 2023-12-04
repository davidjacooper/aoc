use std::collections::HashSet;

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input1.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) => scratchcards_part1(&content)
    }
}

fn scratchcards_part1(content: &str)
{
    let mut sum = 0;
    for line in content.lines()
    {
        if !line.is_empty()
        {
            let (winning_str, have_str) = line[(line.find(':').unwrap() + 2)..].split_once(" | ").unwrap();
            let n_common = as_set(winning_str).intersection(&as_set(have_str)).count();
            let points = (1 << n_common) >> 1;

            sum += points;
            println!("{n_common} in common, #points = {points}");
        }
    }
    println!("Sum == {}", sum);
}

fn as_set(string: &str) -> HashSet<i32>
{
    return string.split_whitespace().map(|s| s.parse().unwrap()).collect();
}
