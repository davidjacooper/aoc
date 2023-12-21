use std::time::Instant;
use std::collections::{HashSet,HashMap};

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            if let Err(err) = step_counter_part2(&content)
            {
                println!("ERROR: {}", err);
            }
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

const TOTAL_STEPS: usize = 26501365;

fn step_counter_part2(content: &str) -> Result<(),&str>
{
    let mut start_i: i64 = 0;
    let mut start_j: i64 = 0;
    let grid: Vec<Vec<bool>> =
        content
        .trim_end()
        .lines()
        .enumerate()
        .map(|(i, line)|
            line
            .chars()
            .enumerate()
            .map(|(j, ch)|
            {
                if ch == 'S'
                {
                    start_i = i as i64;
                    start_j = j as i64;
                }
                ch != '#'
            })
            .collect()
        )
        .collect();

    let mut explored = HashMap::<(i64,i64),bool>::new();
    let mut current = HashSet::<(i64,i64)>::new();
    current.insert((start_i, start_j));

    let mut n_reachable_exactly = 0;
    explored.insert(
        (start_i, start_j),
        if TOTAL_STEPS % 2 == 0
        {
            n_reachable_exactly = 1;
            true
        }
        else
        {
            false
        });

    let initial_steps = (grid.len() - 1) / 2;
    let step_step = grid.len() * 2;

    let mut n_steps = initial_steps;
    let mut prev_n_steps = 0;

    // Compute values for solving quadratic equation:
    let mut points = Vec::<i64>::new();
    for _ in 0..3
    {
        run(prev_n_steps..n_steps,
            &grid,
            &mut explored,
            &mut current,
            &mut n_reachable_exactly);
        points.push(n_reachable_exactly);

        prev_n_steps = n_steps;
        n_steps += step_step;
    }

    // Solve quadratic equation:
    let a = points[2] / 2 - points[1] + points[0] / 2;
    let b = -points[2] / 2 + 2 * points[1] - 3 * points[0] / 2;
    let c = points[0];
    let n = ((TOTAL_STEPS - initial_steps) / step_step) as i64;

    let plots = a * n.pow(2) + b * n + c;
    println!("#plots = {plots}");

    Ok(())
}


fn run(steps: std::ops::Range<usize>,
       grid: &Vec<Vec<bool>>,
       explored: &mut HashMap<(i64,i64),bool>,
       current: &mut HashSet<(i64,i64)>,
       n_reachable_exactly: &mut i64)
{
    let height = grid.len() as i64;
    let width = grid[0].len() as i64;

    let mut next = HashSet::<(i64,i64)>::new();
    let n_steps = steps.end;
    for n in steps
    {
        let even = (TOTAL_STEPS - n) % 2 == 0;

        for (i, j) in current.drain()
        {
            for (i2, j2) in [(i - 1, j), (i, j + 1), (i + 1, j), (i, j - 1)]
            {
                if grid[i2.rem_euclid(height) as usize][j2.rem_euclid(width) as usize]
                    && !explored.contains_key(&(i2, j2))
                {
                    explored.insert(
                        (i2, j2),
                        if even
                        {
                            false
                        }
                        else
                        {
                            *n_reachable_exactly += 1;
                            true
                        });
                    next.insert((i2,j2));
                }
            }
        }

        for pos in next.drain()
        {
            current.insert(pos);
        }
    }
    print_grid(grid, explored);
    println!("#plots reachable after {} steps: {n_reachable_exactly}", n_steps);
}

fn print_grid(grid: &Vec<Vec<bool>>, explored: &HashMap<(i64,i64),bool>)
{
    let mut min_i = i64::MAX;
    let mut min_j = i64::MAX;
    let mut max_i = i64::MIN;
    let mut max_j = i64::MIN;
    for (i, j) in explored.keys()
    {
        min_i = min_i.min(*i);
        min_j = min_j.min(*j);
        max_i = max_i.max(*i);
        max_j = max_j.max(*j);
    }

    let height = grid.len() as i64;
    let width = grid[0].len() as i64;

    for i in (min_i - 2)..=(max_i + 2)
    {
        for j in (min_j - 2)..=(max_j + 2)
        {
            let grid_i = i.rem_euclid(height);
            let grid_j = j.rem_euclid(width);


            print!("{}{}\x1b[m",
                   if grid_i == 0 || grid_i == (height - 1) ||
                      grid_j == 0 || grid_j == (width - 1)
                   {
                       "\x1b[44m"
                   }
                   else if grid_i == 65 || grid_j == 65
                   {
                       "\x1b[42m"
                   }
                   else { "" },
                   match explored.get(&(i, j))
                   {
                       Some(true) => "\x1b[31;1mO",
                       Some(false) => "\x1b[31m.",
                       None => if grid[grid_i as usize][grid_j as usize] { "." } else { "#" }
                   });
        }
        println!();
    }
}
