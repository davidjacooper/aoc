/* Fun facts
 *
 * The input dimensions are 131 x 131, with the starting position dead centre.
 * (There are 65 squares up, down, left and right of the start.)
 *
 * There are no rocks at all in either the row or column of the starting position.
 *
 * When we try to fill adjacent copies of the board, the fill pattern will be reversed, because the dimensions are odd.
 *
 * The origin board can be completely filled with 7481 plots. It _could_ be completely filled with 7407 plots if
 * the pattern were reversed.
 *
 * With only 64 steps (the inner diamond):
 * - we get 3666 vs 3489
 *
 * 26501365 - 65 evenly divides 131 (by 2 * 2 * 5 * 5 * 7 * 17 * 17).
 *
 * => after 26501365 steps, the area spans (26501365 - 65) / 131 = 202300 extra boards in each direction,
 * and the total vertical/horizontal span is thus (202300 * 2 + 1) = 404601.
 *
 * Also, the
 *
 * Taking a smaller example:
 * with 196 steps (65 + 131), we ought to reach the outer edges of the four boards surrounding the origin board.
 *
 *
 */



use std::time::Instant;
use std::collections::{HashSet,HashMap};
// use std::cmp::{min,max};

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
    let mut start_i: i64 = 0;
    let mut start_j: i64 = 0;

    let grid: Vec<Vec<Square>> =
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
                        start_i = i as i64;
                        start_j = j as i64;
                    }
                    if ch != '#' { Passable } else { Rock }
                })
                .collect();
            // row.insert(0, Rock);
            // row.push(Rock);
            row
        })
        .collect();

    let height = grid.len() as i64;
    let width = grid[0].len() as i64;

    // grid.insert(0, vec![Rock; width]);
    // grid.push(vec![Rock; width]);

    // (start_i, start_j) = (0, 0);
    const MAX_STEPS: usize = 131;



    // Account for boundary.
    start_i += 1;
    start_j += 1;

    println!("start=({start_i},{start_j})");

    let mut seen = HashMap::<(i64,i64),bool>::new();
    let mut current = HashSet::<(i64,i64)>::new();
    let mut next = HashSet::<(i64,i64)>::new();
    current.insert((start_i, start_j));

    let mut n_reachable_exactly = 0;
    let mut n_reachable_partially = 0;
    seen.insert(
        (start_i, start_j),
        if MAX_STEPS % 2 == 0
        {
            n_reachable_exactly = 1;
            true
        }
        else
        {
            n_reachable_partially = 1;
            false
        });


    for n_steps in 0..MAX_STEPS
    {
        let even = (MAX_STEPS - n_steps) % 2 == 0;

        for (i, j) in current.drain()
        {
            for (i2, j2) in [(i - 1, j), (i, j + 1), (i + 1, j), (i, j - 1)]
            {
                if grid[i2.rem_euclid(height) as usize]
                       [j2.rem_euclid(width) as usize] == Passable &&
                    !seen.contains_key(&(i2, j2))
                {
                    seen.insert(
                        (i2, j2),
                        if even
                        {
                            n_reachable_partially += 1;
                            false
                        }
                        else
                        {
                            n_reachable_exactly += 1;
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

    print_grid(&grid, &seen);
    println!("#plots reachable in exactly {MAX_STEPS} steps = {n_reachable_exactly}");
    println!("#other plots reachable in fewer steps = {n_reachable_partially}");

    Ok(())
}


fn print_grid(grid: &Vec<Vec<Square>>, seen: &HashMap<(i64,i64),bool>)
{
    let mut min_i = i64::MAX;
    let mut min_j = i64::MAX;
    let mut max_i = i64::MIN;
    let mut max_j = i64::MIN;
    for (i, j) in seen.keys()
    {
        min_i = min_i.min(*i);
        min_j = min_j.min(*j);
        max_i = max_i.max(*i);
        max_j = max_j.max(*j);
    }

    let height = grid.len() as i64;
    let width = grid[0].len() as i64;

    for i in (min_i - 1)..=(max_i + 1)
    {
        for j in (min_j - 1)..=(max_j + 1)
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
                   match seen.get(&(i, j))
                   {
                       Some(true) => "\x1b[31;1mO",
                       Some(false) => "\x1b[31m.",
                       None if grid[grid_i as usize][grid_j as usize] == Rock => "#",
                       None                                 => "."
                   });
        }
        println!();
    }
}
