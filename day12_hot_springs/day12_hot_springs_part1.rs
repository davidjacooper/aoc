// Part 1: recursive solution

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
            hot_springs_part1(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

fn hot_springs_part1(content: &str)
{
    let mut sum = 0;
    for line in content.lines().filter(|l| !l.is_empty())
    {
        let (spring_str, broken_group_str) = line.split_once(' ').unwrap();
        let springs: Vec<char> = spring_str.chars().collect();
        let broken_groups: Vec<usize> = broken_group_str
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        print!("springs={:?}, groups={:?} => ", springs.iter().collect::<String>(), broken_groups);

        let arrangements = count_arrangements(0, &springs, &broken_groups);
        sum += arrangements;
        println!("arrangements={arrangements}");
    }
    println!("sum = {sum}");
}

fn count_arrangements(nest: usize, springs: &[char], broken_groups: &[usize]) -> i32
{
    if springs.len() == 0
    {
        return if broken_groups.len() == 0 { 1 } else { 0 };
    }

    return match springs[0]
    {
        '#' => count_arrangements_in_sequence(nest + 1, springs, broken_groups),
        '.' => count_arrangements(nest + 1, &springs[1..], broken_groups),
        '?' => {
            count_arrangements_in_sequence(nest + 1, springs, broken_groups)
            + count_arrangements(nest + 1, &springs[1..], broken_groups)
        },
        _ => panic!()
    };
}

fn count_arrangements_in_sequence(nest: usize, springs: &[char], broken_groups: &[usize]) -> i32
{
    if broken_groups.len() == 0 { return 0; }
    let len = broken_groups[0];
    for i in 0..len
    {
        let Some(ch) = springs.get(i) else { return 0 };
        if *ch == '.' { return 0; }
    }

    return if let Some(ch) = springs.get(len)
    {
        if *ch == '#' { 0 }
        else
        {
            count_arrangements(nest + 1, &springs[(len + 1)..], &broken_groups[1..])
        }
    }
    else
    {
        if broken_groups.len() == 1 { 1 } else { 0 }
    };
}
