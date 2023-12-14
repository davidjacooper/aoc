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
            parabolic_reflector_dish_part1(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

fn parabolic_reflector_dish_part1(content: &str)
{
    let mut platform = content
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    //print_platform(&platform);

    let orig_platform = platform.clone();
    let mut prev_states = HashMap::<Vec<u128>,u64>::new();
    prev_states.insert(calc_state(&platform), 0);

    // First, we figure out how long the state sequence takes to repeat itself,
    // and where it repeats from.

    let mut cycles: u64 = 0;
    let repeat_from: u64;
    loop
    {
        cycle(&mut platform);
        cycles += 1;
        //println!("---\nAfter cycle {}", cycles);
        //print_platform(&platform);

        let new_state = calc_state(&platform);
        if let Some(from) = prev_states.get(&new_state)
        {
            repeat_from = *from;
            break;
        }
        prev_states.insert(new_state, cycles);
    }

    println!("cycles = {cycles}, repeat_from = {repeat_from}");

    // Then, we reset everything, and run only to the point that should be
    // equivalent to the target number of cycles.

    platform = orig_platform;
    let target = 1000000000;
    for _ in 0..(repeat_from + (target - repeat_from) % (cycles - repeat_from))
    {
        cycle(&mut platform);
    }

    let mut sum = 0;
    let mut weight = platform.len();
    for row in platform.iter()
    {
        let load = row.iter().filter(|c| **c == 'O').count() * weight;
        sum += load;
        weight -= 1;
        //println!("{load}");
    }
    println!("sum = {sum}");
}


fn print_platform(platform: &Vec<Vec<char>>)
{
    println!("---");
    for row in platform.iter()
    {
        println!("{}", row.iter().collect::<String>());
    }
}

fn calc_state(platform: &Vec<Vec<char>>) -> Vec<u128>
{
    return platform
        .iter()
        .map(|row|
            row.iter().enumerate()
                .map(|(i, ch)| if *ch == 'O' { 1u128 << i } else { 0 })
                .sum())
        .collect();
}

fn cycle(mut platform: &mut Vec<Vec<char>>)
{
    for _ in 0..4
    {
        slide_rocks(&mut platform);
        rotate(&mut platform);
    }
}

fn slide_rocks(platform: &mut Vec<Vec<char>>)
{
    let width = platform[0].len();
    let mut blocking = vec![0; width];
    for i in 0..platform.len()
    {
        for j in 0..width
        {
            match platform[i][j]
            {
                '#' => blocking[j] = i + 1,
                'O' => {
                    platform[i][j] = '.';
                    platform[blocking[j]][j] = 'O';
                    blocking[j] += 1;
                }
                _ => {}
            }
        }
    }
}

fn rotate(p: &mut Vec<Vec<char>>)
{
    let size = p.len(); // assume a square
    for i in 0..(size / 2)
    {
        for j in 0..(size / 2)
        {
            let inv_i = size - 1 - i;
            let inv_j = size - 1 - j;
            (p[j][inv_i], p[inv_i][inv_j], p[inv_j][i], p[i][j]) =
                (p[i][j], p[j][inv_i], p[inv_i][inv_j], p[inv_j][i])
        }
    }
}
