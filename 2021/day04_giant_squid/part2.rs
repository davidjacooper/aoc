use std::time::Instant;

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("../inputs/day04_input.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            if let Err(err) = solve(&content)
            {
                println!("ERROR: {}", err);
            }
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

#[derive(Debug)]
struct Board
{
    has_won: bool,
    sum: usize,
    row_counts: [i8; 5],
    col_counts: [i8; 5],
    coords: [Option<(usize, usize)>; 100],
}

fn solve(content: &str) -> Result<(),&str>
{
    let (numbers_str, boards_str) =
        content.trim_end().split_once("\n\n").unwrap();

    let mut boards: Vec<Board> = boards_str
        .split("\n\n")
        .map(|board_str|
        {
            let mut board = Board {
                has_won: false,
                sum: 0,
                row_counts: [0; 5],
                col_counts: [0; 5],
                coords: [None; 100]
            };

            board_str.split("\n").enumerate().for_each(|(i, row_str)|
                row_str.split_whitespace().enumerate().for_each(|(j, val_str)|
                {
                    let val = val_str.parse::<usize>().unwrap();
                    board.sum += val;
                    board.coords[val] = Some((i, j));
                }));
            board
        })
        .collect();

    for number in numbers_str.split(",").map(|s| s.parse::<usize>().unwrap())
    {
        println!("{number}");

        for (board_index, board) in boards.iter_mut().enumerate()
        {
            if !board.has_won
            {
                if let Some((i, j)) = board.coords[number]
                {
                    board.row_counts[i] += 1;
                    board.col_counts[j] += 1;
                    board.sum -= number;

                    if board.row_counts[i] == 5 || board.col_counts[j] == 5
                    {
                        board.has_won = true;
                        println!("Board {} wins; unmarked sum == {}; score == {}",
                            board_index + 1,
                            board.sum,
                            number * board.sum);
                    }

                }
            }
        }
    }
    Ok(())
}
