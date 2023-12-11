use std::time::Instant;
use std::collections::{HashMap, HashSet, VecDeque};
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
            monkey_math(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

#[derive(Debug)]
enum Operation { Add, Multiply, Subtract, Divide}

#[derive(Debug)]
enum Monkey<'a>
{
    Literal(i64),
    Expr(Operation, &'a str, &'a str, ),
}

fn monkey_math(content: &str)
{
    use Monkey::*;
    use Operation::*;

    let mut monkey_names = Vec::<&str>::new();
    let monkeys: HashMap<&str,Monkey> =
        content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();
            let name = words[0].trim_end_matches(":");
            monkey_names.push(name);
            return (
                name,
                match words[1..]
                {
                    [n_str]     => Literal(n_str.parse().unwrap()),
                    [a, "+", b] => Expr(Add, a, b),
                    [a, "*", b] => Expr(Multiply, a, b),
                    [a, "-", b] => Expr(Subtract, a, b),
                    [a, "/", b] => Expr(Divide, a, b),
                    _ => panic!(),
                }
            );
        })
        .collect();

    // print_monkeys(&monkey_names, &monkeys);

    let mut results = HashMap::<&str, i64>::new();
    let mut to_recalc = HashSet::<&str>::new();
    let mut stack = VecDeque::from(vec!["root"]);


    while let Some(monkey_name) = stack.pop_front()
    {
        if monkey_name == "humn"
        {
            to_recalc.insert(monkey_name);
        }
        match monkeys[monkey_name]
        {
            Literal(n) =>
            {
                results.insert(monkey_name, n);
            }
            Expr(ref op, a_name, b_name) =>
            {
                let maybe_a = results.get(a_name);
                let maybe_b = results.get(b_name);

                if let (Some(a), Some(b)) = (maybe_a, maybe_b)
                {
                    results.insert(monkey_name, match op
                    {
                        Add =>      a + b,
                        Multiply => a * b,
                        Subtract => a - b,
                        Divide =>   a / b
                    });

                    if to_recalc.contains(a_name) || to_recalc.contains(b_name)
                    {
                        to_recalc.insert(monkey_name);
                    }
                }
                else
                {
                    stack.push_front(monkey_name);
                    if let None = maybe_a
                    {
                        stack.push_front(a_name);
                    }
                    if let None = maybe_b
                    {
                        stack.push_front(b_name);
                    }
                }
            }
        }
    }
    println!("(part 1) root = {}", results["root"]);

    // Assume (as per the data) that this is strictly a tree, so only one branch leads
    // to the "humn" node.

    let Expr(_, root_child_a, root_child_b) = monkeys["root"] else { panic!() };
    let (mut current, mut target) =
        if to_recalc.contains(root_child_a) { (root_child_a, results[root_child_b]) }
        else                                { (root_child_b, results[root_child_a]) };

    while let Expr(op, a, b) = &monkeys[current]
    {
        let recalc_a = to_recalc.contains(a);
        target = match op
        {
            Add      => target - if recalc_a { results[b] } else { results[a] },
            Multiply => target / if recalc_a { results[b] } else { results[a] },
            Subtract => if recalc_a { target + results[b] } else { results[a] - target },
            Divide =>   if recalc_a { target * results[b] } else { results[a] / target },
        };
        current = if recalc_a { a } else { b };
    }
    println!("(part 2) humn = {target}");

    // print_monkeys(&monkey_names, &results);
    make_graph(&monkeys, &to_recalc);
}


fn print_monkeys<T>(monkey_names: &Vec<&str>, monkeys: &HashMap<&str,T>)
    where T: std::fmt::Debug
{
    for name in monkey_names.iter()
    {
        println!("  {:?}", monkeys[name]);
    }
    println!("---");
}


fn make_graph(monkeys: &HashMap<&str,Monkey>, to_recalc: &HashSet<&str>)
{
    print!("graphviz... ");
    let _ = std::io::stdout().flush();

    let mut graph = String::new();
    graph.push_str("digraph {\n  overlap=false\n");

    use Monkey::*;
    use Operation::*;
    for (name, monkey) in monkeys.iter()
    {
        let colour = match *name
        {
            "root" => "red",
            "humn" => "green",
            _ if to_recalc.contains(name) => "yellow",
            _ => "white"
        };

        graph.push_str(&match monkey
        {
            Literal(n)     => format!("  {name} [label=\"{name}:{n}\",style=\"filled\",fillcolor=\"{colour}\"]\n"),
            Expr(op, _, _) => format!("  {name} [label=\"{name}:{}\",style=\"filled\",fillcolor=\"{colour}\"]\n",
                                      match op { Add => "+",
                                                 Multiply => "*",
                                                 Subtract => "-",
                                                 Divide => "/" }),
        });
    }
    for (name, monkey) in monkeys.iter()
    {
        if let Expr(_op, a, b) = monkey
        {
            graph.push_str(&format!("  {} -> {}\n", name, a));
            graph.push_str(&format!("  {} -> {}\n", name, b));
        }
    }
    graph.push_str("}\n");
    let _ = std::fs::write("graph.dot", graph);
    println!("{:?}", std::process::Command::new("dot").args(["-Tpdf", "-O", "graph.dot"]).output());
}
