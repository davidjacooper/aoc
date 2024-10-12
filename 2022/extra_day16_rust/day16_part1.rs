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

#[derive(Clone,Debug)]
struct State
{
    location: usize,
    last_location: usize,
    valves_on: i64,
    flow_rate: i32,
    pressure_released: i32,
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

    for valve in valves.iter_mut()
    {
        valve.connecting_indexes = valve.connecting_names.iter().map(|name| valve_indexes[name]).collect();
    }

    print_valves(&valves);
    make_graph(&valves);

    let mut states = vec![State { location: valve_indexes["AA"],
                                  last_location: 0xffffffff,
                                  valves_on: 0,
                                  flow_rate: 0,
                                  pressure_released: 0 }];

    for time in 1..=30
    {
        let mut new_states = Vec::<State>::new();
        for state in states
        {
            let valve = &valves[state.location];

            if valve.flow_rate > 0 && (state.valves_on & (1 << state.location)) == 0
            {
                new_states.push(
                    State {
                        location: state.location,
                        last_location: 0xffffffff,
                        valves_on: state.valves_on | (1 << state.location),
                        flow_rate: state.flow_rate + valve.flow_rate,
                        pressure_released: state.pressure_released + state.flow_rate,
                    });
            }

            for connecting in valve.connecting_indexes.iter()
            {
                if *connecting != state.last_location
                {
                    new_states.push(
                        State {
                            location: *connecting,
                            last_location: state.location,
                            valves_on: state.valves_on,
                            flow_rate: state.flow_rate,
                            pressure_released: state.pressure_released + state.flow_rate,
                        });
                }
            }
        }

        new_states.sort_by(|a, b| (b.pressure_released, b.flow_rate).cmp(&(a.pressure_released, a.flow_rate)));

        new_states.truncate(100); // Tunable parameter; simply needs to be above a threshold value
                                  // (which can be determined by experimentation. Higher values
                                  // values are less efficient.

        states = new_states;
        print_states(time, &states);
    }

    println!("max = {}", states[0].pressure_released);

}

fn print_valves(valves: &Vec<Valve>)
{
    for valve in valves.iter()
    {
        println!("{:?}", valve);
    }
    println!("---");
}

fn print_states(time: i32, states: &Vec<State>)
{
    println!("time {time}");
    for state in states.iter()
    {
        println!("  {:?}", state);
    }
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
