use std::time::Instant;
use std::collections::HashMap;
use std::cmp::{min,max};

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input1.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            aplenty_part2(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

#[derive(Debug)]
enum Var { X, M, A, S }

impl Var
{
    fn from(ch: char) -> Var
    {
        return match ch
        {
            'x' => Var::X,
            'm' => Var::M,
            'a' => Var::A,
            's' => Var::S,
            _ => panic!()
        }
    }
}

#[derive(Debug)]
enum Rel { Greater, Less }

impl Rel
{
    fn from(ch: char) -> Rel
    {
        return match ch
        {
            '>' => Rel::Greater,
            '<' => Rel::Less,
            _ => panic!()
        }
    }
}


#[derive(Copy,Clone,Debug)]
enum Dest<'a>
{
    Accept,
    Reject,
    Goto(&'a str)
}

impl Dest<'_>
{
    fn from(s: &str) -> Dest
    {
        return match s
        {
            "A" => Dest::Accept,
            "R" => Dest::Reject,
            _   => Dest::Goto(s)
        }
    }
}

#[derive(Debug)]
struct Rule<'a>
{
    var: Var,
    rel: Rel,
    amount: i64,
    dest: Dest<'a>
}

#[derive(Debug)]
struct Workflow<'a>
{
    rules: Vec<Rule<'a>>,
    dest: Dest<'a>
}


fn aplenty_part2(content: &str)
{
    let (workflows_str, _) = content.trim_end().split_once("\n\n").unwrap();

    let workflows: HashMap<&str,Workflow> =
        workflows_str
        .lines()
        .map(|line| {
            let (name, rule_str) = line.split_once('{').unwrap();
            let (cond_rule_str, dest) = rule_str.rsplit_once(',').unwrap();

            return (
                name,
                Workflow {
                    dest: Dest::from(&dest[0..(dest.len() - 1)]),
                    rules:
                        cond_rule_str
                        .split(',')
                        .map(|s| {
                            let (cond_str, dest) = s.split_once(':').unwrap();
                            let mut chars = cond_str.chars();

                            return Rule {
                                var: Var::from(chars.next().unwrap()),
                                rel: Rel::from(chars.next().unwrap()),
                                amount: cond_str[2..].parse().unwrap(),
                                dest: Dest::from(dest)
                            }
                        })
                        .collect()
                }
            )
        })
        .collect();

    let n_combinations = find_combinations(
        &workflows, Dest::Goto("in"), [(1, 4000), (1, 4000), (1, 4000), (1, 4000)]);

    println!("#combinations = {n_combinations}");

}


fn find_combinations(workflows: &HashMap<&str,Workflow>,
                     dest: Dest,
                     mut values: [(i64,i64);4]) -> i64
{
    if values.iter().any(|(a, b)| a > b)
    {
        return 0;
    }

    return match dest
    {
        Dest::Accept => values.iter().map(|(a, b)| b - a + 1).product(),
        Dest::Reject => 0,
        Dest::Goto(name) =>
        {
            let workflow = workflows.get(name).unwrap();
            let mut combos = 0;

            for rule in workflow.rules.iter()
            {
                let mut new_values = values.clone();
                let index = match rule.var { Var::X => 0,
                                             Var::M => 1,
                                             Var::A => 2,
                                             Var::S => 3 };

                match rule.rel
                {
                    Rel::Greater => {
                        new_values[index].0 = max(new_values[index].0, rule.amount + 1);
                        values[index].1     = min(values[index].1,     rule.amount);
                    }
                    Rel::Less => {
                        new_values[index].1 = min(new_values[index].1, rule.amount - 1);
                        values[index].0     = max(values[index].0,     rule.amount);
                    }
                }

                combos += find_combinations(workflows, rule.dest, new_values);
            }

            combos + find_combinations(workflows, workflow.dest, values)
        }
    }
}
