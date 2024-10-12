use std::time::Instant;
use std::collections::{HashSet};
use std::io::Write;

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            if let Err(err) = never_tell_me_the_odds_part1(&content)
            {
                println!("ERROR: {}", err);
            }
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

/*
 * Input observations:
 * - there are no velocities with a 0 component.
 */

#[derive(Debug)]
struct Coord
{
    x: i64, y: i64, z: i64
}

#[derive(Debug)]
struct Hailstone
{
    pos: Coord, vel: Coord
}

// For test input:
// const TEST_AREA_MIN_X: f64 = 7.0;
// const TEST_AREA_MIN_Y: f64 = 7.0;
// const TEST_AREA_MAX_X: f64 = 27.0;
// const TEST_AREA_MAX_Y: f64 = 27.0;

// For real input:
const TEST_AREA_MIN_X: f64 = 200000000000000.0;
const TEST_AREA_MIN_Y: f64 = 200000000000000.0;
const TEST_AREA_MAX_X: f64 = 400000000000000.0;
const TEST_AREA_MAX_Y: f64 = 400000000000000.0;


fn never_tell_me_the_odds_part1(content: &str) -> Result<(),&str>
{
    let hailstones: Vec<Hailstone> =
        content
        .trim_end()
        .lines()
        .map(|line|
        {
            let coords: Vec<i64> =
                line
                .split(" @ ")
                .flat_map(|s| s.split(", "))
                .map(|s| s.trim().parse().unwrap())
                .collect();

            Hailstone {
                pos: Coord {x: coords[0], y: coords[1], z: coords[2]},
                vel: Coord {x: coords[3], y: coords[4], z: coords[5]}
            }
        })
        .collect();

    println!("hailstones = {:?}", hailstones);

    let mut n_pairs = 0;
    for i in 0..(hailstones.len() - 1)
    {
        let h1 = &hailstones[i];
        let slope1 = h1.vel.y as f64 / h1.vel.x as f64;
        let yaxis_intercept1 = h1.pos.y as f64 - slope1 * h1.pos.x as f64;

        for j in (i + 1)..hailstones.len()
        {
            let h2 = &hailstones[j];
            let slope2 = h2.vel.y as f64 / h2.vel.x as f64;
            let yaxis_intercept2 = h2.pos.y as f64 - slope2 * h2.pos.x as f64;

            let x = (yaxis_intercept1 - yaxis_intercept2) / (slope2 - slope1);
            let y = yaxis_intercept1 + slope1 * x;
            let t1 = (x - h1.pos.x as f64) / h1.vel.x as f64;
            let t2 = (x - h2.pos.x as f64) / h2.vel.x as f64;

            print!("hailstones {i} and {j} intersect at ({x},{y})... ");
            if t1 <= 0.0 || t2 <= 0.0
            {
                println!("in the past: t1 = {t1}, t2 = {t2}");
            }
            else if TEST_AREA_MIN_X <= x && x <= TEST_AREA_MAX_X &&
                    TEST_AREA_MIN_Y <= y && y <= TEST_AREA_MAX_Y
            {
                println!("INSIDE TEST AREA");
                n_pairs += 1;
            }
            else
            {
                println!("outside test area");
            }
        }
    }

    println!("#pairs inside test area = {n_pairs}");

    Ok(())
}
