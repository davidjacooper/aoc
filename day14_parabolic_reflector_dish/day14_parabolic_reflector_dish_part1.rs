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
            parabolic_reflector_dish_part1(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

fn parabolic_reflector_dish_part1(content: &str)
{
    let mut platform = content
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    print_platform(&platform);
    slide_rocks(&mut platform);
    print_platform(&platform);

    let mut sum = 0;
    let mut weight = platform.len();
    for row in platform.iter()
    {
        let load = row.iter().filter(|c| **c == 'O').count() * weight;
        sum += load;
        weight -= 1;
        println!("{load}");
    }
    println!("sum = {sum}");
}


fn print_platform(platform: &Vec<Vec<char>>)
{
    println!("---");
    for row in platform.iter()
    {
        println!("{}", row.iter().collect::<String>());
    }
}

fn slide_rocks(platform: &mut Vec<Vec<char>>)
{
    let width = platform[0].len();
    let mut blocking = vec![0; width];
    for i in 0..platform.len()
    {
        for j in 0..width
        {
            match platform[i][j]
            {
                '#' => blocking[j] = i + 1,
                'O' => {
                    platform[i][j] = '.';
                    platform[blocking[j]][j] = 'O';
                    blocking[j] += 1;
                }
                _ => {}
            }
        }
    }
}
