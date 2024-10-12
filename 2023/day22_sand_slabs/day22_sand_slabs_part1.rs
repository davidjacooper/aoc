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
 */


#[derive(Ord,PartialOrd,Eq,PartialEq,Debug)]
enum Orientation { X, Y, Z }

#[derive(Ord,PartialOrd,Eq,PartialEq,Debug)]
struct Brick
{
    z: usize,
    y: usize,
    x: usize,
    len: usize,
    orientation: Orientation,
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
        .map(|line|
        {
            let p: Vec<usize> =
                line
                .split(&[',', '~'])
                .map(|s| s.parse().unwrap())
                .collect();

            let [x1, y1, z1, x2, y2, z2] = p.as_slice() else { panic!() };

            let (len, orientation) =
                if x1 != x2      { (x1.abs_diff(*x2) + 1, Orientation::X) }
                else if y1 != y2 { (y1.abs_diff(*y2) + 1, Orientation::Y) }
                else             { (z1.abs_diff(*z2) + 1, Orientation::Z) };

            max_x = max_x.max(*x1).max(*x2);
            max_y = max_y.max(*y1).max(*y2);

            Brick
            {
                x: *x1.min(x2),
                y: *y1.min(y2),
                z: *z1.min(z2),
                len: len,
                orientation: orientation
            }
        })
        .collect();


    const FLOOR: usize = usize::MAX;
    let mut below = vec![HashSet::<usize>::new(); bricks.len()];
    let mut above = vec![HashSet::<usize>::new(); bricks.len()];

    bricks.sort();
    let mut positions = vec![vec![Position{brick_i: FLOOR, height: 0}; max_x + 1]; max_y + 1];

    for (i, brick) in bricks.iter_mut().enumerate()
    {
        match &brick.orientation
        {
            Orientation::X =>
            {
                brick.z = 1 +
                    (brick.x..(brick.x + brick.len))
                    .fold(0, |h, x| h.max(positions[brick.y][x].height));

                (brick.x..(brick.x + brick.len))
                    .for_each(|x|
                    {
                        let pos = &mut positions[brick.y][x];
                        if pos.brick_i != FLOOR && pos.height + 1 == brick.z
                        {
                            below[i].insert(pos.brick_i);
                            above[pos.brick_i].insert(i);
                        }
                        pos.brick_i = i;
                        pos.height = brick.z;
                    });
            }

            Orientation::Y =>
            {
                brick.z = 1 +
                    (brick.y..(brick.y + brick.len))
                    .fold(0, |h, y| h.max(positions[y][brick.x].height));

                (brick.y..(brick.y + brick.len))
                    .for_each(|y|
                    {
                        let pos = &mut positions[y][brick.x];
                        if pos.brick_i != FLOOR && pos.height + 1 == brick.z
                        {
                            below[i].insert(pos.brick_i);
                            above[pos.brick_i].insert(i);
                        }
                        pos.brick_i = i;
                        pos.height = brick.z;
                    });
            }

            Orientation::Z =>
            {
                brick.z = 1 + positions[brick.y][brick.x].height;
                let pos = &mut positions[brick.y][brick.x];
                if pos.brick_i != FLOOR
                {
                    below[i].insert(pos.brick_i);
                    above[pos.brick_i].insert(i);
                }
                pos.brick_i = i;
                pos.height += brick.len;
            }
        }

        println!("  {:?}", brick);
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


fn make_graph(bricks: &Vec<Brick>,
              below: &Vec<HashSet<usize>>,
              disintegratable: &HashSet<usize>) -> std::io::Result<()>
{
    let mut graph = std::fs::File::create("graph.dot")?;
    writeln!(graph, "digraph {{\n  overlap=false")?;
    for (i, brick) in bricks.iter().enumerate()
    {
        writeln!(graph,
                 "  b{} [label=\"{}\",shape=\"box\",fixedsize=true,width={},height={},fillcolor=\"{}\"{}]",
                 i,
                 brick.z,
                 match brick.orientation { Orientation::X => brick.len,
                                           Orientation::Y => brick.len,
                                           Orientation::Z => 1 } as f64 / 3.0,
                 match brick.orientation { Orientation::Z => brick.len, _ => 1 } as f64 / 3.0,
                 match brick.orientation { Orientation::X => "lightsalmon",
                                           Orientation::Y => "green",
                                           Orientation::Z => "lightskyblue" },
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
    println!("graphviz... {:?}", std::process::Command::new("dot").args(["-Tpdf", "-O", "graph.dot"]).output());
    Ok(())
}
