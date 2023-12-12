// Part 2: dynamic programming solution

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
            hot_springs_part2(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

fn hot_springs_part2(content: &str)
{
    let mut sum = 0;
    for line in content.lines().filter(|l| !l.is_empty())
    {
        let (spring_str, broken_group_str) = line.split_once(' ').unwrap();

        let mut springs: Vec<char> = vec![spring_str; 5].join("?").chars().collect();
        let broken_groups: Vec<usize> = broken_group_str
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>()
            .repeat(5);

        springs.push('.');

        let n_springs = springs.len();
        let n_groups = broken_groups.len();

        let mut sub_solutions: Vec<Vec<i64>> = vec![vec![0; n_groups + 1]; n_springs + 1];

        sub_solutions[n_springs][n_groups] = 1;
        for g in 0..n_groups
        {
            sub_solutions[n_springs][g] = 0;
        }

        let mut next_soln = 1;
        for s in (0..n_springs).rev()
        {
            if springs[s] == '#'
            {
                next_soln = 0;
            }
            sub_solutions[s][n_groups] = next_soln;
        }

        for s in (0..n_springs).rev()
        {
            for g in (0..n_groups).rev()
            {
                let mut result_if_broken = 0;
                let mut result_if_working = 0;
                let expected_broken_n = broken_groups[g];

                if (springs[s] == '#' || springs[s] == '?') && ((n_springs - s) >= (expected_broken_n + 1))
                {
                    result_if_broken = sub_solutions[s + expected_broken_n + 1][g + 1];
                    for i in 1..expected_broken_n
                    {
                        if springs[s + i] == '.'
                        {
                            result_if_broken = 0;
                            break;
                        }
                    }
                    if springs[s + expected_broken_n] == '#'
                    {
                        result_if_broken = 0;
                    }
                }

                if springs[s] == '.' || springs[s] == '?'
                {
                    result_if_working = sub_solutions[s + 1][g];
                }

                sub_solutions[s][g] = result_if_broken + result_if_working;
            }
        }

        //print_table(&sub_solutions);

        let arrangements = sub_solutions[0][0];
        sum += arrangements;
        println!("  springs={:?}, groups={:?} => arrangements={arrangements}",
                 springs.iter().collect::<String>(), broken_groups);
    }
    println!("sum = {sum}");
}

fn print_table(sub_solutions: &Vec<Vec<usize>>)
{
    println!("sub solutions:");
    for row in sub_solutions.iter()
    {
        println!("{:?}", row);
    }
}
