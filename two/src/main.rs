use std::io;
use std::io::prelude::*;
use anyhow::{Result, anyhow};
use std::cmp;

enum Cube {
    Red,
    Blue,
    Green,
    _NumColors,
}

fn main() -> Result<()> {
    let answer = solve2()?;
    println!("{}", answer);
    Ok(())
}

/* neat, but anyhow! exists, it turns out
macro_rules! errorf {
    ($($t:tt)+) => {
        Err(Box::<dyn Error>::from(format!($($t)*)))
    };
}
*/

fn solve1() -> Result<i32> {
    let stdin = io::stdin();
    let mut ret = 0;
    for line in stdin.lock().lines() {
        let line = &line?;
        //line looks like:
        //Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let mut iter = 
            line.split(&[' ', ':'])
                .filter(|s| !s.is_empty());
        let mut counts = [0; Cube::_NumColors as usize];
        let id = parse_game_id(&mut iter)?;
        let mut done = false;
        while !done {
            let hand_counts;
            (hand_counts, done) = parse_hand(&mut iter)?;
            for i in 0..(Cube::_NumColors as usize) {
                counts[i] = cmp::max(hand_counts[i], counts[i])
            }
        }

        if is_legal_game(counts) {
            ret += id;
        }
    }

    return Ok(ret);
}

fn solve2() -> Result<i32> {
    let stdin = io::stdin();
    let mut ret = 0;
    for line in stdin.lock().lines() {
        let line = &line?;
        //line looks like:
        //Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let mut iter = 
            line.split(&[' ', ':'])
                .filter(|s| !s.is_empty());
        let mut counts = [0; Cube::_NumColors as usize];
        parse_game_id(&mut iter)?; //remove the game id
        let mut done = false;
        while !done {
            let hand_counts;
            (hand_counts, done) = parse_hand(&mut iter)?;
            for i in 0..(Cube::_NumColors as usize) {
                counts[i] = cmp::max(hand_counts[i], counts[i])
            }
        }

        ret += get_power(counts);
    }

    return Ok(ret);
}

fn parse_game_id(iter: &mut dyn Iterator<Item = &str>) -> Result<i32> {
    match iter.next() {
        Some(f) if f == "Game" => 
            Ok(f),
        Some(f) => 
            Err(anyhow!("expected 'Game', got {}", f)),
        None => 
            Err(anyhow!("expected 'Game', got <EOL>")),
    }?;

    let id = match iter.next() {
        Some(f) =>
            f.parse::<i32>().map_err(anyhow::Error::msg),
        None => Err(anyhow!("expected <id>, got <EOL>")),
    }?;

    return Ok(id);
}

#[derive(PartialEq)]
enum CubeSpecStatus {
    EndPile,
    EndHand,
    EndGame,
}

//bool returned is true if this was the end of the line
fn parse_hand(iter: &mut dyn Iterator<Item = &str>) -> Result<([i32; Cube::_NumColors as usize], bool)> {
    let mut counts = [0; Cube::_NumColors as usize];
    let mut status = CubeSpecStatus::EndPile;
    while status == CubeSpecStatus::EndPile {
        let (num_cubes, cube_color);
        (num_cubes, cube_color, status) = parse_pile(iter)?;
        counts[cube_color as usize] = num_cubes;
    }

    return Ok((counts, status == CubeSpecStatus::EndGame))
}

fn parse_pile(iter: &mut dyn Iterator<Item = &str>) -> Result<(i32, Cube, CubeSpecStatus)> {
    let num_cubes = match iter.next() {
        Some(f) =>
            f.parse::<i32>().map_err(anyhow::Error::msg),
        None =>
            Err(anyhow!("expected number of cubes, got <EOL>")),
    }?;

    let cube_color_spec = match iter.next() {
        Some(f) => Ok(f),
        None =>
            Err(anyhow!("expected cube color, got <EOL>")),
    }?;

    let color_terminator = cube_color_spec.as_bytes()[cube_color_spec.len()-1];
    //println!("`{}'", color_terminator);
    let status = match color_terminator {
        b',' => CubeSpecStatus::EndPile,
        b';' => CubeSpecStatus::EndHand,
        _    => CubeSpecStatus::EndGame,
    };

    let cube_color_word = cube_color_spec.trim_end_matches(&[',',';']);
    let cube_color = match cube_color_word {
        "red" => Ok(Cube::Red),
        "blue" => Ok(Cube::Blue),
        "green" => Ok(Cube::Green),
        _ => Err(anyhow!("unknown cube color: `{}'", cube_color_word))
    }?;

    return Ok((num_cubes, cube_color, status));
}

fn is_legal_game(colors: [i32;3]) -> bool {
    let (max_red, max_green, max_blue) = (12, 13, 14);
    return
        colors[Cube::Red as usize] <= max_red &&
        colors[Cube::Blue as usize] <= max_blue &&
        colors[Cube::Green as usize] <= max_green;
}

fn get_power(colors: [i32;3]) -> i32 {
    return colors[Cube::Red as usize] * colors[Cube::Blue as usize] * colors[Cube::Green as usize];
}