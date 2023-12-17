use std::time::Instant;
use std::collections::{HashSet};

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input1.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            clumsy_crucible_part1(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

// #[derive(Debug)]
// struct State
// {
//     i: usize,
//     j: usize,
//     delta_i: u8,
//     delta_j: u8,
//     straight: i8,
//     heat_loss: u32,
// }

struct Block
{
    i: usize,
    j: usize,
    heat_loss: u8,
    min_heat_loss: u32,
    enqueued: bool,
    heap_index: usize,
}

struct Queue
{
    heap: Vec<Option<&Block>>
}

impl Queue
{
    fn pop(&mut self) -> Option<Heap>
    {
        let v = self.heap.get(0);
        if self.heap.len() > 1
        {
            self.heap[0] = self.heap.pop();
            heapify(0);
        }
        return v;
    }

    fn heapify(&mut self, index: usize)
    {
        let min_index = index;
        let min = self.heap[index];
        let l_index = 2 * index + 1;
        let r_index = l_index + 1;
        let len = self.heap.len();

        if l_index < len && self.heap[l_index].min_heat_loss < min
        {
            min = self.heap[l_index].min_heat_loss;
            min_index = l_index;
        }

        if r_index < len && self.heap[r_index].min_heat_loss < min
        {
            min = self.heap[r_index].min_heat_loss;
            min_index = r_index;
        }

        // if let Some(l_block) = self.heap.get(l_index)
        // {
        //     if smallest > l_block.min_heat_loss
        //     {
        //         smallest = l_block.min_heat_loss;
        //         smallest_index = l_index;
        //     }
        //
        //     if let Some(r_block) = self.heap.get(r_index)
        //     {
        //         if smallest > r_block.min_heat_loss
        //         {
        //             smallest = r_block.min_heat_loss;
        //             smallest_index = r_index;
        //         }
        //     }
        //     self.heap[index] = self.heap[next_index];
        //     self.heapify(next_index);
        // }

        if min_index != index
        {
            self.heap.swap(index, min_index);
            self.heapify(min_index);
        }
    }

    fn reduce_n(&mut self, block: &Block, min_heat_loss: u32)
    {
        block.min_heat_loss = min_heat_loss;
        let mut index = block.heap_index;
        while index > 0
        {
            let mut parent_index = (index - 1) / 2;
            if self.heap[parent_index] <= min_heat_loss { break }

            self.heap.swap(index, parent_index);
            index = parent_index;
        }
    }

//     fn add(&mut self, block: &Block)
//     {
//
//     }
}

fn clumsy_crucible_part1(content: &str)
{
    // let map: Vec<Vec<u8>> = content
    //     .lines()
    //     .map(|l| l.bytes().map(|b| b - b'0').collect())
    //     .collect();
    let map: Vec<Vec<Block>> = content
        .lines()
        .enumerate()
        .map(|(i, line|
            line
            .bytes()
            .enumerate()
            .map(|(j, byte)|
                Block {
                    i: i,
                    j: j,
                    heat_loss: byte - b'0',
                    min_heat_loss: u32::MAX,
                    finalised: false,
                })
            .collect())
        .collect();

    let queue = Queue { heap: vec![&map[0][0]] };

    while let Some(block) = queue.pop()
    {

    }


    // Djikstra's algorithm?



    //let sub_solutions =

    // for row in map.iter()
    // {
    //     for cell in row.iter()
    //     {
    //         print!("{} ", cell);
    //     }
    //     println!();
    // }



}

