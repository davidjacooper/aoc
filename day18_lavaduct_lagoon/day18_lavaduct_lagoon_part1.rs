use std::time::Instant;
use std::collections::HashSet;
use std::cmp::{min,max};

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input1.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            lavaduct_lagoon_part1(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

#[derive(Copy,Clone,Hash,Eq,PartialEq,Debug)]
struct Point(i64,i64);

fn lavaduct_lagoon_part1(content: &str)
{
    let mut min_i = 0;
    let mut min_j = 0;
    let mut max_i = 0;
    let mut max_j = 0;
    let mut points = HashSet::<Point>::new();

    let mut pos = Point(0, 0);
    points.insert(pos);
    for lines in content.trim_end().lines()
    {
        let mut it = lines.split_whitespace();
        let dir = it.next().unwrap();
        let dist = it.next().unwrap().parse::<i64>().unwrap();

        match dir
        {
            "U" => {
                for i in (pos.0 - dist)..=(pos.0) { points.insert(Point(i, pos.1)); }
                pos = Point(pos.0 - dist, pos.1);
                min_i = min(min_i, pos.0);
            }
            "R" => {
                for j in (pos.1)..=(pos.1 + dist) { points.insert(Point(pos.0, j)); }
                pos = Point(pos.0, pos.1 + dist);
                max_j = max(max_j, pos.1);
            }
            "D" => {
                for i in (pos.0)..=(pos.0 + dist) { points.insert(Point(i, pos.1)); }
                pos = Point(pos.0 + dist, pos.1);
                max_i = max(max_i, pos.0);
            }
            "L" => {
                for j in (pos.1 - dist)..=(pos.1) { points.insert(Point(pos.0, j)); }
                pos = Point(pos.0, pos.1 - dist);
                min_j = min(min_j, pos.1);
            }
            _ => { panic!() }
        }
    }
    print_points(&points, min_i, min_j, max_i, max_j);


    let mut outside_points = HashSet::<Point>::new();
    let mut current = HashSet::<Point>::new();
    let mut next = HashSet::<Point>::new();
    for i in (min_i - 1)..=(max_i + 1)
    {
        current.insert(Point(i, min_j - 1));
        current.insert(Point(i, max_j + 1));
    }
    for j in (min_j - 1)..=(max_j + 1)
    {
        current.insert(Point(min_i - 1, j));
        current.insert(Point(max_i + 1, j));
    }

    while current.len() > 0
    {
        for pt in current.iter()
        {
            outside_points.insert(*pt);
        }

        for pt in current.drain()
        {

            let up_pt = Point(pt.0 - 1, pt.1);
            if up_pt.0 >= min_i && !outside_points.contains(&up_pt) && !points.contains(&up_pt)
            {
                next.insert(up_pt);
            }

            let right_pt = Point(pt.0, pt.1 + 1);
            if right_pt.1 <= max_j && !outside_points.contains(&right_pt) && !points.contains(&right_pt)
            {
                next.insert(right_pt);
            }

            let down_pt  = Point(pt.0 + 1, pt.1);
            if down_pt.0 <= max_i && !outside_points.contains(&down_pt) && !points.contains(&down_pt)
            {
                next.insert(down_pt);
            }

            let left_pt  = Point(pt.0, pt.1 - 1);
            if left_pt.1 >= min_j && !outside_points.contains(&left_pt) && !points.contains(&left_pt)
            {
                next.insert(left_pt);
            }
        }

        for next_pt in next.drain()
        {
            current.insert(next_pt);
        }
    }

    println!("\noutside points:");
    print_points(&outside_points, min_i, min_j, max_i, max_j);

    let total_area = (max_i - min_i + 3) * (max_j - min_j + 3);
    println!("\narea = {} - {} = {}",
             total_area,
             outside_points.len(),
             total_area - outside_points.len() as i64);
}

fn print_points(points: &HashSet<Point>, i1: i64, j1: i64, i2: i64, j2: i64)
{
    for i in i1..=i2
    {
        for j in j1..=j2
        {
            print!("{}", if points.contains(&Point(i, j)) { '#' } else { '.' })
        }
        println!();
    }
}
