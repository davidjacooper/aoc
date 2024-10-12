use std::time::Instant;
use std::cmp::min;

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input1.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            point_of_incidence_part1(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

fn point_of_incidence_part1(content: &str)
{
    let mut sum = 0;
    for mut puzzle_str in content.split("\n\n")
    {
        puzzle_str = puzzle_str.trim();

        let mut rows = Vec::<u64>::new();
        let mut cols = vec![0u64; puzzle_str.find('\n').unwrap()];

        for (i, line) in puzzle_str.lines().enumerate()
        {
            let mut row_value = 0u64;
            for (j, ch) in line.chars().enumerate()
            {
                if ch == '#'
                {
                    row_value |= 1 << j;
                    cols[j] |= 1 << i;
                }
            }
            rows.push(row_value);
        }

        let row_reflection = find_reflection(&rows);
        let col_reflection = find_reflection(&cols);

        println!("{:?}, {:?}, row-reflect: {row_reflection}, col-reflect: {col_reflection}", rows, cols);

        sum += 100 * row_reflection + col_reflection;
    }

    println!("sum = {sum}");
}

fn find_reflection(numbers: &[u64]) -> usize
{
    for i in 0..(numbers.len() - 1)
    {
        let mut smudged = smudge(numbers[i], numbers[i + 1]);
        if smudged || numbers[i] == numbers[i + 1]
        {
            let mut complete = true;
            for j in 1..=min(i, numbers.len() - i - 2)
            {
                if numbers[i - j] != numbers[i + 1 + j]
                {
                    if !smudged && smudge(numbers[i - j], numbers[i + 1 + j])
                    {
                        smudged = true;
                    }
                    else
                    {
                        complete = false;
                        break;
                    }
                }
            }
            if complete && smudged
            {
                return i + 1;
            }
        }
    }
    return 0;
}

fn smudge(n1: u64, n2: u64) -> bool
{
    return (n1 ^ n2).count_ones() == 1;
}
