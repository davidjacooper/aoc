use std::time::Instant;
use std::collections::{HashMap, HashSet};

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input1.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            haunted_wasteland_part2(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

#[derive(Debug)]
struct Node<'a>
{
    left: &'a str,
    right: &'a str,
}

fn haunted_wasteland_part2(content: &str)
{
    let (steps, node_str) = content.split_once("\n\n").unwrap();

    let nodes: HashMap<&str,Node> =
        node_str
        .lines()
        .map(|line| (&line[0..3], Node { left: &line[7..10], right: &line[12..15] }) )
        .collect();

    let mut sub_cycles = Vec::<i64>::new();

    for start_node in nodes.keys().filter(|k| k.ends_with("A"))
    {
        let mut n_steps = 0;
        let mut current = *start_node;
        for step in steps.chars().cycle()
        {
            if current.ends_with("Z")
            {
                break;
            }

            let node = nodes.get(current).unwrap();
            current = match step
            {
                'L' => node.left,
                'R' => node.right,
                _ => panic!(),
            };
            n_steps += 1;
        }
        sub_cycles.push(n_steps);
    }

    println!("sub cycles = {:?}", sub_cycles);

    let mut factors = HashSet::<i64>::new();

    for mut sub_cycle in sub_cycles
    {
        for f in 2..((sub_cycle as f64).sqrt() as i64 + 1)
        {
            while sub_cycle % f == 0
            {
                factors.insert(f);
                sub_cycle /= f;
            }
        }
        if sub_cycle > 1
        {
            factors.insert(sub_cycle);
        }
    }

    println!("total steps = {}", factors.iter().product::<i64>());
}
