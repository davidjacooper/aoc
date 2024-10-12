use std::time::Instant;
use std::collections::HashMap;

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            proboscidea_volcanium_part1(&content);
            println!("Time: {:?}", Instant::now().duration_since(start));
        }
    }
}

#[derive(Debug)]
struct Valve<'a>
{
    #[allow(dead_code)]
    index: usize,
    #[allow(dead_code)]
    name: &'a str,
    flow_rate: i32,
    connecting_names: Vec<&'a str>,
    connecting_indexes: Vec<usize>,
}


fn proboscidea_volcanium_part1(content: &str)
{
    let mut valve_indexes = HashMap::<&str,usize>::new();
    let mut valves: Vec<Valve> = content
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(index, line)|
            match line.split_whitespace().collect::<Vec<&str>>().as_slice()
            {
                [_, name, _, _, rate_spec, _, _, _, _, connecting @ ..] =>
                {
                    //println!("rate_spec='{rate_spec}'");
                    valve_indexes.insert(name, index);
                    return Valve {
                        index: index,
                        name: name,
                        flow_rate: rate_spec.trim_start_matches("rate=").trim_end_matches(";").parse().unwrap(),
                        connecting_names: connecting.iter().map(|s| s.trim_end_matches(",")).collect(),
                        connecting_indexes: Vec::with_capacity(0) // temp
                    }
                },
                _ => {
                    panic!("Panicking about '{}'", line);
                }
            })
        .collect();

    make_graph(&valves);

    for valve in valves.iter_mut()
    {
        valve.connecting_indexes = valve.connecting_names.iter().map(|name| valve_indexes[name]).collect();
    }

    println!("max={}", calc_route(valve_indexes["AA"], 30, 0, &valves, 0, 0));
}


fn calc_route(location: usize, time_left: i32, flow_rate: i32, valves: &[Valve], valves_on: u64, visited: u64) -> i32
{
    if time_left == 0 { return 0; }

    return valves[location]
        .connecting_indexes
        .iter()
        .filter(|v| visited & (1 << *v) == 0)
        .map(|next_location| calc_route(*next_location, time_left - 1, flow_rate, valves, valves_on, visited | (1 << location)))
        .chain(
            if (valves_on & (1 << location)) == 0 && valves[location].flow_rate > 0
            {
                Some(calc_route(location, time_left - 1, flow_rate + valves[location].flow_rate, valves, valves_on | (1 << location), 0))
            }
            else
            {
                None
            })
        .max()
        .unwrap_or(0)
        + flow_rate
}

fn make_graph(valves: &Vec<Valve>)
{
    let mut graph = String::new();
    graph.push_str("graph {\n  overlap=false\n");
    for valve in valves.iter()
    {
        graph.push_str(&format!("  {} [label=\"{0} ({})\",color=\"{}\",penwidth={}]\n",
                                valve.name,
                                valve.flow_rate,
                                if valve.flow_rate == 0 { "gray" } else { "blue" },
                                if valve.flow_rate == 0 { 1 }      else { 2 }));
    }
    for valve in valves.iter()
    {
        for connecting_name in valve.connecting_names.iter()
        {
            graph.push_str(&format!("  {} -- {}\n", valve.name, connecting_name));
        }
    }
    graph.push_str("}\n");
    let _ = std::fs::write("graph.dot", graph);
    println!("graphviz... {:?}", std::process::Command::new("neato").args(["-Tpdf", "-O", "graph.dot"]).output());
}
