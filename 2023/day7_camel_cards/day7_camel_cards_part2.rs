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
            camel_cards_part2(&content);
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

const CARDS: &str = "J23456789TQKA";

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

#[derive(Debug)]
struct Hand<'a>
{
    hand_str: &'a str,
    hand_type: HandType,
    values: Vec<u8>,
    bid: i32,
}

fn camel_cards_part2(content: &str)
{
    let mut hands: Vec<Hand> = content
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line|
        {
            let (hand_str, bid_str) = line.split_once(" ").unwrap();

            let hand: Vec<u8> = hand_str.chars().map(|c| CARDS.find(c).unwrap() as u8).collect();

            let mut kinds = vec![0; CARDS.len()];
            for card in &hand
            {
                kinds[*card as usize] += 1;
            }
            let jokers = kinds[0];
            kinds.sort_by(|a, b| b.cmp(a)); // Reverse sort; most-repeats first

            use HandType::*;
            return Hand {
                hand_str: hand_str,
                hand_type: match kinds.as_slice()
                {
                    [5, ..]                   => FiveOfAKind,
                    [4, 1, ..] if jokers == 1 => FiveOfAKind,
                    [4, 1, ..] if jokers == 4 => FiveOfAKind,
                    [3, 2, ..] if jokers == 2 => FiveOfAKind,
                    [3, 2, ..] if jokers == 3 => FiveOfAKind,

                    [4, 1, ..]                => FourOfAKind,
                    [3, 1, ..] if jokers == 1 => FourOfAKind,
                    [3, 1, ..] if jokers == 3 => FourOfAKind,
                    [2, 2, ..] if jokers == 2 => FourOfAKind,

                    [3, 2, ..]                => FullHouse,
                    [2, 2, ..] if jokers == 1 => FullHouse,
                    // If [3, 1, ..], with a joker, it's always better to form
                    // four-of-a-lind than a full-house.

                    [3, 1, ..]                => ThreeOfAKind,
                    [2, 1, ..] if jokers == 1 => ThreeOfAKind,
                    [2, 1, ..] if jokers == 2 => ThreeOfAKind,

                    [2, 2, ..]                => TwoPair,
                    // If [2, 1, ..], with a joker, it's always better to form a three-of-a-kind than a two-pair.

                    [2, 1, ..]                => OnePair,
                    [1, ..] if jokers == 1    => OnePair,

                    _                         => HighCard
                },
                values: hand,
                bid: bid_str.parse().unwrap()
            };
        })
        .collect();

    hands.sort_by(|hand1, hand2| (hand1.hand_type, &hand1.values).cmp(&(hand2.hand_type, &hand2.values)));

    let mut sum = 0;
    for (rank0, hand) in hands.iter().enumerate()
    {
        let winnings = (rank0 as i32 + 1) * hand.bid;
        sum += winnings;
        println!("{:?}, rank == {}, winnings = {}", hand, rank0 + 1, winnings);
    }

    println!("sum = {sum}");
}
