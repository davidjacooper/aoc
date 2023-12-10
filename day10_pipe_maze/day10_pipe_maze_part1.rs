use std::time::Instant;
use std::collections::HashSet;
// use std::iter::FromIterator;

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input1.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            pipe_maze_part1(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

enum Dir { Up, Down, Left, Right }

fn pipe_maze_part1(content: &str)
{
    let mut grid: Vec<Vec<char>> =
        content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line|
        {
            let mut v: Vec<char> = line.chars().collect();
            v.insert(0, '.');
            v.push('.');
            return v;
        })
        .collect();

    grid.insert(0, vec!['.'; grid[0].len()]);
    grid.push(vec!['.'; grid[0].len()]);

    let (start_i, start_j) =
        grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().position(|c| *c == 'S').map(|j| (i, j)))
        .nth(0)
        .unwrap();
    let mut path = vec![(start_i, start_j)];

    use Dir::*;
    let (mut i, mut j, mut dir) =
        if      "|7F".contains(grid[start_i - 1][start_j]) { (start_i - 1, start_j, Up) }
        else if "|JL".contains(grid[start_i + 1][start_j]) { (start_i + 1, start_j, Down) }
        else if "-7J".contains(grid[start_i][start_j - 1]) { (start_i, start_j - 1, Left) }
        else if "-FL".contains(grid[start_i][start_j + 1]) { (start_i, start_j + 1, Right) }
        else { panic!() };

    while (i, j) != (start_i, start_j)
    {
        path.push((i, j));
        (i, j, dir) = match (grid[i][j], dir)
        {
            ('|', Up)    | ('J', Right) | ('L', Left) => (i - 1, j, Up),
            ('|', Down)  | ('7', Right) | ('F', Left) => (i + 1, j, Down),
            ('-', Left)  | ('7', Up)    | ('J', Down) => (i, j - 1, Left),
            ('-', Right) | ('F', Up)    | ('L', Down) => (i, j + 1, Right),
            _ => panic!()
        };
    }

    let path_set: HashSet<&(usize, usize)> = path.iter().collect();
    for (i, row) in grid.iter().enumerate()
    {
        for (j, ch) in row.iter().enumerate()
        {
            if (i, j) == (start_i, start_j)
            {
                print!("\x1b[41;1m{}\x1b[m", ch);
            }
            else if path_set.contains(&(i, j))
            {
                print!("\x1b[44;1m{}\x1b[m", ch);
            }
            else
            {
                print!("{ch}");
            }
        }
        println!();
    }

    println!("length = {}; half-way = {}", path_set.len(), path_set.len() / 2);
}

