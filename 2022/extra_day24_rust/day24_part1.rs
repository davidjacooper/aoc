use std::time::Instant;
use std::collections::HashSet;

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input1.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            blizzard_basin_part1(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

#[derive(Copy,Clone,Debug)]
struct Blizzard
{
    i: usize,
    j: usize,
    delta_i: i8,
    delta_j: i8,
}

fn blizzard_basin_part1(mut content: &str)
{
    content = content.trim_end();
    let width = content.find('\n').unwrap();
    let height = content.lines().count();

    let mut blizzards: Vec<Blizzard> =
        content
        .lines()
        .enumerate()
        .flat_map(|(i, row)|
            row
            .chars()
            .enumerate()
            .filter_map(move |(j, ch)|
            {
                return match ch
                {
                    '^' => Some(Blizzard{i: i, j: j, delta_i: -1, delta_j: 0}),
                    '>' => Some(Blizzard{i: i, j: j, delta_i: 0,  delta_j: 1}),
                    'v' => Some(Blizzard{i: i, j: j, delta_i: 1,  delta_j: 0}),
                    '<' => Some(Blizzard{i: i, j: j, delta_i: 0,  delta_j: -1}),
                    _ => None
                }
            })
        )
        .collect();

    let mut grid: Vec<Vec<u8>> = vec![vec![0; width as usize]; height as usize];

    update_grid(&mut grid, &blizzards);

    let mut expeditions = HashSet::<(usize,usize)>::new();
    let mut next_expeditions = HashSet::<(usize,usize)>::new();
    expeditions.insert((0, 1));
    print_valley(&grid, &expeditions, width, height);

    let mut time = 1;
    let mut finished = false;
    while !finished
    {
        println!("\nMinute {time}:");
        move_blizzards(&mut blizzards, width, height);
        update_grid(&mut grid, &blizzards);

        for (ex_i, ex_j) in expeditions.drain()
        {
            if grid[ex_i][ex_j] == 0
            {
                next_expeditions.insert((ex_i, ex_j));
            }

            if ex_i > 0 && grid[ex_i - 1][ex_j] == 0
            {
                next_expeditions.insert((ex_i - 1, ex_j));
            }

            if grid[ex_i + 1][ex_j] == 0
            {
                next_expeditions.insert((ex_i + 1, ex_j));
                finished |= ex_i == height - 2;
            }

            if grid[ex_i][ex_j - 1] == 0
            {
                next_expeditions.insert((ex_i, ex_j - 1));
            }

            if grid[ex_i][ex_j + 1] == 0
            {
                next_expeditions.insert((ex_i, ex_j + 1));
            }
        }

        for ex in next_expeditions.drain()
        {
            expeditions.insert(ex);
        }

        print_valley(&grid, &expeditions, width, height);
        time += 1;
    }
}

fn move_blizzards(blizzards: &mut Vec<Blizzard>, width: usize, height: usize)
{
    for b in blizzards.iter_mut()
    {
        b.i = b.i.checked_add_signed(b.delta_i as isize).unwrap();
        if b.i == 0 { b.i = height - 2 }
        if b.i == (height - 1) { b.i = 1 }

        b.j = b.j.checked_add_signed(b.delta_j as isize).unwrap();
        if b.j == 0 { b.j = width - 2 }
        if b.j == (width - 1) { b.j = 1 }
    }
}

fn update_grid(grid: &mut Vec<Vec<u8>>, blizzards: &Vec<Blizzard>)
{
    let height = grid.len();
    let width = grid[0].len();

    for row in grid.iter_mut()
    {
        row.fill(0);
        row[0] = 0xff;
        row[width - 1] = 0xff;
    }
    grid[0].fill(0xff);
    grid[height - 1].fill(0xff);

    // Start and end positions
    grid[0][1] = 0;
    grid[height - 1][width - 2] = 0;

    for Blizzard{i, j, ..} in blizzards.iter()
    {
        grid[*i][*j] += 1;
    }
}


fn print_valley(grid: &Vec<Vec<u8>>, expeditions: &HashSet::<(usize,usize)>, width: usize, height: usize)
{
    for i in 0..height
    {
        for j in 0..width
        {
            let n = grid[i][j];
            if expeditions.contains(&(i, j))
            {
                print!("\x1b[{}mE\x1b[m", if n == 0 { 41 } else { 45 });
            }
            else
            {
                match n
                {
                    0    => print!("."),
                    0xff => print!("#"),
                    _    => print!("\x1b[{}m{}\x1b[m",
                                match n { 1 => 44, 2 => 46, 3 => 42, _ => 47 },
                                n)
                }
            }
        }
        println!();
    }
}
