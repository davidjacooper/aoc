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
            cosmic_expansion_part1(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

fn cosmic_expansion_part1(content: &str)
{
    let mut galaxies = Vec::<(usize,usize)>::new();

    let mut occupied_rows = HashSet::<usize>::new();
    let mut occupied_cols = HashSet::<usize>::new();

    let mut max_row = 0;
    let mut max_col = 0;
    for (i, line) in content.lines().filter(|l| !l.is_empty()).enumerate()
    {
        for (j, ch) in line.chars().enumerate()
        {
            if ch == '#'
            {
                galaxies.push((i, j));
                max_row = i;
                if max_col < j
                {
                    max_col = j;
                }

                occupied_rows.insert(i);
                occupied_cols.insert(j);
            }
        }
    }

    let expanded_rows = create_expansion(&occupied_rows, max_row + 1);
    let expanded_cols = create_expansion(&occupied_cols, max_col + 1);

    let mut sum = 0;
    for g1 in 0..(galaxies.len() - 1)
    {
        let (i1, j1) = galaxies[g1];

        println!("galaxy ({},{}) -> ({},{})", i1, j1, expanded_rows[i1], expanded_cols[j1]);

        for g2 in g1..galaxies.len()
        {
            let (i2, j2) = galaxies[g2];
            let distance = (expanded_rows[i2] as i32 - expanded_rows[i1] as i32).abs() +
                           (expanded_cols[j2] as i32 - expanded_cols[j1] as i32).abs();
            sum += distance;
        }
    }

    println!("sum = {sum}");
}

fn create_expansion(occupied: &HashSet<usize>, len: usize) -> Vec<usize>
{
    let mut expanded_positions = Vec::<usize>::new();
    let mut expansion = 0;
    for i in 0..len
    {
        expanded_positions.push(i + expansion);
        if !occupied.contains(&i)
        {
            expansion += 1;
        }
    }
    return expanded_positions;
}
