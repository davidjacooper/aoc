use std::time::Instant;

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input.txt".to_string());
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
    let mut count = 0;
    let mut last_depth = i32::MAX;

    for line in content.trim_end().lines()
    {
        let depth: i32 = line.parse().unwrap();
        if depth > last_depth
        {
            count += 1;
        }
        last_depth = depth;
    }

    println!("#depth increases == {}", count);
    Ok(())
}
