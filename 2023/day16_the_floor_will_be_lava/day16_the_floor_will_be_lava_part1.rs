use std::time::Instant;
use std::collections::{HashSet,VecDeque};

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input1.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            the_floor_will_be_lava_part1(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

#[derive(Copy,Clone,Debug)]
struct Tile {
    obj: char,
    energised: bool
}

#[derive(Copy,Clone,Hash,Eq,PartialEq,Debug)]
enum Direction { Up, Right, Down, Left }
use Direction::*;

#[derive(Copy,Clone,Hash,Eq,PartialEq,Debug)]
struct State {
    i: usize,
    j: usize,
    dir: Direction
}

impl State
{
    fn move_one(&self) -> State
    {
        return match &self.dir
        {
            Up    => State { i: self.i - 1, ..*self },
            Right => State { j: self.j + 1, ..*self },
            Down  => State { i: self.i + 1, ..*self },
            Left  => State { j: self.j - 1, ..*self },
        };
    }

    fn turn_left(&self) -> State
    {
        return State
        {
            dir: match self.dir { Right=>Up, Down=>Right, Left=>Down, Up=>Left },
            ..*self
        }
    }

    fn turn_right(&self) -> State
    {
        return State
        {
            dir: match self.dir { Right=>Down, Down=>Left, Left=>Up, Up=>Right },
            ..*self
        }
    }
}

fn the_floor_will_be_lava_part1(content: &str)
{
    let edge_tile = Tile { obj: '#', energised: false };
    let mut grid = content.trim_end()
                          .lines()
                          .map(|l| {
                              let mut row = l.chars()
                                             .map(|c| Tile { obj: c,
                                                             energised: false })
                                             .collect::<Vec<Tile>>();
                              row.insert(0, edge_tile);
                              row.push(edge_tile);
                              return row;
                          })
                          .collect::<Vec<Vec<Tile>>>();

    grid.insert(0, vec![edge_tile; grid[0].len()]);
    grid.push(vec![edge_tile; grid[0].len()]);

    print_grid(&grid);

    let mut n_energised = 0;
    let mut seen = HashSet::<State>::new();
    let mut queue = VecDeque::<State>::new();
    queue.push_back(State { i: 1, j: 1, dir: Right });

    while let Some(state) = queue.pop_front()
    {
        if seen.contains(&state) { continue; }

        let State { i, j, dir } = state;
        if !grid[i][j].energised
        {
            grid[i][j].energised = true;
            n_energised += 1;
        }

        seen.insert(state);

        match (grid[i][j].obj, dir)
        {
            ('#', _) =>
            {
                n_energised -= 1; // Doesn't count.
            }
            ('.', _) | ('-', Left | Right) | ('|', Up | Down) =>
            {
                queue.push_back(state.move_one());
            }
            ('-', _) | ('|', _) =>
            {
                queue.push_back(state.turn_left().move_one());
                queue.push_back(state.turn_right().move_one());
            }
            ('/', Left | Right) | ('\\', Up | Down) =>
            {
                queue.push_back(state.turn_left().move_one());
            }
            ('/', Up | Down) | ('\\', Left | Right) =>
            {
                queue.push_back(state.turn_right().move_one());
            }
            _ => panic!()
        }
    }

    print_grid(&grid);
    println!("\n#energised = {n_energised}");
}

fn print_grid(grid: &Vec<Vec<Tile>>)
{
    println!();
    for row in grid.iter()
    {
        for tile in row.iter()
        {
            if tile.energised
            {
                print!("\x1b[44m{}\x1b[m", tile.obj);
            }
            else
            {
                print!("{}", tile.obj);
            }
        }
        println!();
    }
}

