use std::time::Instant;
use std::collections::{HashSet};

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

    let mut n_disintegratable = 0;
    for (i, _brick) in bricks.iter().enumerate()
    {
        if above[i].iter().all(|j| below[*j].len() >= 2)
        {
            n_disintegratable += 1;
        }
    }

    println!("#disintegratable = {n_disintegratable}");


    Ok(())
}
