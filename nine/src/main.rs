use anyhow::{Result, anyhow};
use std::io::prelude::*;

fn main() -> Result<()> {
    let answer = solve2()?;
    println!("{}", answer);
    return Ok(());
}

fn solve() -> Result<i64> {
    let stdin = std::io::stdin();
    let line_iter = stdin.lock().lines();

    //let file = std::fs::File::open("./short_input.txt")?;
    //let line_iter = std::io::BufReader::new(file).lines();

    let mut ret = 0;
    for line in line_iter {
        let line = line?;
        if line.is_empty() { continue; }

        let sequence = parse_line(&line)?;
        let next_num = extrapolate_next(&sequence);
        ret += next_num;
    }

    return Ok(ret);
}

fn solve2() -> Result<i64> {
    let stdin = std::io::stdin();
    let line_iter = stdin.lock().lines();

    //let file = std::fs::File::open("./short_input.txt")?;
    //let line_iter = std::io::BufReader::new(file).lines();

    let mut ret = 0;
    for line in line_iter {
        let line = line?;
        if line.is_empty() { continue; }

        let mut sequence = parse_line(&line)?;
        sequence.reverse();
        let next_num = extrapolate_next(&sequence);
        ret += next_num;
    }

    return Ok(ret);
}

fn parse_line(line: &str) -> Result<Vec<i64>> {
    let mut ret = Vec::new();
    let parts = line.split(' ');
    for num in parts {
        ret.push(num.parse::<i64>()?);
    }

    return Ok(ret);
}

fn extrapolate_next(seq: &Vec<i64>) -> i64 {
    return seq.last().unwrap_or(&0) + get_next_change(seq);
}

fn get_next_change(seq: &Vec<i64>) -> i64 {
    if seq.len() <= 1 {
        return 0;
    }

    let mut has_non_zero = false;
    for num in seq.iter() {
        if *num != 0 {
            has_non_zero = true;
        }
    }

    if !has_non_zero {
        return 0;
    }

    let mut integral_seq = Vec::with_capacity(seq.len() - 1);
    let mut iter = seq.iter();
    let mut prev = iter.next().unwrap_or(&0);
    while let Some(this) = iter.next() {
        integral_seq.push(this-prev);
        prev = this;
    }

    return get_next_change(&integral_seq) + integral_seq.last().unwrap_or(&0);
}