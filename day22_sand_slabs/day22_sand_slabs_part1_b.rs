use std::time::Instant;
use std::collections::{HashSet};
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
            if let Err(err) = sand_slabs_part1(&content)
            {
                println!("ERROR: {}", err);
            }
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

/*
 * Here, we keep track of which bricks are directly resting on top of which other bricks. A given
 * brick is 'safe' to remove if all bricks directly above are also directly above at least one
 * other brick.
 *
 * Simplification of day22_sand_slabs_part1.rs. Same approach, but without needing to explicitly
 * distinguish between different 'types' of bricks.
 *
 * Also, we expose an easter egg by using the node naming scheme implied by the problem
 * description.
 */


#[derive(Ord,PartialOrd,Eq,PartialEq,Debug)]
struct Brick
{
    z1: usize,
    z2: usize,
    y1: usize,
    y2: usize,
    x1: usize,
    x2: usize,
    name: String,
}

#[derive(Copy,Clone,Debug)]
struct Position
{
    brick_i: usize,
    height: usize
}

fn sand_slabs_part1(content: &str) -> Result<(),&str>
{
    let mut max_x = 0;
    let mut max_y = 0;

    let mut bricks: Vec<Brick> =
        content
        .trim_end()
        .lines()
        .enumerate()
        .map(|(i,line)|
        {
            let dim: Vec<usize> =
                line
                .split(&[',', '~'])
                .map(|s| s.parse().unwrap())
                .collect();

            let brick = Brick
            {
                x1: dim[0].min(dim[3]),
                y1: dim[1].min(dim[4]),
                z1: dim[2].min(dim[5]),
                x2: dim[0].max(dim[3]),
                y2: dim[1].max(dim[4]),
                z2: dim[2].max(dim[5]),
                name: alphabetic_n(i)
            };

            max_x = max_x.max(brick.x2);
            max_y = max_y.max(brick.y2);

            brick
        })
        .collect();


    const FLOOR: usize = usize::MAX;
    let mut below = vec![HashSet::<usize>::new(); bricks.len()];
    let mut above = vec![HashSet::<usize>::new(); bricks.len()];

    bricks.sort();
    let mut positions = vec![vec![Position{brick_i: FLOOR, height: 0}; max_x + 1]; max_y + 1];

    for (i, brick) in bricks.iter_mut().enumerate()
    {
        let brick_height = brick.z2 - brick.z1;
        brick.z1 = 0;
        for x in brick.x1..=brick.x2
        {
            for y in brick.y1..=brick.y2
            {
                brick.z1 = brick.z1.max(positions[y][x].height + 1);
            }
        }
        brick.z2 = brick.z1 + brick_height;

        for x in brick.x1..=brick.x2
        {
            for y in brick.y1..=brick.y2
            {
                let pos = &mut positions[y][x];
                if pos.brick_i != FLOOR && pos.height + 1 == brick.z1
                {
                    below[i].insert(pos.brick_i);
                    above[pos.brick_i].insert(i);
                }
                pos.brick_i = i;
                pos.height = brick.z2;
            }
        }
    }

    let mut disintegratable = HashSet::<usize>::new();
    for (i, _brick) in bricks.iter().enumerate()
    {
        if above[i].iter().all(|j| below[*j].len() >= 2)
        {
            disintegratable.insert(i);
        }
    }

    make_graph(&bricks, &below, &disintegratable).unwrap();
    println!("#disintegratable = {}", disintegratable.len());

    Ok(())
}

fn alphabetic_n(mut n: usize) -> String
{
    let mut s = String::new();
    loop
    {
        s.insert(0, ((n % 26) as u8 + b'A') as char);
        if n < 26 { return s }
        n = (n / 26) - 1;
    }
}


fn make_graph(bricks: &Vec<Brick>,
              below: &Vec<HashSet<usize>>,
              disintegratable: &HashSet<usize>) -> std::io::Result<()>
{
    let mut graph = std::fs::File::create("graph2.dot")?;
    writeln!(graph, "digraph {{\n  overlap=false")?;
    for (i, brick) in bricks.iter().enumerate()
    {
        writeln!(graph,
                 "  b{} [label=\"{}\",shape=\"box\",fixedsize=true,width={},height={},fillcolor=\"{}\"{}]",
                 i,
                 brick.name.repeat(if brick.name == "HO" { brick.z2 - brick.z1 + 1 } else { 1 }),
                 ((brick.x2 - brick.x1 + 1).max(brick.y2 - brick.y1 + 1)) as f64 / 3.0,
                 (brick.z2 - brick.z1 + 1) as f64 / 3.0,
                 if brick.name == "HO"        { "lightsalmon" }
                 else if brick.x1 != brick.x2 { "yellow" }
                 else if brick.y1 != brick.y2 { "green" }
                 else                         { "lightskyblue" },
                 if disintegratable.contains(&i)
                 {
                     ",color=\"red\",style=\"filled,dashed\",penwidth=4"
                 }
                 else { ",style=\"filled\"" }
        )?;
    }

    for (i, indexes) in below.iter().enumerate()
    {
        for j in indexes.iter()
        {
            writeln!(graph, "  b{i} -> b{j}")?;
        }
    }
    writeln!(graph, "}}")?;
    println!("graphviz... {:?}", std::process::Command::new("dot").args(["-Tpdf", "-O", "graph2.dot"]).output());
    Ok(())
}
