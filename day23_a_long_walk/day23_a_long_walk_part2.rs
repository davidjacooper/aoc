use std::time::Instant;
use std::collections::{HashSet,HashMap,VecDeque};
use std::io::Write;

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            if let Err(err) = a_long_walk_part1(&content)
            {
                println!("ERROR: {}", err);
            }
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

/*
 * Similarly to part 1, we first abstract the input into a minimal node-edge graph, since there
 * are generally many input grid squares that collapse into a single graph edge.
 *
 * However, in part 2, the graph is undirected, and there are _many_ cycles. Our BFS thing from
 * before won't work.
 *
 * I'm sure there is a smarter way to do this, but the solution here is just a brute-force-ish
 * depth-first search. It takes a little under a minute.
 */

#[derive(Debug)]
struct Node
{
    index: usize,
    i: usize,
    j: usize,
    dest: Vec<(usize,usize)>
}

fn a_long_walk_part1(content: &str) -> Result<(),&str>
{
    let grid: Vec<Vec<char>> =
        content
        .trim_end()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // print_grid(&grid);

    let nodes = find_graph(&grid);
    make_graph_pdf(&nodes).unwrap();

    let n_steps = find_longest_distance(0, 1, &nodes);
    println!("#steps = {n_steps}");

    Ok(())
}

fn print_grid(grid: &Vec<Vec<char>>)
{
    for row in grid.iter()
    {
        for ch in row.iter()
        {
            print!("{}", ch);
        }
        println!();
    }
}

fn find_graph(grid: &Vec<Vec<char>>) -> Vec<Node>
{
    let height = grid.len();
    let width = grid[0].len();

    let mut nodes = Vec::<Node>::new();
    let mut node_indexes = HashMap::<(usize,usize),usize>::new();

    nodes.push(Node { index: 0, i: 0,          j: 1,         dest: Vec::new() });
    nodes.push(Node { index: 1, i: height - 1, j: width - 2, dest: Vec::new() });

    node_indexes.insert((nodes[0].i, nodes[0].j), 0);
    node_indexes.insert((nodes[1].i, nodes[1].j), 1);

    // Find nodes
    for (i, row) in grid.iter()
                        .enumerate()
                        .skip(1)
                        .take_while(|(i, _)| *i < height - 1)
    {
        for (j, ch) in row.iter()
                          .enumerate()
                          .skip(1)
                          .take_while(|(j, _)| *j < width - 1)
        {
            if *ch == '.'
            {
                let n_connections = (grid[i - 1][j] != '#') as u8 +
                                    (grid[i + 1][j] != '#') as u8 +
                                    (grid[i][j - 1] != '#') as u8 +
                                    (grid[i][j + 1] != '#') as u8;
                if n_connections > 2
                {
                    let index = nodes.len();
                    node_indexes.insert((i, j), index);
                    nodes.push(Node { index: index, i: i, j: j, dest: Vec::new() });
                }
            }
        }
    }

    // Find node connections
    for index in 0..nodes.len() //node in &mut nodes
    {
        let node = &nodes[index];
        let mut explored = HashSet::<(usize,usize)>::new();
        let mut queue = VecDeque::<(usize,usize,usize)>::new();

        explored.insert((node.i, node.j));
        if node.i > 0          { queue.push_back((node.i - 1, node.j, 1)) }
        if node.i < height - 1 { queue.push_back((node.i + 1, node.j, 1)) }
        queue.push_back((node.i, node.j - 1, 1));
        queue.push_back((node.i, node.j + 1, 1));

        while let Some((i, j, dist)) = queue.pop_front()
        {
            if !explored.insert((i, j)) // Tests whether it was already present
            {
                continue
            }

            if let Some(to_node) = node_indexes.get(&(i, j))
            {
                let to_index = nodes[*to_node].index;
                nodes[index].dest.push((to_index, dist));
            }
            else
            {
                if grid[i][j] != '#'
                {
                    if i > 0          { queue.push_back((i - 1, j, dist + 1)) }
                    if i < height - 1 { queue.push_back((i + 1, j, dist + 1)) }
                    queue.push_back((i, j - 1, dist + 1));
                    queue.push_back((i, j + 1, dist + 1));
                }
            }
        }
    }

    nodes
}

// Failed attempt:
// fn find_longest_distance(start: usize, end: usize, nodes: &Vec<Node>) -> usize
// {
//     // Verified assumption: there are no cycles.
//
//     let mut path = vec![(0, HashSet::<usize>::new()); nodes.len()];
//     let mut queue = VecDeque::<usize>::new();
//     queue.push_back(start);
//     while let Some(index_a) = queue.pop_front()
//     {
//         for (index_b, dist_a_to_b) in nodes[index_a].dest.iter()
//         {
//             let (dist_a, explored_a) = &path[index_a];
//             if explored_a.contains(index_b) { continue }
//
//             let (dist_b, _) = path[*index_b];
//             let new_dist_b = dist_a + dist_a_to_b;
//
//             if dist_b < new_dist_b
//             {
//                 let mut new_explored_b = explored_a.clone();
//                 new_explored_b.insert(index_a);
//                 path[*index_b] = (new_dist_b, new_explored_b);
//
//                 queue.push_back(*index_b);
//             }
//
//             // total_dist[*to_index] = total_dist[*to_index].max(total_dist[from_index] + dist);
//             // queue.push_back(*to_index);
//         }
//     }
//     // return total_dist[end];
//     path[end].0
// }


fn find_longest_distance(start: usize, end: usize, nodes: &Vec<Node>) -> usize
{
    let mut explored = HashSet::new();
    find_longest_distance_recurse(0, start, end, nodes, &mut explored)
}

fn find_longest_distance_recurse(dist_so_far: usize, from: usize, end: usize,
                                 nodes: &Vec<Node>, explored: &mut HashSet<usize>) -> usize
{
    if from == end { return dist_so_far }

    let mut dist = 0;
    explored.insert(from);

    for (to_index, edge_dist) in nodes[from].dest.iter()
    {
        if !explored.contains(to_index)
        {
            dist = dist.max(find_longest_distance_recurse(dist_so_far + edge_dist,
                                                          *to_index,
                                                          end,
                                                          nodes,
                                                          explored));
        }
    }

    explored.remove(&from);
    return dist;
}


fn make_graph_pdf(nodes: &Vec<Node>) -> std::io::Result<()>
{
    let mut graph = std::fs::File::create("graph2.dot")?;
    writeln!(graph, "graph {{\n  overlap=false")?;
    for Node{index, i, j, ..} in nodes.iter()
    {
        writeln!(graph,
                 "  n{} [label=\"{0}:({},{})\",shape=\"box\",style=\"filled\",fillcolor=\"{}\"]",
                 index,
                 i,
                 j,
                 match index { 0 => "green", 1 => "red", _ => "white" })?;
    }

    for node in nodes.iter()
    {
        for (to_index, dist) in node.dest.iter()
        {
            if node.index < *to_index
            {
                writeln!(graph,
                        "  n{} -- n{} [label=\"{}\"]",
                        node.index,
                        to_index,
                        dist)?;
            }
        }
    }
    writeln!(graph, "}}")?;
    println!("graphviz... {:?}", std::process::Command::new("neato").args(["-Tpdf", "-O", "graph2.dot"]).output());
    Ok(())
}
