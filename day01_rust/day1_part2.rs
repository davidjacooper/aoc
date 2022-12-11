use std::io;

fn high_check(h3: &mut i32, h2: &mut i32, h1: &mut i32, cur: i32)
{
    if *h1 < cur
    {
        *h3 = *h2;
        *h2 = *h1;
        *h1 = cur;
    }
    else if *h2 < cur
    {
        *h3 = *h2;
        *h2 = cur;
    }
    else if *h3 < cur
    {
        *h3 = cur;
    }
}


fn main() 
{
    let mut cur_value: i32 = 0;
    let mut high1_value: i32 = 0;
    let mut high2_value: i32 = 0;
    let mut high3_value: i32 = 0;
    
    for line in io::stdin().lines()
    {
        let s = line.unwrap();
        if s.is_empty()
        {
            println!("{}", cur_value);
            high_check(&mut high3_value, &mut high2_value, &mut high1_value, cur_value);
            cur_value = 0;
        }
        else
        {
            cur_value += s.parse::<i32>().unwrap_or(0);
        }
    }
    
    println!("{}", cur_value);
    high_check(&mut high3_value, &mut high2_value, &mut high1_value, cur_value);
    
    println!("---\nThe top three elves are carrying {}, {} and {} calories, summing to {}.", 
        high1_value, high2_value, high3_value,
        high1_value + high2_value + high3_value);
}
