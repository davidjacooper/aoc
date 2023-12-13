use std::time::Instant;

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input1.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            monkey_map_part1(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

#[derive(Debug)]
enum Direction { Right, Down, Left, Up }

fn monkey_map_part1(content: &str)
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

    print_board(&board);

    let mut i = 1;
    let mut j = board[1].iter().position(|c| *c == '.').unwrap();
    let mut dir = Direction::Right;
    println!("start=({i},{j})");

    use Direction::*;

    for step in step_str.trim().split_inclusive(&['R', 'L'])
    {
        let turn = step.chars().last().unwrap();
        let mut dist = step.trim_end_matches(&['R', 'L']).parse::<usize>().unwrap();

        print!("{dist}-{turn}, from ({i},{j}:{:?}) to... ", dir);
        while dist > 0
        {
            let (mut i2, mut j2) = (i, j);
            match dir
            {
                Right => j2 += 1,
                Down  => i2 += 1,
                Left  => j2 -= 1,
                Up    => i2 -= 1
            }

            if board[i2][j2] == ' '
            {
                match dir
                {
                    Right => while board[i2][j2 - 1] != ' ' { j2 -= 1; },
                    Down  => while board[i2 - 1][j2] != ' ' { i2 -= 1; },
                    Left  => while board[i2][j2 + 1] != ' ' { j2 += 1; },
                    Up    => while board[i2 + 1][j2] != ' ' { i2 += 1; },
                };
            }

            if board[i2][j2] == '#'
            {
                break;
            }

            (i, j) = (i2, j2);
            dist -= 1;
        }

        dir = match (&dir, turn)
        {
            (Right, 'R') | (Left,  'L') => Down,
            (Down,  'R') | (Up,    'L') => Left,
            (Left,  'R') | (Right, 'L') => Up,
            (Up,    'R') | (Down,  'L') => Right,
            _ => dir
        };
        println!("({i},{j}:{:?})", dir);
    }

    // 1-based row/col numbering (in the prob statement) cancels out padding rows and cols.
    println!("Final position: ({i},{j},{:?})", dir);
    println!("Password = {}", 1000 * i + 4 * j + match dir { Right => 0, Down => 1, Left => 2, Up => 3 });

}

fn print_board(board: &Vec<Vec<char>>)
{
    for row in board.iter()
    {
        println!("[{}]", row.iter().collect::<String>());
    }
}
