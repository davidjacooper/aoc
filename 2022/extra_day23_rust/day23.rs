use std::time::Instant;
use std::collections::{HashMap,HashSet}    ;
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
            unstable_diffusion(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

#[derive(Copy,Clone,Hash,Eq,PartialEq,Debug)]
struct Point(i64, i64);

impl std::ops::Add<Point> for Point
{
    type Output = Point;

    fn add(self, rhs: Point) -> Point
    {
        return Point(self.0 + rhs.0, self.1 + rhs.1);
    }
}

const N : Point = Point(-1, 0);
const NE: Point = Point(-1, 1);
const E : Point = Point(0, 1);
const SE: Point = Point(1, 1);
const S : Point = Point(1, 0);
const SW: Point = Point(1, -1);
const W : Point = Point(0, -1);
const NW: Point = Point(-1, -1);

fn unstable_diffusion(content: &str)
{
    let mut elves: HashSet<Point> = content
        .trim_end()
        .lines()
        .enumerate()
        .flat_map(|(i, row)| row
            .chars()
            .enumerate()
            .filter_map(move |(j, ch)| if ch == '#' { Some(Point(i as i64, j as i64)) } else { None })
        )
        .collect();

    let mut proposed = HashMap::<Point,bool>::new();
    let mut move_map = HashMap::<Point,Point>::new();
    let mut next_elves = HashSet::<Point>::new();

    let (mut min_i, mut min_j, mut max_i, mut max_j);

    let checks = [(N, [N, NE, NW]),
                  (S, [S, SE, SW]),
                  (W, [W, NW, SW]),
                  (E, [E, NE, SE])];

    let mut round_n = 0;
    let mut finished = false;
    while !finished
    {
        round_n += 1;
        proposed.clear();
        move_map.clear();

        finished = true; // tentatively
        for pos in elves.iter()
        {
            let next_pos =
                if !elves_at(&elves, pos, &[N, NE, E, SE, S, SW, W, NW])
                {
                    *pos
                }
                else if let Some(dir) = (0..checks.len())
                                        .map(|n| checks[(round_n - 1 + n) % 4])
                                        .find(|(_, check_dirs)|
                                            !elves_at(&elves, pos, check_dirs))
                                        .map(|(dir, _)| dir)
                {
                    finished = false;
                    *pos + dir
                }
                else
                {
                    *pos // Not explicitly specified?
                };

            move_map.insert(*pos, next_pos);
            proposed.insert(next_pos, proposed.get(&next_pos) == None);
        }

        min_i = i64::MAX;
        min_j = i64::MAX;
        max_i = i64::MIN;
        max_j = i64::MIN;

        for pos in elves.drain()
        {
            let maybe_next_pos = move_map.get(&pos).unwrap();
            let next_pos = if *proposed.get(maybe_next_pos).unwrap() { maybe_next_pos } else { &pos };

            min_i = min(min_i, next_pos.0);
            min_j = min(min_j, next_pos.1);
            max_i = max(max_i, next_pos.0);
            max_j = max(max_j, next_pos.1);
            next_elves.insert(*next_pos);
        }

        (elves, next_elves) = (next_elves, elves);

        let space = (max_i + 1 - min_i) * (max_j + 1 - min_j) - elves.len() as i64;

        if round_n == 10 || finished
        {
            println!("---");
            print_elves(&elves, min_i, min_j, max_i, max_j);
            println!("after round {round_n}, space = {space}");
        }
    }
}

fn elves_at(elves: &HashSet<Point>, elf: &Point, directions: &[Point]) -> bool
{
    return directions.iter().any(|delta| elves.contains(&(*elf + *delta)));
}

fn print_elves(elves: &HashSet<Point>, i1: i64, j1: i64, i2: i64, j2: i64)
{
    for i in (i1 - 1)..=(i2 + 1)
    {
        for j in (j1 - 1)..=(j2 + 1)
        {
            print!("{}", if elves.contains(&Point(i, j)) { '#' } else { '.' });
        }
        println!();
    }
}
