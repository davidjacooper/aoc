use std::io;

fn main() 
{
    let mut cur_value: i32 = 0;
    let mut index: i32 = 1;
    let mut max_index: i32 = 0;
    let mut max_value: i32 = -1;
    
    for line in io::stdin().lines()
    {
        let s = line.unwrap();
        if s.is_empty()
        {
            println!("Elf {} is carrying {} calories.", index, cur_value);
            if max_value < cur_value
            {
                max_index = index;
                max_value = cur_value;
            }
            index += 1;
            cur_value = 0;
        }
        else
        {
            cur_value += s.parse::<i32>().unwrap_or(0);
        }
    }
    println!("Elf {} is carrying {} calories.", index, cur_value);
    
    if max_value < cur_value
    {
        max_index = index;
        max_value = cur_value;
    }
    
    println!("---\nMaximum\n---\nElf {} is carrying {} calories.", max_index, max_value);
}
