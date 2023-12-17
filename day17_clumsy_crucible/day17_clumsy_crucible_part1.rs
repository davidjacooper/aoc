use std::time::Instant;
use std::collections::{HashMap,HashSet};

const MAX_STRAIGHT_RUN: u8 = 3;

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

#[derive(Copy,Clone,Hash,Eq,PartialEq,Debug)]
enum Direction { Up, Right, Down, Left }
use Direction::*;

impl Direction
{
    // fn delta(&self) -> (isize, isize)
    // {
    //     return match self {
    //         Up => (-1, 0),
    //         Right => (0, 1),
    //         Down => (1, 0),
    //         Left => (0, -1)
    //     }
    // }
    fn move_pos(&self, i: usize, j: usize, i_bound: usize, j_bound: usize) -> Option<(usize, usize)>
    {
        return match (
            i.checked_add_signed(match self { Up   => -1, Down  => 1, _ => 0 }),
            j.checked_add_signed(match self { Left => -1, Right => 1, _ => 0 })
        )
        {
            (Some(i2), Some(j2)) if i2 < i_bound && j2 < j_bound => Some((i2, j2)),
            _ => None
        }
    }

    fn opposite(&self) -> Direction
    {
        return match self { Up => Down, Right => Left, Down => Up, Left => Right };
    }
}

#[derive(Copy,Clone,Hash,Eq,PartialEq,Debug)]
struct Node
{
    i: usize,
    j: usize,
    from_dir: Direction,
    straight_dist: u8
}

struct Queue
{
    heap: Vec<Node>,
    min_dist: HashMap<Node,u32>,
    heap_indexes: HashMap<Node,usize>
}

impl Queue
{
    fn new() -> Queue
    {
        return Queue { heap: Vec::new(),
                       min_dist: HashMap::new(),
                       heap_indexes: HashMap::new() };
    }

    fn get_dist(&self, node: &Node) -> Option<u32>
    {
        return self.min_dist.get(node).copied();
    }

    fn pop(&mut self) -> Option<Node>
    {
        return match self.heap.len()
        {
            0 => None,
            1 => self.heap.pop(),
            _ => {
                let v = self.heap.swap_remove(0);
                self.heapify(0);
                Some(v)
            }
        }
    }

    fn heapify(&mut self, index: usize)
    {
        let mut min_index = index;
        let mut min = self.min_dist.get(&self.heap[index]).unwrap();
        let l_index = 2 * index + 1;
        let r_index = l_index + 1;
        let len = self.heap.len();

        if l_index < len
        {
            let l_dist = self.min_dist.get(&self.heap[l_index]).unwrap();
            if l_dist < min
            {
                min = l_dist;
                min_index = l_index;
            }
        }

        if r_index < len
        {
            if self.min_dist.get(&self.heap[r_index]).unwrap() < min
            {
                min_index = r_index;
            }
        }

        if min_index != index
        {
            self.heap.swap(index, min_index);
            self.heapify(min_index);
        }
    }

    fn add_or_reduce(&mut self, node: Node, new_dist: u32)
    {
        let mut index =
            self.heap_indexes
                .get(&node)
                .map(|i| *i)
                .unwrap_or_else(||
                {
                    let i = self.heap.len();
                    self.heap.push(node);
                    self.heap_indexes.insert(node, i);
                    return i;
                });

        self.min_dist.insert(node, new_dist);

        while index > 0
        {
            let parent_index = (index - 1) / 2;
            let parent_node: Node = *self.heap.get(parent_index).unwrap();

            if self.min_dist.get(&parent_node).unwrap() <= &new_dist { break }

            self.heap.swap(index, parent_index);
            self.heap_indexes.insert(node, parent_index);
            self.heap_indexes.insert(parent_node, index);

            index = parent_index;
        }
    }
}


fn clumsy_crucible_part1(content: &str)
{
    let map: Vec<Vec<u8>> = content
        .lines()
        .map(|line|
            line
            .bytes()
            .map(|byte| byte - b'0')
            .collect())
        .collect();

    let mut steps = HashMap::<Node,Node>::new();

    let height = map.len();
    let width = map[0].len();

    let mut queue = Queue::new();
    queue.add_or_reduce(Node { i: 0,
                               j: 0,
                               from_dir: Right,
                               straight_dist: 0 }, 0);

    let mut node;
    let mut dist;

    loop
    {
        node = queue.pop().unwrap();
        let Node { i, j, .. } = node;
        dist = queue.get_dist(&node).unwrap();

        if i == height - 1 && j == width - 1
        {
            // last_node = node;
            break
        }

        // if i > 0
        //     && from_dir != Down
        //     && !(from_dir == Up && straight_dist == MAX_STRAIGHT_RUN)
        // {
        //     // println!("Up");
        //     // Up
        //     let adj_node = Node {
        //         i: i - 1,
        //         j: j,
        //         from_dir: Up,
        //         straight_dist: if from_dir == Up { straight_dist + 1 } else { 1 }
        //     };
        //
        //     let existing_dist = queue.get_dist(&adj_node).unwrap_or(u32::MAX);
        //     let new_dist = dist + map[i - 1][j] as u32;
        //     if new_dist < existing_dist
        //     {
        //         queue.add_or_reduce(adj_node, new_dist);
        //         steps.insert(adj_node, node);
        //     }
        // }

        branch(node, Up,    &map, &mut queue, &mut steps);
        branch(node, Right, &map, &mut queue, &mut steps);
        branch(node, Down,  &map, &mut queue, &mut steps);
        branch(node, Left,  &map, &mut queue, &mut steps);

        // if i < (map.len() - 1)
        //     && from_dir != Up
        //     && !(from_dir == Down && straight_dist == MAX_STRAIGHT_RUN)
        // {
        //     // println!("Down");
        //     // Down
        //     let adj_node = Node {
        //         i: i + 1,
        //         j: j,
        //         from_dir: Down,
        //         straight_dist: if from_dir == Down { straight_dist + 1 } else { 1 }
        //     };
        //
        //     let existing_dist = queue.get_dist(&adj_node).unwrap_or(u32::MAX);
        //     let new_dist = dist + map[i + 1][j] as u32;
        //     // println!("  new_dist={new_dist}, existing_dist={existing_dist}");
        //     if new_dist < existing_dist
        //     {
        //         queue.add_or_reduce(adj_node, new_dist);
        //         steps.insert(adj_node, node);
        //     }
        // }
        //
        // if j > 0
        //     && from_dir != Right
        //     && !(from_dir == Left && straight_dist == MAX_STRAIGHT_RUN)
        // {
        //     // println!("Left");
        //     // Left
        //     let adj_node = Node {
        //         i: i,
        //         j: j - 1,
        //         from_dir: Left,
        //         straight_dist: if from_dir == Left { straight_dist + 1 } else { 1 }
        //     };
        //
        //     let existing_dist = queue.get_dist(&adj_node).unwrap_or(u32::MAX);
        //     let new_dist = dist + map[i][j - 1] as u32;
        //     if new_dist < existing_dist
        //     {
        //         queue.add_or_reduce(adj_node, new_dist);
        //         steps.insert(adj_node, node);
        //     }
        // }
        //
        // if j < (map[0].len() - 1)
        //     && from_dir != Left
        //     && !(from_dir == Right && straight_dist == MAX_STRAIGHT_RUN)
        // {
        //     // println!("Right");
        //     // Right
        //     let adj_node = Node {
        //         i: i,
        //         j: j + 1,
        //         from_dir: Right,
        //         straight_dist: if from_dir == Right { straight_dist + 1 } else { 1 }
        //     };
        //
        //     let existing_dist = queue.get_dist(&adj_node).unwrap_or(u32::MAX);
        //     let new_dist = dist + map[i][j + 1] as u32;
        //     // println!("  new_dist={new_dist}, existing_dist={existing_dist}");
        //     if new_dist < existing_dist
        //     {
        //         queue.add_or_reduce(adj_node, new_dist);
        //         steps.insert(adj_node, node);
        //     }
        // }
    }

    let mut path = HashSet::<(usize,usize)>::new();
    println!("Nodes (reversed):");
    loop
    {
        println!("{:?}", node);
        let Node { i, j, .. } = node;
        path.insert((i, j));
        match steps.get(&node)
        {
            Some(prev_node) => node = *prev_node,
            None => break
        }
    }

    print_map(&map, &path);

    println!("min heat loss = {}", dist);
}

// fn branch(node: Node,
//           direction: Direction,
//           map: &Vec<Vec<u8>>,
//           queue: &mut Queue,
//           steps: &mut HashMap<Node,Node>)
// {
//     let Node { i, j, from_dir, straight_dist } = node;
//
//     if from_dir == direction.opposite() ||
//         from_dir == direction && straight_dist == MAX_STRAIGHT_RUN { return; }
//
//     let Some((i2, j2)) = direction.move_pos(i, j, map.len(), map[0].len()) else { return };
//
//     let adj_node = Node {
//         i: i2,
//         j: j2,
//         from_dir: direction,
//         straight_dist: if from_dir == direction { straight_dist + 1 } else { 1 }
//     };
//
//     let existing_dist = queue.get_dist(&adj_node).unwrap_or(u32::MAX);
//     let new_dist = queue.get_dist(&node).unwrap() + map[i2][j2] as u32;
//     if new_dist < existing_dist
//     {
//         queue.add_or_reduce(adj_node, new_dist);
//         steps.insert(adj_node, node);
//     }
// }

fn branch(node: Node,
          direction: Direction,
          map: &Vec<Vec<u8>>,
          queue: &mut Queue,
          steps: &mut HashMap<Node,Node>)
{
    let Node { i, j, from_dir, straight_dist } = node;

    if from_dir == direction.opposite() ||
        from_dir == direction && straight_dist == MAX_STRAIGHT_RUN { return; }

    let Some((i2, j2)) = direction.move_pos(i, j, map.len(), map[0].len()) else { return };

    let adj_node = Node {
        i: i2,
        j: j2,
        from_dir: direction,
        straight_dist: if from_dir == direction { straight_dist + 1 } else { 1 }
    };

    let existing_dist = queue.get_dist(&adj_node).unwrap_or(u32::MAX);
    let new_dist = queue.get_dist(&node).unwrap() + map[i2][j2] as u32;
    if new_dist < existing_dist
    {
        queue.add_or_reduce(adj_node, new_dist);
        steps.insert(adj_node, node);
    }
}


// fn branch(node: Node,
//           direction: Direction,
//           map: &Vec<Vec<u8>>,
//           queue: &mut Queue,
//           steps: &mut HashMap<Node,Node>)
// {
//     let Node { i, j, from_dir, straight_dist } = node;
//
//     if from_dir == direction.opposite() ||
//         from_dir == direction && straight_dist == MAX_STRAIGHT_RUN { return; }
//
//     let Some((i2, j2)) = direction.move_pos(i, j, map.len(), map[0].len()) else { return };
//
//     let straight_dist = if from_dir == direction { straight_dist + 1 } else { 1 };
//
//     // if any nodes already exist that are equivalent but with lower 'straight dist', then there's no point
//     // creating a new one.
//
//     let adj_node = Node {
//         i: i2,
//         j: j2,
//         from_dir: direction,
//         straight_dist: if from_dir == direction { straight_dist + 1 } else { 1 }
//     };
//
//     let new_dist = queue.get_dist(&node).unwrap() + map[i2][j2] as u32;
//     if new_dist >= queue.get_dist(&adj_node).unwrap_or(u32::MAX) { return; }
//
//     // for sd in 1..straight_dist
//     // {
//     //     if let Some(existing_dist) = queue.get_dist(&Node { straight_dist: sd, ..adj_node })
//     //     {
//     //         if new_dist >= existing_dist { return };
//     //     }
//     // }
//
//     queue.add_or_reduce(adj_node, new_dist);
//     steps.insert(adj_node, node);
//
//     // let adj_node = Node {
//     //     i: i2,
//     //     j: j2,
//     //     from_dir: direction,
//     //     straight_dist: if from_dir == direction { straight_dist + 1 } else { 1 }
//     // };
//     //
//     // let existing_dist = queue.get_dist(&adj_node).unwrap_or(u32::MAX);
//     // let new_dist = queue.get_dist(&node).unwrap() + map[i2][j2] as u32;
//     // if new_dist < existing_dist
//     // {
//     //     queue.add_or_reduce(adj_node, new_dist);
//     //     steps.insert(adj_node, node);
//     // }
// }


fn print_map(map: &Vec<Vec<u8>>, path: &HashSet<(usize,usize)>)
{
    println!();
    for (i, row) in map.iter().enumerate()
    {
        for (j, v) in row.iter().enumerate()
        {
            if path.contains(&(i, j))
            {
                print!("\x1b[44m{v}\x1b[m");
            }
            else
            {
                print!("{v}");
            }
        }
        println!();
    }
}
