use std::time::Instant;
use std::collections::VecDeque;

macro_rules! if_you_give_a_seed_a_fertilizer_part2
{
    (
        seeds : $( $n_start:literal $n_range:literal )+
        $(
            $_y:ident - to - $_z:ident map :
            $(
                $map_dest_start:literal
                $map_src_start:literal
                $map_range:literal
            )+
        )+
    ) =>
    {
        let mut numbers = VecDeque::from([ $( ($n_start, $n_range) ),+ ]);

        for map in [ $( vec![ $( ($map_dest_start, $map_src_start, $map_range) ),+ ] ),+ ]
        {
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
        }
        println!("Minimum = {}", numbers.iter()
                                        .map(|(start, _)| start)
                                        .min()
                                        .unwrap());
    }
}

fn main()
{
    let start = Instant::now();
    if_you_give_a_seed_a_fertilizer_part2!(
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    );
    println!("Time: {:?}", Instant::now().duration_since(start));
}
