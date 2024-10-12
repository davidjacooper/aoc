use std::time::Instant;
use std::collections::{HashSet,BTreeSet};

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
 * Here, we keep track of which bricks are directly resting on top of which other bricks. To find
 * how many bricks fall when brick i is removed, we take away that brick from each of bricks above
 * it, and check whether any of them are now above 0 bricks. If they are, then they fall, we do
 * count++, and we repeat for each of the other falling bricks, in Z order (which also happens to
 * be index order).
 *
 * However, this is only very slightly faster than solution 2_a, where we simply re-do the whole
 * simulation for each brick.
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
                        if pos.height + 1 == brick.z
                        {
                            below[i].insert(pos.brick_i);
                            if pos.brick_i != FLOOR
                            {
                                above[pos.brick_i].insert(i);
                            }
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
                        if pos.height + 1 == brick.z
                        {
                            below[i].insert(pos.brick_i);
                            if pos.brick_i != FLOOR
                            {
                                above[pos.brick_i].insert(i);
                            }
                        }
                        pos.brick_i = i;
                        pos.height = brick.z;
                    });
            }

            Orientation::Z =>
            {
                brick.z = 1 + positions[brick.y][brick.x].height;
                let pos = &mut positions[brick.y][brick.x];
                below[i].insert(pos.brick_i);
                if pos.brick_i != FLOOR
                {
                    above[pos.brick_i].insert(i);
                }
                pos.brick_i = i;
                pos.height += brick.len;
            }
        }

        // println!("  {:?}", brick);
    }

    let mut sum = 0;
    let mut to_check = BTreeSet::<usize>::new();
    for (missing_brick_i, _brick) in bricks.iter().enumerate()
    {
        // print!("With brick {missing_brick_i} missing... ");

        let mut n_fall = 0;
        let mut test_below = below.clone();

        to_check.insert(missing_brick_i);
        while let Some(j) = to_check.pop_first()
        {
            // j is a brick known to be removed: either missing_brick_i itself, or a consequence thereof.
            // k is a brick on top of j, which _may_ fall.
            for k in above[j].iter()
            {
                test_below[*k].remove(&j);
                if test_below[*k].len() == 0
                {
                    n_fall += 1;
                    to_check.insert(*k);
                }
            }
        }

        sum += n_fall;
        // println!("{n_fall} others fall");
    }

    println!("sum = {sum}");

    Ok(())
}
