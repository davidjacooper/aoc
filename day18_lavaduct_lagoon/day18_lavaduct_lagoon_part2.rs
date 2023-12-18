use std::time::Instant;
use std::collections::HashMap;
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
            lavaduct_lagoon_part2(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
enum Direction { Up, Right, Down, Left }
use Direction::*;

#[derive(Ord,PartialOrd,Eq,PartialEq,Debug)]
enum LineSegment { UpRight, UpLeft, DownRight, DownLeft, Vertical }
use LineSegment::*;

#[derive(Ord,PartialOrd,Eq,PartialEq,Debug)]
struct Point
{
    j: i64,
    line: LineSegment
}

#[derive(Eq,PartialEq,Debug)]
enum InOut { Outside, Inside, TopLine, BottomLine }


fn lavaduct_lagoon_part2(content: &str)
{
    // Part 1 input:

    // let input: Vec<(Direction,i64)> =
    //     content
    //     .trim_end()
    //     .lines()
    //     .map(|l| {
    //         let mut it = l.split_whitespace();
    //         return (
    //             match it.next().unwrap() { "R" => Right,
    //                                        "D" => Down,
    //                                        "L" => Left,
    //                                        "U" => Up,
    //                                         _  => panic!() },
    //             it.next().unwrap().parse::<i64>().unwrap()
    //         )
    //     })
    //     .collect();

    // Part 2 input:

    let input: Vec<(Direction,i64)> =
        content
        .trim_end()
        .lines()
        .map(|l| {
            let (_, code) = l.split_once('#').unwrap();
            return (
                match code.as_bytes()[5] { b'0' => Right,
                                           b'1' => Down,
                                           b'2' => Left,
                                           b'3' => Up,
                                           _    => panic!() },
                i64::from_str_radix(&code[0..5], 16).unwrap()
            )
        })
        .collect();


    let mut points = HashMap::<i64,Vec<Point>>::new();
    let mut cur_i = 0;
    let mut cur_j = 0;
    let mut min_i = 0;
    let mut max_i = 0;
    let mut prev_dir: Direction = input.last().unwrap().0;

    println!("{:?}", input);

    for (dir, dist) in input
    {
        println!("{:?}->{:?} {dist}", prev_dir, dir);

        match dir
        {
            Up =>
            {
                for i in (cur_i - dist)..=(cur_i)
                {
                    points.entry(i).or_insert_with(|| Vec::new());
                }

                points.get_mut(&cur_i).unwrap().push(
                    Point {
                        j: cur_j,
                        line: match prev_dir { Right => UpLeft, Left => UpRight, _ => panic!() }
                    });

                for i in (cur_i - dist + 1)..(cur_i)
                {
                    points.get_mut(&i).unwrap().push(
                        Point {
                            j: cur_j,
                            line: Vertical
                        });
                }
                cur_i -= dist;
                min_i = min(min_i, cur_i);
            }
            Right =>
            {
                points.entry(cur_i).or_insert_with(|| Vec::new()).push(
                    Point {
                        j: cur_j,
                        line: match prev_dir { Up => DownRight, Down => UpRight, _ => panic!() }
                    });
                cur_j += dist;
            }
            Down =>
            {
                for i in cur_i..=(cur_i + dist)
                {
                    points.entry(i).or_insert_with(|| Vec::new());
                }

                points.get_mut(&cur_i).unwrap().push(
                    Point {
                        j: cur_j,
                        line: match prev_dir { Right => DownLeft, Left => DownRight, _ => panic!() }
                    });

                for i in (cur_i + 1)..(cur_i + dist)
                {
                    points.get_mut(&i).unwrap().push(
                        Point {
                            j: cur_j,
                            line: Vertical
                        });
                }
                cur_i += dist;
                max_i = max(max_i, cur_i);
            }
            Left =>
            {
                points.entry(cur_i).or_insert_with(|| Vec::new()).push(
                    Point {
                        j: cur_j,
                        line: match prev_dir { Up => DownLeft, Down => UpLeft, _ => panic!() }
                    });
                cur_j -= dist;
            }
        }
        prev_dir = dir;
    }

    let mut area = 0;

    for i in min_i..=max_i
    {
        let row = points.get_mut(&i).unwrap();
        row.sort();
        if i % 10000 == 0
        {
            println!("i={i}: {:?}", row);
        }

        use InOut::*;
        let mut in_out = Outside;
        let mut start_j = 0;

        for Point{j, line} in row.iter()
        {
            if in_out == Outside
            {
                start_j = *j;
            }

            in_out = match (line, in_out)
            {
                (DownRight, Outside) | (UpRight, Inside)  => TopLine,
                (DownRight, Inside)  | (UpRight, Outside) => BottomLine,
                (Vertical, Outside)  | (DownLeft, BottomLine) | (UpLeft, TopLine)    => Inside,
                (Vertical, Inside)   | (DownLeft, TopLine)    | (UpLeft, BottomLine) => Outside,
                _ => panic!()
            };

            if in_out == Outside
            {
                let d_area = j - start_j + 1;
                area += d_area;
            }
        }
    }

    println!("area = {area}");
}
