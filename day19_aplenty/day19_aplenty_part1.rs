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
            aplenty_part1(&content);
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

impl Rule<'_>
{
    fn matches(&self, x: i64, m: i64, a: i64, s: i64) -> bool
    {
        let val = match self.var { Var::X => x, Var::M => m, Var::A => a, Var::S => s };
        return match self.rel
        {
            Rel::Greater => val > self.amount,
            Rel::Less    => val < self.amount
        }
    }
}


#[derive(Debug)]
struct Workflow<'a>
{
    rules: Vec<Rule<'a>>,
    dest: Dest<'a>
}


fn aplenty_part1(content: &str)
{
    let (workflows_str, ratings_str) = content.trim_end().split_once("\n\n").unwrap();

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

    let mut sum = 0;
    for line in ratings_str.lines()
    {
        let values = line[1..(line.len() - 1)]
            .split(',')
            .map(|s| s[2..].parse().unwrap())
            .collect::<Vec<i64>>();

        let [x, m, a, s] = &values[..] else { panic!() };
        print!("x={x}, m={m}, a={a}, s={s}: ");

        let mut workflow = workflows.get("in").unwrap();

        loop
        {
            match workflow.rules
                .iter()
                .filter_map(|rule|
                    if rule.matches(*x, *m, *a, *s) { Some(rule.dest) }
                    else                            { None }
                )
                .nth(0)
                .unwrap_or(workflow.dest)
            {
                Dest::Accept =>
                {
                    println!("Accepted");
                    sum += x + m + a + s;
                    break;
                }
                Dest::Reject =>
                {
                    println!("Rejected");
                    break;
                }
                Dest::Goto(name) =>
                {
                    workflow = workflows.get(name).unwrap();
                }
            }
        }
    }

    println!("sum = {sum}");
}

