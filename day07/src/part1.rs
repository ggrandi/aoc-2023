use std::{cmp::Ordering, str::FromStr};

use anyhow::{anyhow, bail, Ok, Result};

pub fn part1(input: &str) -> Result<()> {
    let mut hands = input
        .trim()
        .lines()
        .map(FromStr::from_str)
        .collect::<Result<Vec<Hand>, _>>()?;

    hands.sort_unstable();

    println!(
        "part1: {}",
        hands
            .iter()
            .map(Hand::get_bid)
            .enumerate()
            .map(|(i, b)| (i + 1) * b)
            .sum::<usize>()
    );

    Ok(())
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Cards {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Cards {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Cards::Two),
            '3' => Ok(Cards::Three),
            '4' => Ok(Cards::Four),
            '5' => Ok(Cards::Five),
            '6' => Ok(Cards::Six),
            '7' => Ok(Cards::Seven),
            '8' => Ok(Cards::Eight),
            '9' => Ok(Cards::Nine),
            'T' => Ok(Cards::Ten),
            'J' => Ok(Cards::Jack),
            'Q' => Ok(Cards::Queen),
            'K' => Ok(Cards::King),
            'A' => Ok(Cards::Ace),
            c => bail!("couldn't find the card {}", c),
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Cards; 5],
    bid: usize,
    hand_type: HandType,
}

impl Hand {
    pub fn new(cards: [Cards; 5], bid: usize) -> Self {
        let mut num_cards = [0; 13];

        for card in cards.iter() {
            num_cards[*card as usize] += 1;
        }

        let hand_type = Hand::calculate_hand_type(num_cards);

        Self {
            cards,
            bid,
            hand_type,
        }
    }

    fn get_bid(&self) -> usize {
        self.bid
    }

    fn calculate_hand_type(num_cards: [u8; 13]) -> HandType {
        let mut num_pairs: u8 = 0;
        let mut triple = false;

        for num_card in num_cards {
            match num_card {
                5 => return HandType::FiveOfAKind,
                4 => return HandType::FourOfAKind,
                3 => triple = true,
                2 => num_pairs += 1,
                _ => {}
            }
        }

        if triple && num_pairs == 1 {
            HandType::FullHouse
        } else if triple {
            HandType::ThreeOfAKind
        } else if num_pairs == 2 {
            HandType::TwoPair
        } else if num_pairs == 1 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').ok_or(anyhow!("no space in hand"))?;

        let bid = bid.parse()?;

        if cards.len() != 5 {
            bail!("cards should be 5")
        }

        let mut ccards = [Cards::Two; 5];

        for (i, c) in cards.chars().enumerate() {
            ccards[i] = c.try_into()?;
        }

        Ok(Hand::new(ccards, bid))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {}
            o => return o,
        };

        for (scard, ocard) in self.cards.iter().zip(other.cards.iter()) {
            match scard.cmp(ocard) {
                Ordering::Equal => {}
                o => return o,
            }
        }

        Ordering::Equal
    }
}
