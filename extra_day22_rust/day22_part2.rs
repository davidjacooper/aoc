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

#[derive(Copy,Clone,Eq,PartialEq,Hash,Debug)]
struct Position
{
    i: usize,
    j: usize,
    dir: Direction
}

impl Position
{
    fn move_n(&self, n: isize) -> Position
    {
        use Direction::*;
        return match self.dir
        {
            Right => Position { i: self.i, j: self.j.wrapping_add_signed(n), dir: self.dir },
            Down  => Position { i: self.i.wrapping_add_signed(n), j: self.j, dir: self.dir },
            Left  => Position { i: self.i, j: self.j.wrapping_add_signed(-n), dir: self.dir },
            Up    => Position { i: self.i.wrapping_add_signed(-n), j: self.j, dir: self.dir },
        }
    }

    fn turn_left(&self) -> Position
    {
        use Direction::*;
        return Position {
            i: self.i,
            j: self.j,
            dir: match self.dir { Left => Down, Up => Left, Right => Up, Down => Right }
        }
    }

    fn turn_right(&self) -> Position
    {
        use Direction::*;
        return Position {
            i: self.i,
            j: self.j,
            dir: match self.dir { Left => Up, Up => Right, Right => Down, Down => Left }
        }
    }

    fn turn_around(&self) -> Position
    {
        use Direction::*;
        return Position {
            i: self.i,
            j: self.j,
            dir: match self.dir { Left => Right, Up => Down, Right => Left, Down => Up }
        }
    }

    fn same_place_as(&self, other: &Position) -> bool
    {
        return self.i == other.i && self.j == other.j
    }
}

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

    let mut pos = Position {
        i: 1,
        j: board[1].iter().position(|c| *c == '.').unwrap(),
        dir: Direction::Right
    };

    let perimeter = find_perimeter(&board, pos);
    print_board(&board, &perimeter);
    let perimeter_map = create_perimeter_map(perimeter);

    use Direction::*;

    for step in step_str.trim().split_inclusive(&['R', 'L'])
    {
        let turn_ch = step.chars().last().unwrap();
        let mut dist = step.trim_end_matches(&['R', 'L']).parse::<usize>().unwrap();

        while dist > 0
        {
            let mut pos2 = pos.move_n(1);

            if board[pos2.i][pos2.j] == ' '
            {
                pos2 = *perimeter_map.get(&pos2).unwrap();
            }

            if board[pos2.i][pos2.j] == '#'
            {
                break;
            }

            pos = pos2;
            dist -= 1;
        }

        pos = match turn_ch
        {
            'R' => pos.turn_right(),
            'L' => pos.turn_left(),
            _ => pos
        };
    }

    // 1-based row/col numbering (in the prob statement) cancels out padding rows and cols.
    println!("{:?}", pos);
    println!("Password = {}", 1000 * pos.i + 4 * pos.j + match pos.dir { Right => 0, Down => 1, Left => 2, Up => 3 });
}

fn print_board(board: &Vec<Vec<char>>, perimeter: &Vec<Position>)
{
    let perim_set = perimeter
        .iter()
        .map(|pos| {
            let Position { i, j, dir: _ } = pos.move_n(1);
            return (i, j)
        })
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

fn find_perimeter(board: &Vec<Vec<char>>, start_pos: Position) -> Vec<Position>
{
    let mut perimeter = Vec::<Position>::new();
    let mut pos = start_pos;
    loop
    {
        perimeter.push(pos.turn_left());
        let pos_ahead = pos.move_n(1);
        let pos_left = pos_ahead.turn_left().move_n(1);

        pos = if board[pos_left.i][pos_left.j] != ' '        { pos_left }
              else if board[pos_ahead.i][pos_ahead.j] == ' ' { pos.turn_right() }
              else                                           { pos_ahead };

        if pos == start_pos { break }
    }
    return perimeter;
}

fn create_perimeter_map(mut perimeter: Vec<Position>) -> HashMap<Position,Position>
{
    let mut map = HashMap::<Position,Position>::new();
    loop
    {
        let mut prev_pos = perimeter.last().unwrap();

        // n1 and n2 track the indexes that make up the 'zipper'. They must start together at a
        // concave vertex, which we must locate first. (A cube template must have at least two of these.)

        let len = perimeter.len();
        let mut n1 = perimeter.len() - 1;
        let mut n2 = 0;
        let mut found = false;
        for (n, pos) in perimeter.iter().enumerate()
        {
            if prev_pos.move_n(1).same_place_as(&pos.move_n(1))
            {
                n2 = n;
                found = true;
                break;
            }
            (n1, prev_pos) = (n, pos);
        }
        if !found { panic!() }

        loop
        {
            let pos1 = perimeter[n1];
            let pos2 = perimeter[n2];
            map.insert(pos1.move_n(1), pos2.turn_around());
            map.insert(pos2.move_n(1), pos1.turn_around());

            let next_n1 = n1.checked_sub(1).unwrap_or(len - 1);
            let next_n2 = (n2 + 1) % len;
            if next_n1 == n2
            {
                return map; // Both sides of the 'zipper' have met back up, which means we're done.
            }
            if pos1.same_place_as(&perimeter[next_n1]) && pos2.same_place_as(&perimeter[next_n2])
            {
                break; // This is as far as this 'zipper' can go.
            }
            (n1, n2) = (next_n1, next_n2);
        }

        // Delete the now 'zipped-up' part of the perimeter.
        perimeter.drain(n1..=n2);
    }
}
