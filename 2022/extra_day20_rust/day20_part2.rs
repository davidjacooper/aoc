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
            grove_positioning_system_part1(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}


fn grove_positioning_system_part1(content: &str)
{
    let mut numbers: Vec<i64> =
        content
        .lines()
        .flat_map(|l| l.parse::<i64>().ok())
        .map(|n| n * 811589153)
        .collect();
    let len = numbers.len();
    let mut indexes: Vec<usize> = (0..len).collect();

    for _ in 0..10
    {
        for meta_index in 0..len
        {
            let old_index = indexes[meta_index];
            let new_index = (old_index as i64 + numbers[old_index]).rem_euclid(len as i64 - 1) as usize;

            let value = numbers.remove(old_index);
            numbers.insert(new_index, value);

            if new_index > old_index
            {
                for index in indexes.iter_mut()
                {
                    if *index > old_index && *index <= new_index
                    {
                        *index -= 1;
                    }
                }
            }
            else if new_index < old_index
            {
                for index in indexes.iter_mut()
                {
                    if *index < old_index && *index >= new_index
                    {
                        *index += 1;
                    }
                }
            }

            indexes[meta_index] = new_index;
        }
    }

    let zero_i = numbers.iter().position(|n| *n == 0).unwrap();
    let n1 = numbers[(zero_i + 1000) % len];
    let n2 = numbers[(zero_i + 2000) % len];
    let n3 = numbers[(zero_i + 3000) % len];

    println!("n1 = {n1}, n2 = {n2}, n3 = {n3}, sum = {}", n1 + n2 + n3);
}
