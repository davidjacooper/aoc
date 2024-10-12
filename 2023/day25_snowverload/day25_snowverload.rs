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
            if let Err(err) = snowverload(&content)
            {
                println!("ERROR: {}", err);
            }
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

/*
 * Here, we do a series of BFSs, starting from each node in turn, and keep a tally of the number of
 * times we encounter each edge in the process. The theory is that at least one of the three key
 * edges _must show up on every search, whereas the other edges might not.
 *
 * In the general case, this doesn't necessarily, absolutely mean that the highest tallies will be
 * for the three key edges, and this isn't even true for the test input. BUT it works for the real
 * input, perhaps for reasons of scale, so there.
 *
 * Once we know which three edges to disconnect, we do one last BFS, from one arbitrary node, and
 * count the number of other nodes we can visit. If we've succeeded, then this will be somewhat
 * less than the total number of nodes.
 */


fn snowverload(content: &str) -> Result<(),&str>
{
    let mut connections = HashMap::<&str,Vec<&str>>::new();

    for line in content.trim_end().lines()
    {
        let (l_node, r_node_str) = line.split_once(": ").unwrap();

        connections.entry(l_node).or_insert_with(|| Vec::new());
        for r_node in r_node_str.split_whitespace()
        {
            connections
                .get_mut(l_node)
                .unwrap()
                .push(r_node);
            connections
                .entry(r_node)
                .or_insert_with(|| Vec::new())
                .push(l_node);
        }
    }

    println!("Doing lots of BFSs");
    let mut edge_tally = calc_edge_tally(&connections);
    edge_tally.sort_by(|a, b| b.cmp(a));

    let disallowed_edges = HashSet::from([
        (edge_tally[0].1, edge_tally[0].2),
        (edge_tally[1].1, edge_tally[1].2),
        (edge_tally[2].1, edge_tally[2].2)]);

    make_graph_pdf(&connections, &disallowed_edges).unwrap();

    println!("Checking graph after disallowing {:?}", disallowed_edges);

    let mut explored = HashSet::<&str>::new();
    let mut queue = VecDeque::from([connections.keys().nth(0).unwrap()]);
    while let Some(node1) = queue.pop_front()
    {
        if !explored.insert(node1) { continue }

        for node2 in connections.get(node1).unwrap()
        {
            if !disallowed_edges.contains(&(node1, node2)) &&
               !disallowed_edges.contains(&(node2, node1))
            {
                queue.push_back(node2);
            }
        }
    }

    let n_total = connections.len();
    let n_group_a = explored.len();
    let n_group_b = n_total - n_group_a;
    println!("Nodes: total - {n_total}, group A - {n_group_a}, group B - {n_group_b}, product = {}",
             n_group_a * n_group_b);

    Ok(())
}

fn calc_edge_tally<'a>(connections: &'a HashMap<&'a str,Vec<&'a str>>)
    -> Vec<(usize,&'a str,&'a str)>
{
    let mut edge_tally = HashMap::<(&str,&str),usize>::new();

    let mut explored = HashSet::<&str>::new();
    let mut queue = VecDeque::<(&str,&str)>::new();

    for start_node in connections.keys()
    {
        explored.clear();
        for next_node in connections.get(start_node).unwrap()
        {
            queue.push_back((start_node, next_node));
        }

        while let Some((node1, node2)) = queue.pop_front()
        {
            if !explored.insert(node2) { continue }

            let key = if node1 <= node2 { (node1, node2) }
                      else              { (node2, node1) };

            *edge_tally.entry(key).or_insert(0) += 1;

            for node3 in connections.get(node2).unwrap()
            {
                queue.push_back((node2, node3));
            }
        }
    }

    edge_tally
        .iter()
        .map(|((node1, node2), tally)| (*tally, *node1, *node2))
        .collect()
}


fn make_graph_pdf(connections: &HashMap<&str,Vec<&str>>,
                  disallowed_edges: &HashSet<(&str,&str)> ) -> std::io::Result<()>
{
    let mut graph = std::fs::File::create("graph.dot")?;
    writeln!(graph, "graph {{\n  overlap=false")?;

    for (from_node, to_node) in disallowed_edges
    {
        writeln!(graph, "  {from_node} [style=\"filled\",fillcolor=\"red\"]")?;
        writeln!(graph, "  {to_node} [style=\"filled\",fillcolor=\"red\"]")?;
    }

    for (from_node, to_nodes) in connections
    {
        for to_node in to_nodes
        {
            if from_node < to_node
            {

                writeln!(
                    graph,
                    "  {from_node} -- {to_node} [{}]",
                    if disallowed_edges.contains(&(from_node, to_node)) ||
                        disallowed_edges.contains(&(to_node, from_node))
                    {
                        "len=100, penwidth=3, color=\"red\""
                    }
                    else
                    {
                        ""
                    }
                )?;
            }
        }
    }
    writeln!(graph, "}}")?;
    println!("graphviz... {:?}", std::process::Command::new("neato").args(["-Tpdf", "-O", "graph.dot"]).output());
    Ok(())
}
