use anyhow::{Result, anyhow};
use std::{io::prelude::*, collections::HashMap};

fn main() -> Result<()> {
    let answer = solve2()?;
    println!("{}", answer);
    return Ok(());
}

enum Direction { Left, Right }

fn solve() -> Result<i32> {
    let (directions, nodes) = parse_input()?;

    let mut ret = 0;
    let mut current_key = [b'A', b'A', b'A'];
    for direction in directions.iter().cycle() {
        ret += 1;

        let index = nodes.get(&current_key)
            .ok_or(anyhow!("no node found for {}", String::from_utf8(current_key.to_vec())?))?;

        current_key = match direction {
            Direction::Left => index.0,
            Direction::Right => index.1,
        };

        if current_key == [b'Z', b'Z', b'Z'] {
            break
        }
    }

    return Ok(ret);
}

fn solve2() -> Result<u64> {
    let (directions, nodes) = parse_input()?;

    let mut current_keys: Vec<&[u8; 3]> = nodes.keys()
        .filter(|k| k[2] == b'A')
        .collect();
    let mut ret = 1;
    let mut iteration = 0;
    let mut remaining = current_keys.len();
    'outer: for direction in directions.iter().cycle() {
        iteration += 1;

        for current_key in current_keys.iter_mut() {
            if current_key[2] == b'Z' {
                continue
            }

            let index = nodes.get(*current_key)
                .ok_or(anyhow!("no node found for {}", String::from_utf8(current_key.to_vec())?))?;

            *current_key = match direction {
                Direction::Left => &index.0,
                Direction::Right => &index.1,
            };

            if current_key[2] == b'Z' { 
                ret = num::integer::lcm(ret, iteration);

                remaining -= 1;
                if remaining == 0 {
                    break 'outer
                }
            };
        }
    }

    return Ok(ret);
}

fn parse_input() -> Result<(Vec<Direction>, HashMap<[u8;3], ([u8;3], [u8;3])>)> {
    let stdin = std::io::stdin();
    let mut line_iter = stdin.lock().lines();

    let direction_input = line_iter.next().ok_or(anyhow!("no direction line given!"))??;
    let directions = parse_direction_line(&direction_input)?;
    let mut nodes = HashMap::new();
    
    for line in line_iter {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let (key, left, right) = parse_node_line(&line)?;
        nodes.insert(key, (left, right));
    }

    return Ok((directions, nodes))
}

fn parse_direction_line(line: &str) -> Result<Vec<Direction>> {
    let mut ret = Vec::new();
    for b in line.bytes() {
        let to_append = match b {
            b'L' => Ok(Direction::Left),
            b'R' => Ok(Direction::Right),
            _ => Err(anyhow!("unknown byte in direction string: {}", b))
        }?;

        ret.push(to_append);
    }

    return Ok(ret);
}

fn parse_node_line(line: &str) -> Result<([u8; 3], [u8; 3], [u8; 3])> {
    let mut byte_iter = line.bytes();
    let mut key = [0; 3];
    for i in 0..3 {
        key[i] = byte_iter.next().ok_or(anyhow!("ran out of bytes parsing key"))?;
    }

    let mut byte_iter = byte_iter.skip(4);
    let mut left = [0; 3];
    for i in 0..3 {
        left[i] = byte_iter.next().ok_or(anyhow!("ran out of bytes parsing left"))?;
    }


    let mut byte_iter = byte_iter.skip(2);
    let mut right = [0; 3];
    for i in 0..3 {
        right[i] = byte_iter.next().ok_or(anyhow!("ran out of bytes parsing right"))?;
    }

    return Ok((key, left, right));
}
