use std::collections::VecDeque;

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input1.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) => if_you_give_a_seed_a_fertilizer_part2(&content)
    }
}

fn if_you_give_a_seed_a_fertilizer_part2(content: &str)
{
    let (seeds, map_list) = content.split_once("\n\n").unwrap();

    let mut numbers: VecDeque<(i64,i64)> =
        seeds.split_whitespace()
             .skip(1) // Skip "seeds:"
             .map(|s| s.parse().unwrap())
             .collect::<Vec<i64>>()
             .chunks(2)
             .map(|chunk| (chunk[0], chunk[1]))
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

        let mut next_numbers = VecDeque::<(i64, i64)>::new();

        while let Some((mut n_start, mut n_range)) = numbers.pop_front()
        {
            for (map_dest_start, map_src_start, map_range) in &map
            {
                // Establish that the ranges overlap
                if n_start < (map_src_start + map_range) && *map_src_start < (n_start + n_range)
                {
                    // Split off parts of the number range before the mapping range
                    let preceeding = map_src_start - n_start;
                    if preceeding > 0
                    {
                        numbers.push_front((n_start, preceeding));
                        n_start += preceeding;
                        n_range -= preceeding;
                    }

                    // Split off parts of the number range after the mapping range
                    let succeeding = (n_start + n_range) - (map_src_start + map_range);
                    if succeeding > 0
                    {
                        numbers.push_front((map_src_start + map_range, succeeding));
                        n_range -= succeeding;
                    }

                    // Map only the overlap.
                    n_start += map_dest_start - map_src_start;
                    break;
                }
            }
            next_numbers.push_back((n_start, n_range));
        }
        numbers = next_numbers;

        println!("numbers = {:?}", numbers);
    }

    println!("Minimum = {}", numbers.iter()
                                    .map(|(start, range)| start)
                                    .min()
                                    .unwrap());
}
