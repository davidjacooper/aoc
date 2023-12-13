use std::time::Instant;
use std::collections::{HashSet,HashMap};

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input1.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            monkey_map_part2(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

#[derive(Copy,Clone,Eq,PartialEq,Hash,Debug)]
enum Direction { Right, Down, Left, Up }

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
enum Turn { TurnRight, TurnLeft }


fn monkey_map_part2(content: &str)
{
    let (board_str, step_str) = content.split_once("\n\n").unwrap();
    let width = board_str.lines().map(|l| l.len()).max().unwrap();

    let mut board: Vec<Vec<char>> = board_str
        .lines()
        .map(|line|
        {
            let mut row: Vec<char> = line.chars().collect();
            row.insert(0, ' ');
            for _ in 0..=(width - line.len())
            {
                row.push(' ');
            }
            return row;
        })
        .collect();

    board.insert(0, vec![' '; width + 2]);
    board.push(vec![' '; width + 2]);


    let mut i = 1;
    let mut j = board[1].iter().position(|c| *c == '.').unwrap();
    let mut dir = Direction::Right;

    let perimeter = find_perimeter(&board, i, j, dir);
    print_board(&board, &perimeter);
    let p_map = perimeter_map(perimeter);

    use Direction::*;

    for step in step_str.trim().split_inclusive(&['R', 'L'])
    {
        let turn_ch = step.chars().last().unwrap();
        let mut dist = step.trim_end_matches(&['R', 'L']).parse::<usize>().unwrap();

        //print!("{dist}-{turn_ch}, from ({i},{j}:{:?}) to... ", dir);
        while dist > 0
        {
            let (mut i2, mut j2) = move_n(i, j, 1, dir);
            let mut dir2 = dir;

            if board[i2][j2] == ' '
            {
                (i2, j2, dir2) = p_map.get(&(i2, j2, dir2)).unwrap().clone();
            }

            if board[i2][j2] == '#'
            {
                break;
            }

            (i, j, dir) = (i2, j2, dir2);
            dist -= 1;
        }

        match turn_ch
        {
            'R' => dir = turn(dir, Turn::TurnRight),
            'L' => dir = turn(dir, Turn::TurnLeft),
            _ => {}
        }
        //println!("**({i},{j}:{:?})**", dir);
    }

    // 1-based row/col numbering (in the prob statement) cancels out padding rows and cols.
    println!("Final position: ({i},{j},{:?})", dir);
    println!("Password = {}", 1000 * i + 4 * j + match dir { Right => 0, Down => 1, Left => 2, Up => 3 });

}

fn print_board(board: &Vec<Vec<char>>, perimeter: &Vec<(usize,usize,Direction)>)
{
    let perim_set = perimeter
        .iter()
        .map(|(i,j,dir)| move_n(*i, *j, 1, *dir))
        .collect::<HashSet<(usize,usize)>>();

    print!("      ");
    for j in 0..board[0].len()
    {
        print!("{}", j % 10);
    }
    println!();

    for (i, row) in board.iter().enumerate()
    {
        print!("{:>4} [", i);
        for (j, ch) in row.iter().enumerate()
        {
            if perim_set.contains(&(i,j))
            {
                print!("\x1b[44m{ch}\x1b[m");
            }
            else
            {
                print!("{ch}");
            }
        }
        println!("]");
    }
}

fn move_n(i: usize, j: usize, n: isize, dir: Direction) -> (usize,usize)
{
    use Direction::*;
    return match dir
    {
        Right => (i, j.wrapping_add_signed(n)),
        Down  => (i.wrapping_add_signed(n), j),
        Left  => (i, j.wrapping_add_signed(-n)),
        Up    => (i.wrapping_add_signed(-n), j)
    }
}

fn turn(dir: Direction, which_turn: Turn) -> Direction
{
    use Direction::*;
    use Turn::*;
    return match (dir, which_turn)
    {
        (Right, TurnRight) | (Left,  TurnLeft) => Down,
        (Down,  TurnRight) | (Up,    TurnLeft) => Left,
        (Left,  TurnRight) | (Right, TurnLeft) => Up,
        (Up,    TurnRight) | (Down,  TurnLeft) => Right,
    };
}

fn find_perimeter(board: &Vec<Vec<char>>,
                  start_i: usize,
                  start_j: usize,
                  start_dir: Direction) -> Vec<(usize,usize,Direction)>
{
    use Turn::*;

    let mut perimeter = Vec::<(usize,usize,Direction)>::new();

    let (mut i, mut j, mut dir) = (start_i, start_j, start_dir);
    loop
    {
        let left = turn(dir, TurnLeft);
        perimeter.push((i, j, left));
        let (i2, j2) = move_n(i, j, 1, dir);
        let (i3, j3) = move_n(i2, j2, 1, left);

        if board[i3][j3] != ' ' // Turn left
        {
            (i, j) = (i3, j3);
            dir = left;
        }
        else if board[i2][j2] == ' ' // Turn right, keep existing (i,j)
        {
            dir = turn(dir, TurnRight);
        }
        else
        {
            (i, j) = (i2, j2);
        }

        if (i, j, dir) == (start_i, start_j, start_dir)
        {
            break;
        }
    }
    return perimeter;
}

fn perimeter_map(mut perimeter: Vec<(usize,usize,Direction)>) ->
    HashMap<(usize,usize,Direction),(usize,usize,Direction)>
{
    let mut map = HashMap::<(usize,usize,Direction),(usize,usize,Direction)>::new();
    loop
    {
        let (mut prev_i, mut prev_j, mut prev_dir) = perimeter.last().unwrap();

        // n1 and n2 track the indexes that make up the 'zipper'. They must start together at a
        // concave vertex, which we must locate first. (A cube template must have at least two of these.)

        let len = perimeter.len();
        let mut n1 = perimeter.len() - 1;
        let mut n2 = 0;
        let mut found = false;
        for (n, (i, j, dir)) in perimeter.iter().enumerate()
        {
            if move_n(prev_i, prev_j, 1, prev_dir) == move_n(*i, *j, 1, *dir)
            {
                n2 = n;
                found = true;
                break;
            }
            (n1, prev_i, prev_j, prev_dir) = (n, *i, *j, *dir);
        }
        if !found { panic!() }

        let (mut i1, mut j1, mut dir1) = perimeter[n1];
        let (mut i2, mut j2, mut dir2) = perimeter[n2];
        loop
        {
            let (outer_i1, outer_j1) = move_n(i1, j1, 1, dir1);
            let (outer_i2, outer_j2) = move_n(i2, j2, 1, dir2);

            use Turn::*;
            map.insert((outer_i1, outer_j1, dir1), (i2, j2, turn(turn(dir2, TurnLeft), TurnLeft)));
            map.insert((outer_i2, outer_j2, dir2), (i1, j1, turn(turn(dir1, TurnLeft), TurnLeft)));

            let next_n1 = if n1 == 0         { len - 1 } else { n1 - 1 };
            let next_n2 = if n2 == (len - 1) { 0 }       else { n2 + 1 };
            if next_n1 == n2
            {
                // Both sides of the 'zipper' have met back up, which means we're done.
                return map;
            }


            if next_n1 == next_n2 { panic!() } // The indexes should never be equal, because that
                                               // implies an unmappable bit of perimeter.

            let (next_i1, next_j1, next_dir1) = perimeter[next_n1];
            let (next_i2, next_j2, next_dir2) = perimeter[next_n2];
            if (next_i1, next_j1, next_i2, next_j2) == (i1, j1, i2, j2)
            {
                // Both sides of the 'zipper' encounter a convex vertex at the same time, which we
                // detect because the perimeter points on either side of such a vertex have the
                // same (i,j) coordinates.
                //
                // This is as far as this 'zipper' can go.
                break;
            }
            (n1, i1, j1, dir1, n2, i2, j2, dir2) = (next_n1, next_i1, next_j1, next_dir1,
                                                    next_n2, next_i2, next_j2, next_dir2);
        }

        // Delete the now 'zipped-up' part of the perimeter.
        perimeter.drain(n1..=n2);
    }
}
