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
            wait_for_it(&content);
            println!("Time: {:?}", Instant::now().duration_since(start));
        }
    }
}

fn wait_for_it(content: &str)
{
    let mut it = content.lines();
    let mut product = 1.0;

    for (race_time, record_dist) in n_iter(it.next().unwrap()).zip(n_iter(it.next().unwrap()))
    {
        println!("race_time == {race_time}, record_dist == {record_dist}");

        // rel_dist = button_time * (race_time - button_time) - record_dist
        //          = -button_time**2 + race_time * button_time - record_dist

        // Apply quadratic formula with:
        // a == -1
        // b == race_time
        // c == -record_dist
        // => roots = (-race_time +- sqrt(race_time**2 - 4*record_dist)) / -2

        let c = (((race_time * race_time) - 4.0 * record_dist)).sqrt();
        let root1 = (-race_time + c) / -2.0;
        let root2 = (-race_time - c) / -2.0;

        let iroot1 = if root1.fract() == 0.0 { root1 + 1.0 } else { root1.ceil() };
        let iroot2 = if root2.fract() == 0.0 { root2 - 1.0 } else { root2.floor() };

        let ways_to_win = iroot2 - iroot1 + 1.0;
        product *= ways_to_win;

        println!("roots = {iroot1}, {iroot2}; ways to win = {ways_to_win}");
    }

    println!("product = {product}");
}


fn n_iter(line: &str) -> impl Iterator<Item = f64> + '_
{
    return line.split_whitespace().skip(1).map(|s| s.parse().unwrap());
}
