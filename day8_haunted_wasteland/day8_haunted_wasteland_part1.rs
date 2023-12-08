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
            haunted_wasteland_part1(&content);
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

fn haunted_wasteland_part1(content: &str)
{
    let (steps, node_str) = content.split_once("\n\n").unwrap();

    let nodes: HashMap<&str,Node> =
        node_str
        .lines()
        .map(|line| (&line[0..3], Node { left: &line[7..10], right: &line[12..15] }) )
        .collect();

    println!("nodes = {:?}\n", nodes);

    let mut n_steps = 0;
    let mut current = "AAA";
    for step in steps.chars().cycle()
    {
        print!("{} ", current);
        if current == "ZZZ"
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

    println!("\n\nn_steps = {n_steps}");
}
