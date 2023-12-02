use std::fs;
use std::env;

fn main()
{
    // Optional filename as a command-line argument
    let file = env::args().nth(1).unwrap_or("input1.txt".to_string());

    if let Ok(content) = fs::read_to_string(file.clone())
    {
        let mut sum = 0;
        for line in content.lines()
        {
            if let Some((header, game_str)) = line.split_once(": ")
            {
                let game_id: i32 = header.trim_start_matches("Game ").parse().unwrap();
                let mut possible = true;
                'game: for draw_str in game_str.split("; ")
                {
                    for cube_str in draw_str.split(", ")
                    {
                        let (n_str, colour) = cube_str.split_once(' ').unwrap();
                        let n: i32 = n_str.parse().unwrap();

                        if (colour == "red" && n > 12) || (colour == "green" && n > 13) || (colour == "blue" && n > 14)
                        {
                            possible = false;
                            break 'game;
                        }
                    }
                }

                println!("Game {} - possible? {}", game_id, possible);

                if possible
                {
                    sum += game_id;
                }
            }
        }
        println!("Sum == {}", sum);
    }
    else
    {
        println!("Cannot read '{}'", file);
    }
}
