use std::io;
use std::io::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let result = solve2()?;
    println!("{}", result);
    Ok(())
}

fn _solve() -> Result<i64, Box<dyn Error>> {
    let mut result: i64 = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mut found = false;
        let mut val_chars  = vec!['0', '0']; 
        for c in line?.chars() {
            if c >= '0' && c <= '9' {
                if !found {
                    val_chars[0] = c;
                }
                val_chars[1] = c;
                found = true;
            }
        }

        result += val_chars.into_iter().collect::<String>().parse::<i64>()?;
    }

    Ok(result)
}

fn solve2() -> Result<i64, Box<dyn Error>> {
    let mut result: i64 = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let (mut first, mut last) = (0, 0);
        let mut buf = ['x', 'x', 'x', 'x', 'x'];
        let mut idx = 0;
        let mut found = false;

        for c in line?.chars() {
            buf[idx] = c;
            idx = (idx + 1) % 5;
            let three = &[buf[(idx+2)%5], buf[(idx+3)%5], buf[(idx+4)%5]].iter().collect::<String>();
            let four = &[buf[(idx+1)%5], buf[(idx+2)%5], buf[(idx+3)%5], buf[(idx+4)%5]].iter().collect::<String>();
            let five = &[buf[idx], buf[(idx+1)%5], buf[(idx+2)%5], buf[(idx+3)%5], buf[(idx+4)%5]].iter().collect::<String>();
            //println!("{},{},{}", three, four, five);
            last = parse_number(&c.to_string())
                .or_else(|| parse_number(three))
                .or_else(|| parse_number(four))
                .or_else(|| parse_number(five))
                .unwrap_or(last);
            if last != 0 && !found {
                found = true;
                first = last;
            }
        }

        println!("{}{}", first, last);
        result += last;
        result += first * 10;
    }

    return Ok(result)
}

fn parse_number(input: &String) -> Option<i64> {
    if input.len() == 1 {
        return match input.parse::<i64>() {
            Err(_) => None,
            Ok(f) => Some(f),
        };
    }

    let ret = match input.as_str() {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    };

    return ret;
}