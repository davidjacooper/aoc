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
            lens_library_part1(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

fn lens_library_part1(content: &str)
{
    let mut sum = 0;
    for step in content.trim().split(",")
    {
        let mut hash = 0;
        for b in step.bytes().filter(|b| *b != b'\n')
        {
            hash = (hash + b as u32) * 17 % 256;
        }
        println!("step='{step}', hash={hash}");
        sum += hash;
    }
    println!("sum = {sum}");
}
