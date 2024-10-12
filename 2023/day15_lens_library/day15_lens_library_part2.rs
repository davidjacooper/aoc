// The HASHMAP is a lie. There is no HASHMAP.

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
            lens_library_part2(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

struct Lens<'a>
{
    label: &'a str,
    focal_length: u8,
}

fn lens_library_part2(content: &str)
{
    let mut boxes: Vec<Vec<Lens>> =
        (0..256).map(|_| Vec::<Lens>::new()).collect();

    for step in content.trim().split(",")
    {
        if step.ends_with('-')
        {
            let label = &step[..(step.len() - 1)];
            boxes[hash(label)].retain(|l| l.label != label);
        }
        else
        {
            let (label, focal_str) = step.split_once('=').unwrap();
            let focal_length: u8 = focal_str.parse().unwrap();
            let lens_box = &mut boxes[hash(label)];
            if let Some(lens) = lens_box.iter_mut()
                                        .find(|l| l.label == label)
            {
                lens.focal_length = focal_length;
            }
            else
            {
                lens_box.push(Lens { label: label,
                                     focal_length: focal_length });
            }
        }

        //print_boxes(step, &boxes);
    }

    let mut sum = 0;
    for (box_n, lens_box) in boxes.iter().enumerate()
    {
        for (lens_n, lens) in lens_box.iter().enumerate()
        {
            let power = (box_n + 1) * (lens_n + 1) * lens.focal_length as usize;
            sum += power;
            // println!("{}: {} * {} * {} = {}",
            //          lens.label,
            //          box_n + 1,
            //          lens_n + 1,
            //          lens.focal_length,
            //          power);
        }
    }
    println!("sum = {sum}");
}

fn hash(s: &str) -> usize
{
    let mut hash: usize = 0;
    for b in s.bytes().filter(|b| *b != b'\n')
    {
        hash = (hash + b as usize) * 17 % 256;
    }
    return hash;
}

fn print_boxes(step: &str, boxes: &Vec<Vec<Lens>>)
{
    println!("After \"{step}\":");
    for (box_n, lens_box) in boxes.iter().enumerate()
    {
        if !lens_box.is_empty()
        {
            println!("Box {}: {}",
                        box_n,
                        lens_box.iter()
                                .map(|l| format!("[{} {}]",
                                                l.label, l.focal_length))
                                .collect::<Vec<String>>()
                                .join(" "));
        }
    }
}
