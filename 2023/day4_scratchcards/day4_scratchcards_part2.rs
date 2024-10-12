use std::collections::{HashSet, VecDeque};

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input1.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) => scratchcards_part2(&content)
    }
}

fn scratchcards_part2(content: &str)
{
    let mut sum = 0;
    let mut copy_stack = VecDeque::from(vec![1; 1000]);

    for line in content.lines()
    {
        if !line.is_empty()
        {
            let (winning_str, have_str) = line[(line.find(':').unwrap() + 2)..].split_once(" | ").unwrap();
            let n_common = as_set(winning_str).intersection(&as_set(have_str)).count();

            let copies = copy_stack.pop_front().unwrap();
            sum += copies;

            for i in 0..n_common
            {
                *copy_stack.get_mut(i).unwrap() += copies;
            }
            println!("copies=={copies}, n_common=={n_common}");
        }
    }

    println!("Sum == {}", sum);
}

fn as_set(string: &str) -> HashSet<i32>
{
    return string.split_whitespace().map(|s| s.parse().unwrap()).collect();
}
