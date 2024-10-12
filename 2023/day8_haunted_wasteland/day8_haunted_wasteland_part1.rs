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

    make_graph(&nodes);

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


fn make_graph(nodes: &HashMap<&str,Node>)
{
    let mut graph = String::new();
    graph.push_str("digraph {\n  overlap=false\n");
    for name in nodes.keys()
    {
        graph.push_str(&format!("  {} [style=\"filled\",fillcolor=\"{}\"]\n",
                                name,
                                if name.ends_with("A") { "green" }
                                else if name.ends_with("Z") { "red" }
                                else { "white" }));
    }
    for (name, Node { left, right }) in nodes.iter()
    {
        graph.push_str(&format!("  {} -> {} [color=\"blue\"]\n", name, left));
        graph.push_str(&format!("  {} -> {} [color=\"magenta\"]\n", name, right));
    }
    graph.push_str("}\n");
    let _ = std::fs::write("graph.dot", graph);
    println!("graphviz... {:?}", std::process::Command::new("neato").args(["-Tpdf", "-O", "graph.dot"]).output());
}
