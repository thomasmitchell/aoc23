use anyhow::Result;
use std::io;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() -> Result<()> {
    let answer = solve2()?;
    println!("{}", answer);
    Ok(())
}

fn _solve1() -> Result<i32> {
    let stdin = io::stdin();
    let mut ret = 0;

    let mut validated_coords = HashSet::new();
    let mut numbers = Vec::new();


    let mut y_coord = 0;

    for line in stdin.lock().lines() {
        let line = line?;

        let mut current_num = String::with_capacity(5);
        let mut x_coord = -1;

        for square in line.as_bytes() {
            x_coord += 1;
            if *square >= b'0' && *square <= b'9' {
                current_num.push(*square as char);
                continue;
            }

            if current_num.len() > 0 {
                numbers.push((
                    current_num.clone(), 
                    (x_coord - current_num.len() as i32, y_coord),
                ));
                current_num.clear();
            }

            if *square == b'.' {
                continue
            }

            let (x, y) = (x_coord as i32, y_coord as i32);
            //if we're here, it's non-numeric, and not a dot, so its a symbol.
            validated_coords.insert((x-1, y-1));
            validated_coords.insert((x, y-1));
            validated_coords.insert((x+1, y-1));
            validated_coords.insert((x+1, y));
            validated_coords.insert((x+1, y));
            validated_coords.insert((x+1, y+1));
            validated_coords.insert((x, y+1));
            validated_coords.insert((x-1, y+1));
            validated_coords.insert((x-1, y));
        }

        if current_num.len() > 0 {
            numbers.push((
                current_num.clone(), 
                (x_coord - (current_num.len() - 1) as i32, y_coord),
            ));
            current_num.clear();
        }

        y_coord += 1;
    }

    for number in numbers {
        let mut validated = false;
        for i in 0..number.0.len() {
            if validated_coords.contains(&(number.1.0 + i as i32, number.1.1)) {
                validated = true;
                break;
            }
        }

        if validated {
            //unique_part_numbers.insert(number.0.parse::<i32>()?);
            ret += number.0.parse::<i32>()?;
            println!("{}, ({}, {})", number.0, number.1.0, number.1.1);
        }
    }

    return Ok(ret);
}

fn solve2() -> Result<i32> {
    let stdin = io::stdin();
    let mut ret = 0;

    let mut gear_coords = HashSet::new();
    let mut numbers = HashMap::new();


    let mut y_coord = 0;

    for line in stdin.lock().lines() {
        let line = line?;

        let mut current_num = String::with_capacity(5);
        let mut x_coord = -1;

        for square in line.as_bytes() {
            x_coord += 1;
            if *square >= b'0' && *square <= b'9' {
                current_num.push(*square as char);
                continue;
            }

            if current_num.len() > 0 {
                for i in 0..current_num.len() {
                    numbers.insert(
                        (x_coord - (i+1) as i32, y_coord),
                        current_num.clone(), 
                    );
                }
                current_num.clear();
            }

            if *square == b'*' {
                let (x, y) = (x_coord as i32, y_coord as i32);
                gear_coords.insert((x, y));
            }
        }

        if current_num.len() > 0 {
            for i in 0..current_num.len() {
                numbers.insert(
                    (x_coord - i as i32, y_coord),
                    current_num.clone(), 
                );
            }
            current_num.clear();
        }

        y_coord += 1;
    }

    let adjacent_offsets = 
    [
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
    ];

    for gear in gear_coords {
        let mut adjacent_numbers = HashSet::new();

        for offset in adjacent_offsets {
            match numbers.get(&(gear.0+offset.0, gear.1+offset.1)) {
                Some(f) => { adjacent_numbers.insert(f); },
                None => {},
            }
        }

        if adjacent_numbers.len() == 2 {
            //unique_part_numbers.insert(number.0.parse::<i32>()?);
            let mut to_add = 1;
            for num in adjacent_numbers.iter() {
                to_add *= num.parse::<i32>()?;
            }
            ret += to_add;
        }
    }

    return Ok(ret);
}