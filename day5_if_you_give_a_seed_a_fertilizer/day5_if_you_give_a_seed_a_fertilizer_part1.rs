fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input1.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) => if_you_give_a_seed_a_fertilizer_part1(&content)
    }
}

fn if_you_give_a_seed_a_fertilizer_part1(content: &str)
{
    let (seeds, map_list) = content.split_once("\n\n").unwrap();

    let mut numbers: Vec<i64> = seeds.split_whitespace()
                                     .skip(1) // Skip "seeds:"
                                     .map(|s| s.parse().unwrap())
                                     .collect();

    println!("numbers = {:?}", numbers);

    for map_str in map_list.split("\n\n")
    {
        let map: Vec<(i64, i64, i64)> =
            map_str.lines()
                   .skip(1) // Skip map header
                   .map(|line|
                   {
                       let mut it = line.splitn(3, ' ')
                                        .map(|s| s.parse().unwrap());
                       return (it.next().unwrap(),
                               it.next().unwrap(),
                               it.next().unwrap());
                   })
                   .collect();

        for n in &mut numbers
        {
            for (dest_start, src_start, range) in &map
            {
                if *src_start <= *n && *n < (src_start + range)
                {
                    *n += dest_start - src_start;
                    break;
                }
            }
        }

        println!("numbers = {:?}", numbers);
    }

    println!("Minimum = {}", numbers.iter().min().unwrap());
}
