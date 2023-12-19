use anyhow::{Result, anyhow};
use std::io::prelude::*;

fn main() -> Result<()> {
    let answer = solve()?;
    println!("{}", answer);
    Ok(())
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
enum Rank { Jack, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Queen, King, Ace }

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
enum ScoreType { HighCard, OnePair, TwoPair, ThreeOfKind, FullHouse, FourOfKind, FiveOfKind }

struct Hand { 
    cards: [Rank; Hand::MAX_HAND_SIZE],
    bet: i32,
    len: usize,
}

impl Hand {
    const MAX_HAND_SIZE: usize = 5;

    fn new(bet: i32) -> Hand {
        return Hand{cards: [Rank::Two; Self::MAX_HAND_SIZE], bet: bet, len: 0}
    }

    fn insert(&mut self, card: Rank) -> Result<()> {
        if self.len >= Self::MAX_HAND_SIZE {
            return Err(anyhow!("Hand is already full"));
        }

        self.cards[self.len] = card; 
        self.len += 1;
        return Ok(());
    }

    fn score_type(&self) -> ScoreType {
        let mut score = ScoreType::HighCard;
        let rank_freq = self._gen_rank_frequencies();
        for rank_count in &rank_freq[Rank::Two as usize..] {
            score = match rank_count {
                2 => match score {
                    ScoreType::OnePair => ScoreType::TwoPair,
                    ScoreType::ThreeOfKind => ScoreType::FullHouse,
                    _ => ScoreType::OnePair
                },
                3 => match score {
                    ScoreType::OnePair => ScoreType::FullHouse,
                    _ => ScoreType::ThreeOfKind
                }
                4 => ScoreType::FourOfKind,
                5 => ScoreType::FiveOfKind,
                _ => score,
            }
        }
        
        //assign jokers
        for _ in 0..rank_freq[Rank::Jack as usize] {
            score = match score {
                ScoreType::HighCard => ScoreType::OnePair,
                ScoreType::OnePair => ScoreType::ThreeOfKind,
                ScoreType::TwoPair => ScoreType::FullHouse,
                ScoreType::ThreeOfKind => ScoreType::FourOfKind,
                ScoreType::FourOfKind => ScoreType::FiveOfKind,
                _ => score,
            }
        }

        return score;
    }

    fn _gen_rank_frequencies(&self) -> [u32; Rank::Ace as usize + 1] {
        let mut ret = [0; Rank::Ace as usize + 1];
        for &card in &self.cards[0..self.len] {
            ret[card as usize] += 1;
        }

        return ret;
    }

    fn cmp_high_card(&self, rhs: &Hand) -> std::cmp::Ordering {
        for i in 0..self.len {
            let c = self.cards[i].cmp(&rhs.cards[i]);
            match c {
                std::cmp::Ordering::Less | std::cmp::Ordering::Greater => return c,
                _ => {}
            }
        }

        return std::cmp::Ordering::Equal;
    }

}


impl std::cmp::Ord for Hand {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        let left_score = self.score_type() ;
        let right_score = rhs.score_type();
        let c = left_score.cmp(&right_score);
        match c {
            std::cmp::Ordering::Less | std::cmp::Ordering::Greater => return c,
            _ => {}
        }

        return self.cmp_high_card(rhs);
    }
}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(rhs)) }
}

impl std::cmp::PartialEq for Hand {
    fn eq(&self, rhs: &Self) -> bool { return self.cmp(rhs) == std::cmp::Ordering::Equal; }
}

impl std::cmp::Eq for Hand {}

fn solve() -> Result<i32> {
    let mut hands = parse_hands()?;

    let mut score = 0;
    hands.sort();
    let mut multiplier = 1;
    for hand in hands {
        score += hand.bet * multiplier;
        multiplier += 1;
    }

    return Ok(score);
}

fn parse_hands() -> Result<Vec<Hand>> {
    //let file = std::fs::File::open("./short_input.txt")?;
    //let line_iter = std::io::BufReader::new(file).lines();
    let stdin = std::io::stdin();
    let line_iter = stdin.lock().lines();
    let mut line_num = 0;
    let mut hands = Vec::<Hand>::new();
    for line in line_iter {
        line_num += 1;
        let line = line?;
        if line.is_empty() { continue }

        let hand= parse_line(line)
            .map_err(|e| anyhow!("line {}: {}", line_num, e))?;

        hands.push(hand);
    }

    return Ok(hands);
}

fn parse_line(s: String) -> Result<Hand> {
    let mut parts = s.split(' ');
    let hand_spec = parts.next().ok_or_else(|| anyhow!("no hand spec!"))?;
    let bet_size_spec = parts.next().ok_or_else(|| anyhow!("no bet!"))?;
    let bet_size = bet_size_spec.parse::<i32>()
        .map_err(|e| anyhow!("could not parse bet size: {}", e))?;
    let mut hand = Hand::new(bet_size);

    for c in hand_spec.chars() {
        let card = match c {
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            'T' => Rank::Ten,
            'J' => Rank::Jack,
            'Q' => Rank::Queen,
            'K' => Rank::King,
            'A' => Rank::Ace,
            _ => { return Err(anyhow!("unrecognized card type {}", c))},
        };

        hand.insert(card)?;
    }

    return Ok(hand);
}
