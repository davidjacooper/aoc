use std::time::Instant;

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("../inputs/day02_input.txt".to_string());
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
    let mut x = 0;
    let mut depth = 0;

    for line in content.trim_end().lines()
    {
        let (dir, dist) = line.split_once(" ").unwrap();
        let dist_n: i32 = dist.parse().unwrap();
        match dir
        {
            "forward" => x     += dist_n,
            "up"      => depth -= dist_n,
            _         => depth += dist_n,
        }
    }

    println!("x=={}, depth=={}, product={}", x, depth, x * depth);
    Ok(())
}
