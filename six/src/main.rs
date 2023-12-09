use anyhow::{anyhow, Result};
use std::io::prelude::*;

fn main() -> Result<()> {
    let answer = solve2()?;
    println!("{}", answer);
    Ok(())
}

fn solve1() -> Result<i64> {
    let mut ret = 1;
    let stdin = std::io::stdin();
    let mut line_iter = stdin.lock().lines();
    let time_line = line_iter.next().ok_or(anyhow!("no time line"))??;
    let times = parse_line(&time_line)?;
    let dist_line = line_iter.next().ok_or(anyhow!("no dist line"))??;
    let dists = parse_line(&dist_line)?;

    for i in 0..times.len() {
        let solution_count = num_solutions(times[i], dists[i]);
        ret *= solution_count;
    }

    return Ok(ret);
}

fn solve2() -> Result<i64> {
    let stdin = std::io::stdin();
    let mut line_iter = stdin.lock().lines();
    let time_line = line_iter.next().ok_or(anyhow!("no time line"))??;
    let time = parse_line_combine_nums(&time_line)?;
    let dist_line = line_iter.next().ok_or(anyhow!("no dist line"))??;
    let dist = parse_line_combine_nums(&dist_line)?;

    return Ok(num_solutions(time, dist));
}

fn parse_line(line: &String) -> Result<Vec<i64>> {
    let mut iter = line.split(' ').filter(|s| !s.is_empty() );
    //throw away label
    iter.next().ok_or(anyhow!("No label at start of line"))?;
    let mut ret = Vec::<i64>::new();

    for num in iter {
        ret.push(num.parse::<i64>()?);
    }

    return Ok(ret);
}

fn parse_line_combine_nums(line: &String) -> Result<i64> {
    let mut iter = line.split(' ').filter(|s| !s.is_empty() );
    //throw away label
    iter.next().ok_or(anyhow!("No label at start of line"))?;
    let mut buf = String::new();

    for num in iter {
        buf.push_str(num);
    }

    return buf.parse::<i64>().map_err(|e| anyhow!("parsing number: {}", e));
}

fn num_solutions(time: i64, record_dist: i64) -> i64 {
    let mut ret = 0;
    let mut last_dist = 0;
    for i in 1..time {
        let dist = dist_for(i, time);
        if dist > record_dist {
            ret += 1;
            last_dist = dist;
            continue;
        }

        if dist < last_dist {
            break;
        }
    }

    return ret;
}

fn dist_for(hold_time: i64, total_time: i64) -> i64 {
    let go_time = total_time - hold_time;
    return go_time * hold_time;
}