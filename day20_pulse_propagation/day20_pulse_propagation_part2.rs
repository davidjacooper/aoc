use std::time::Instant;
use std::collections::{HashMap,HashSet,VecDeque};
use std::io::Write;

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input1.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            pulse_propagation_part1(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}


#[derive(Debug)]
enum ModuleType { Broadcast, FlipFlop, Conjunction }
use ModuleType::*;

#[derive(Debug)]
struct Module<'a>
{
    mtype: ModuleType,
    dest: Vec<&'a str>,
}

struct ConjState<'a>
{
    which: HashSet<&'a str>,
    n_input: u32,
    n_input_high: u32,
}


fn pulse_propagation_part1(content: &str)
{
    let modules: HashMap<&str,Module> =
        content
        .trim_end()
        .lines()
        .map(|line| {
            let (name_str, dest_str) = line.split_once(" -> ").unwrap();
            let (mtype, name) = match name_str.chars().nth(0).unwrap()
            {
                'b' => (Broadcast, name_str),
                '%' => (FlipFlop, &name_str[1..]),
                '&' => (Conjunction, &name_str[1..]),
                _   => panic!()
            };
            return (
                name,
                Module {
                    mtype: mtype,
                    dest: dest_str.split(", ").collect()
                }
            )
        })
        .collect();

    let untyped_modules: Vec<&str> =
        modules
        .values()
        .flat_map(|m| m.dest.iter())
        .copied()
        .filter(|name| !modules.contains_key(name))
        .collect();

    let mut flip_flop_state: HashMap<&str,bool> =
        modules
        .iter()
        .filter_map(|(name, module)|
            match &module.mtype { FlipFlop => Some((*name, false)), _ => None })
        .collect();

    let mut conj_state: HashMap<&str,ConjState> =
        modules
        .iter()
        .filter_map(|(name, module)|
            match &module.mtype
            {
                Conjunction => Some((*name, ConjState { which: HashSet::new(),
                                                        n_input: 0,
                                                        n_input_high: 0 })),
                _           => None
            })
        .collect();

    for Module { dest, .. } in modules.values()
    {
        for d in dest.iter()
        {
            if let Some(cs) = conj_state.get_mut(d)
            {
                cs.n_input += 1;
            }
        }
    }

    let mut pulses = VecDeque::<(&str,&str,bool)>::new();
    let mut n_low_pulses: u64 = 0;
    let mut n_high_pulses: u64 = 0;

    for i in 1..10000
    {
        pulses.push_back(("button", "broadcaster", false));

        while let Some((from_name, to_name, pulse_high)) = pulses.pop_front()
        {
            if pulse_high
            {
                n_high_pulses += 1
            }
            else
            {
                n_low_pulses += 1;
            }

            //println!("{from_name} --{}--> {to_name}", if pulse_high { "high" } else { "low" });

            let Some(Module{mtype, dest}) = modules.get(to_name) else { continue };
            let mut send: Option<bool> = None;

            match mtype
            {
                Broadcast =>
                {
                    send = Some(pulse_high);
                }
                FlipFlop =>
                {
                    if !pulse_high
                    {
                        let state = flip_flop_state.get_mut(to_name).unwrap();
                        *state = !*state;
                        send = Some(*state);
                    }
                }
                Conjunction =>
                {
                    let state = conj_state.get_mut(to_name).unwrap();
                    let remembered_high = state.which.contains(from_name);

                    if pulse_high && !remembered_high
                    {
                        state.n_input_high += 1;
                        state.which.insert(from_name);
                    }
                    else if !pulse_high && remembered_high
                    {
                        state.n_input_high -= 1;
                        state.which.remove(&from_name);
                    }

                    send = Some(state.n_input_high < state.n_input);

                    if send == Some(false) && state.n_input > 1
                    {
                        println!(" conj {to_name} sending LOW signal on button press {i}");
                    }
                }
            }

            if let Some(next_pulse) = send
            {
                for d in dest.iter()
                {
                    pulses.push_back((to_name, d, next_pulse));
                }
            }
        }
    }

    // println!("#low = {n_low_pulses}, #high = {n_high_pulses} => product = {}", n_low_pulses * n_high_pulses);
}


fn make_graph(modules: &HashMap<&str,Module>,
              untyped_modules: &Vec<&str>) -> std::io::Result<()>
{
    let mut graph = std::fs::File::create("graph.dot")?;
    writeln!(graph, "digraph {{\n  overlap=false")?;
    for (name, Module{mtype, ..}) in modules.iter()
    {
        writeln!(graph,
                 "  {} [shape=\"box\",style=\"filled\",fillcolor=\"{}\"]",
                 name,
                 match mtype {
                     Broadcast => "green",
                     FlipFlop  => "cyan",
                     Conjunction => "red",
                 })?;
    }

    for name in untyped_modules.iter()
    {
        writeln!(graph, "  {} [shape=\"box\"]", name)?;
    }

    for (name, Module{ dest, .. }) in modules.iter()
    {
        for d in dest.iter()
        {
            writeln!(graph, "  {} -> {}", name, d)?;
        }
    }
    writeln!(graph, "}}")?;
    println!("graphviz... {:?}", std::process::Command::new("neato").args(["-Tpdf", "-O", "graph.dot"]).output());
    Ok(())
}
