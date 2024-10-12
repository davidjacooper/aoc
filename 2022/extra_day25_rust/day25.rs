use std::time::Instant;
use std::collections::HashMap;

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input1.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            full_of_hot_air(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

fn full_of_hot_air(content: &str)
{
    let digits = [('=', -2), ('-', -1), ('0', 0), ('1', 1), ('2', 2)];
    let digits_parse = HashMap::from(digits);

    let mut sum: i64 =
        content
        .trim_end()
        .lines()
        .map(|line| line.chars().fold(0i64, |n, ch| n * 5 + digits_parse.get(&ch).unwrap()))
        .sum();

    println!("Decimal: {sum}");
    let mut snafu_sum_rev = String::new();
    while sum > 0
    {
        sum += 2;
        let n = sum % 5;
        sum /= 5;
        snafu_sum_rev.push(digits[n as usize].0);
    }
    print!("SNAFU: ");
    snafu_sum_rev.chars().rev().for_each(|ch| print!("{ch}"));
    println!();
}
