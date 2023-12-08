use std::{io, collections::HashSet};
use std::io::prelude::*;
use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let answer = solve2()?;
    println!("{}", answer);
    return Ok(());
}

fn solve1() -> Result<i32> {
    let mut ret = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let mut line_parts = line.split(' ').filter(|s| !s.is_empty() );

        parse_card_id(&mut line_parts)
            .map_err(|e| anyhow!("error parsing card id: {}", e.to_string()))?;

        let winning_numbers = parse_winning_numbers(&mut line_parts)
            .map_err(|e| anyhow!("error parsing winning numbers: {}", e.to_string()))?;

        let given_numbers = parse_given_numbers(&mut line_parts)
            .map_err(|e| anyhow!("error parsing given numbers: {}", e.to_string()))?;

        ret += score(&winning_numbers, &given_numbers);
    }

    return Ok(ret);
}

fn solve2() -> Result<i32> {
    let mut copies = Vec::<i32>::new();
    let stdin = io::stdin();
    let mut card_idx = 0 as usize;
    for line in stdin.lock().lines() {
        let line = line?;
        let mut line_parts = line.split(' ').filter(|s| !s.is_empty() );

        parse_card_id(&mut line_parts)
            .map_err(|e| anyhow!("error parsing card id: {}", e.to_string()))?;

        let winning_numbers = parse_winning_numbers(&mut line_parts)
            .map_err(|e| anyhow!("error parsing winning numbers: {}", e.to_string()))?;

        let given_numbers = parse_given_numbers(&mut line_parts)
            .map_err(|e| anyhow!("error parsing given numbers: {}", e.to_string()))?;

        match copies.get(card_idx) {
            Some(f) => { copies[card_idx] = f + 1; },
            None => { copies.push(1); },
        };

        let num_matches = count_matches(&winning_numbers, &given_numbers);
        for i in card_idx+1..card_idx+1+(num_matches as usize) {
            match copies.get(i) {
                Some(f) => { copies[i] = f + copies[card_idx]; },
                None => { copies.push(copies[card_idx]); },
            };
        }

        card_idx += 1;
    }

    let mut ret = 0;
    for copies_of_ticket in copies.iter() {
        ret += copies_of_ticket;
    }

    return Ok(ret);
}

fn parse_card_id(iter: &mut dyn Iterator<Item = &str>) -> Result<i32> {
    match iter.next() {
        Some(f) if f == "Card" =>
            Ok(f),
        Some(f) =>
            Err(anyhow!("expected 'Card', got {}", f)),
        None =>
            Err(anyhow!("expected 'Card', got <EOL>")),
    }?;

    return match iter.next() {
        Some(f) => {
            let f = f.trim_end_matches(":");
            Ok(f.parse::<i32>()?)
        }
        None => Err(anyhow!("expected <id>, got <EOL>")),
    };
}

fn parse_winning_numbers(iter: &mut dyn Iterator<Item = &str>) -> Result<HashSet<i32>> {
    let mut ret = HashSet::<i32>::new();
    while let Some(val) = iter.next() {
        if val == "|" {
            return Ok(ret);
        }

        ret.insert(val.parse::<i32>()?);
    }

    return Err(anyhow!("Unexpected <EOL> when parsing winning numbers"));
}

fn parse_given_numbers(iter: &mut dyn Iterator<Item = &str>) -> Result<HashSet<i32>> {
    let mut ret = HashSet::<i32>::new();
    while let Some(val) = iter.next() {
        ret.insert(val.parse::<i32>()?);
    }

    return Ok(ret);
}

fn count_matches(winning: &HashSet<i32>, scoring: &HashSet<i32>) -> i32 {
    let mut ret = 0;
    for num in scoring.iter() {
        if winning.contains(num) {
            ret += 1;
        }
    }

    return ret;
}

fn score(winning: &HashSet<i32>, scoring: &HashSet<i32>) -> i32 {
    return (1 << count_matches(winning, scoring)) >> 1;
}
