use std::time::Instant;

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input2.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            not_enough_minerals_part1(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

#[derive(Ord,PartialOrd,Eq,PartialEq,Copy,Clone,Debug)]
struct State
{
    geodes: i32,
    geode_bots: i32,
    obsidian: i32,
    obsidian_bots: i32,
    clay: i32,
    clay_bots: i32,
    ore: i32,
    ore_bots: i32,
}

fn not_enough_minerals_part1(content: &str)
{
    let mut sum = 0;

    for line in content.lines().filter(|l| !l.is_empty())
    {
        let n: Vec<i32> = line.split(&[' ', ':']).filter_map(|s| s.parse().ok()).collect();
        let [blueprint, ore_bot_ore, clay_bot_ore, obsidian_bot_ore, obsidian_bot_clay, geode_bot_ore, geode_bot_obsidian] = n[..] else { panic!(); };

        let mut states = vec![State { ore: 0, clay: 0, obsidian: 0, geodes: 0, ore_bots: 1, clay_bots: 0, obsidian_bots: 0, geode_bots: 0 }];
        for _time in 1..=24
        {
            let mut new_states = Vec::<State>::new();
            for state in states
            {
                let mut new_state = state.clone();
                new_state.ore += state.ore_bots;
                new_state.clay += state.clay_bots;
                new_state.obsidian += state.obsidian_bots;
                new_state.geodes += state.geode_bots;

                if state.obsidian >= geode_bot_obsidian && state.ore >= geode_bot_ore
                {
                    let mut new_state1 = new_state.clone();
                    new_state1.obsidian -= geode_bot_obsidian;
                    new_state1.ore -= geode_bot_ore;
                    new_state1.geode_bots += 1;
                    new_states.push(new_state1);
                }

                if state.clay >= obsidian_bot_clay && state.ore >= obsidian_bot_ore
                {
                    let mut new_state2 = new_state.clone();
                    new_state2.clay -= obsidian_bot_clay;
                    new_state2.ore -= obsidian_bot_ore;
                    new_state2.obsidian_bots += 1;
                    new_states.push(new_state2);
                }

                if state.ore >= clay_bot_ore
                {
                    let mut new_state3 = new_state.clone();
                    new_state3.ore -= clay_bot_ore;
                    new_state3.clay_bots += 1;
                    new_states.push(new_state3);
                }

                if state.ore >= ore_bot_ore
                {
                    let mut new_state4 = new_state.clone();
                    new_state4.ore -= ore_bot_ore;
                    new_state4.ore_bots += 1;
                    new_states.push(new_state4);
                }

                new_states.push(new_state);
            }
            new_states.sort_by(|a, b| b.cmp(a));
            new_states.truncate(63);
            states = new_states;
        }

        let quality = blueprint * states[0].geodes;
        sum += quality;
        println!("blueprint {}: geodes = {}, quality = {}", blueprint, states[0].geodes, quality);
    }

    println!("sum = {sum}");
}
