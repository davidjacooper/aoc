use std::fs;
use std::env;

fn main()
{
    let file = env::args().nth(1).unwrap_or("input2.txt".to_string());

    if let Ok(content) = fs::read_to_string(file.clone())
    {
        let mut sum = 0;
        for line in content.lines()
        {
            if let Some((header, game_str)) = line.split_once(": ")
            {
                let game_id: i32 = header.trim_start_matches("Game ").parse().unwrap();
                let mut max_red = 0;
                let mut max_green = 0;
                let mut max_blue = 0;

                for draw_str in game_str.split("; ")
                {
                    for cube_str in draw_str.split(", ")
                    {
                        let (n_str, colour) = cube_str.split_once(' ').unwrap();
                        let n: i32 = n_str.parse().unwrap();
                        match colour
                        {
                            "red"   => if max_red < n   { max_red = n; }
                            "green" => if max_green < n { max_green = n; }
                            "blue"  => if max_blue < n  { max_blue = n; }
                            _       => panic!()
                        }
                    }
                }

                let power = max_red * max_green * max_blue;
                println!("Game {} - {} red, {} green, {} blue - {} power", game_id, max_red, max_green, max_blue, power);

                sum += power;
            }
        }
        println!("Sum == {}", sum);
    }
    else
    {
        println!("Cannot read '{}'", file);
    }
}
