use std::time::Instant;
use std::collections::HashMap;

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

    // Part 1 input rotated:

    // let input: Vec<(Direction,i64)> =
    //     content
    //     .trim_end()
    //     .lines()
    //     .map(|l| {
    //         let mut it = l.split_whitespace();
    //         return (
    //             match it.next().unwrap() { "D" => Right,
    //                                        "L" => Down,
    //                                        "U" => Left,
    //                                        "R" => Up,
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
    let mut verticals = Vec::<(i64, i64, i64)>::new();

    let mut cur_i = 0;
    let mut cur_j = 0;
    let mut prev_dir: Direction = input.last().unwrap().0;

    //println!("{:?}", input);

    for (dir, dist) in input
    {
        //println!("{:?}->{:?} {dist}", prev_dir, dir);

        match dir
        {
            Up =>
            {
                points.entry(cur_i).or_insert_with(|| Vec::new()).push(
                    Point {
                        j: cur_j,
                        line: match prev_dir { Right => UpLeft, Left => UpRight, _ => panic!() }
                    });

                points.entry(cur_i - dist + 1).or_insert_with(|| Vec::new()).push(
                    Point {
                        j: cur_j,
                        line: Vertical
                    });

                verticals.push((cur_i - dist + 2, cur_i - 1, cur_j));

                cur_i -= dist;
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
                points.entry(cur_i).or_insert_with(|| Vec::new()).push(
                    Point {
                        j: cur_j,
                        line: match prev_dir { Right => DownLeft, Left => DownRight, _ => panic!() }
                    });

                points.entry(cur_i + 1).or_insert_with(|| Vec::new()).push(
                    Point {
                        j: cur_j,
                        line: Vertical
                    });

                verticals.push((cur_i + 2, cur_i + dist - 1, cur_j));

                cur_i += dist;
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

    for (i, row) in points.iter_mut()
    {
        for (vert_i1, vert_i2, vert_j) in verticals.iter()
        {
            if vert_i1 <= i && i <= vert_i2
            {
                row.push(Point { j: *vert_j, line: Vertical });
            }
        }
    }

    let mut row_indexes = points.keys().map(|&k| k).collect::<Vec<i64>>();
    row_indexes.sort();

    let mut prev_i = row_indexes[0] - 1;
    let mut prev_row_area = 0;
    let mut area = 0;

    for i in row_indexes
    {
        let row = points.get_mut(&i).unwrap();
        row.sort();
        //println!("i={i}: {:?}", row);

        let d_area = (i - prev_i - 1) * prev_row_area;
        area += d_area;
        //println!("  +{d_area}");

        use InOut::*;
        let mut in_out = Outside;
        let mut start_j = 0;
        let mut row_area = 0;

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
                //println!("  +{d_area}");
                row_area += d_area;
            }
        }

        if in_out != Outside
        {
            panic!();
        }

        area += row_area;
        prev_row_area = row_area;
        prev_i = i;
    }

    println!("area = {area}");
}
