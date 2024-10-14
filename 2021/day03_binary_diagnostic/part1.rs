use std::time::Instant;

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
    let mut n_rows = 0;
    let mut counts = vec![0; 12];
    let mut n_cols = 0;

    for line in content.trim_end().lines()
    {
        n_cols = line.len();
        n_rows += 1;
        for (i, ch) in line.chars().enumerate()
        {
            if ch == '1'
            {
                counts[n_cols - i - 1] += 1;
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..n_cols
    {
        if counts[i] * 2 >= n_rows
        {
            gamma += 1 << i;
        }
        else
        {
            epsilon += 1 << i;
        }
    }

    println!("gamma=={}, epsilon=={}, product=={}", gamma, epsilon, gamma * epsilon);
    println!("n_rows=={}, n_cols=={}, counts=={:?}", n_rows, n_cols, counts);
    Ok(())
}
