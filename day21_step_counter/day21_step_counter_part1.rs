use std::time::Instant;
use std::collections::HashSet;


fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            if let Err(err) = step_counter_part1(&content)
            {
                println!("ERROR: {}", err);
            }
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
enum Square { Rock, Passable, PartiallyReachable, ExactlyReachable }
use Square::*;


fn step_counter_part1(content: &str) -> Result<(), &str>
{
    let mut start_i: usize = 0;
    let mut start_j: usize = 0;

    let mut grid: Vec<Vec<Square>> =
        content
        .trim_end()
        .lines()
        .enumerate()
        .map(|(i, line)|
        {
            let mut row: Vec<Square> =
                line
                .chars()
                .enumerate()
                .map(|(j, ch)|
                {
                    if ch == 'S'
                    {
                        start_i = i + 1;
                        start_j = j + 1;
                    }
                    if ch != '#' { Passable } else { Rock }
                })
                .collect();
            row.insert(0, Rock);
            row.push(Rock);
            row
        })
        .collect();

    let width = grid[0].len();
    grid.insert(0, vec![Rock; width]);
    grid.push(vec![Rock; width]);

    println!("start=({start_i},{start_j})");

    let mut n_reachable_exactly = 1;
    grid[start_i][start_j] = ExactlyReachable;

    let mut current = HashSet::<(usize,usize)>::new();
    let mut next = HashSet::<(usize,usize)>::new();
    current.insert((start_i, start_j));

    const MAX_STEPS: usize = 64;
    for n_steps in 0..MAX_STEPS
    {
        let even = n_steps % 2 == 0;

        for (i, j) in current.drain()
        {
            for (i2, j2) in [(i - 1, j), (i, j + 1), (i + 1, j), (i, j - 1)]
            {
                if grid[i2][j2] == Passable
                {
                    grid[i2][j2] = if even { PartiallyReachable } else { n_reachable_exactly += 1;
                                                                         ExactlyReachable };
                    next.insert((i2,j2));
                }
            }
        }

        for pos in next.drain()
        {
            current.insert(pos);
        }
    }

    print_grid(&grid);
    println!("#garden plots reachable in exactly {MAX_STEPS} steps = {n_reachable_exactly}");

    Ok(())
}


fn print_grid(grid: &Vec<Vec<Square>>)
{
    for row in grid.iter()
    {
        for square in row
        {
            print!("{}", match square { Rock                          => "#",
                                        Passable                      => ".",
                                        PartiallyReachable            => "o",
                                        ExactlyReachable              => "\x1b[31;1mO\x1b[m" });
        }
        println!();
    }
}
