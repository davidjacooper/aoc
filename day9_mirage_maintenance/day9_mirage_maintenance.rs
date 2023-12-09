use std::time::Instant;

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input1.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            mirage_maintenance(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

fn mirage_maintenance(content: &str)
{
    let mut sum_prev = 0;
    let mut sum_next = 0;
    for line in content.lines().filter(|l| !l.is_empty())
    {
        let (prev_n, next_n) = predict(line.split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<i32>>());
        println!("{prev_n} <= [{line}] => {next_n}");
        sum_prev += prev_n;
        sum_next += next_n;
    }
    println!("sum prev (part 2) = {sum_prev}, sum next (part 1) = {sum_next}");
}

fn predict(sequence: Vec<i32>) -> (i32, i32)
{
    return if sequence.iter().all(|i| *i == 0)
    {
        (0, 0)
    }
    else
    {
        let (prev_diff, next_diff) = predict(sequence.windows(2).map(|win| win[1] - win[0]).collect());
        (sequence[0] - prev_diff, sequence[sequence.len() - 1] + next_diff)
    }
}
