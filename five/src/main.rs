use anyhow::{anyhow, Result};
use std::io::prelude::*;
use std::cmp::min;

fn main() -> Result<()>{
    let answer = solve2()?;
    println!("{}", answer);
    return Ok(());
}

fn solve1() -> Result<i64> {
    let stdin = std::io::stdin();
    let mut line_iter = stdin.lock().lines();
    let seed_line = line_iter.next()
        .ok_or(anyhow!("expected 'seeds', got <EOF>"))?
        .map_err(|e| anyhow!("reading seed line: {}", e))?;

    let mut transformed = parse_seed_line(seed_line)
        .map_err(|s| anyhow!("parsing seed line: {}", s.to_string()))?;

    //empty line
    line_iter.next().ok_or(anyhow!("expected empty line, got <EOF>"))??;

    loop {
        let (conv_rules, last) = parse_conversion_block(&mut line_iter)?;
        convert_set(&mut transformed, &conv_rules);
        if last { break; }
    }

    let mut ret = i64::MAX;
    for item in transformed {
        ret = min(ret, item);
    }

    return Ok(ret);
}

fn solve2() -> Result<i64> {
    //let file = std::fs::File::open("./short_input.txt")?;
    //let mut line_iter = std::io::BufReader::new(file).lines();
    let stdin = std::io::stdin();
    let mut line_iter = stdin.lock().lines();
    let seed_line = line_iter.next()
        .ok_or(anyhow!("expected 'seeds', got <EOF>"))?
        .map_err(|e| anyhow!("reading seed line: {}", e))?;

    let mut transformed = parse_seed_line_as_ranges(seed_line)
        .map_err(|s| anyhow!("parsing seed line: {}", s.to_string()))?;

    //empty line
    line_iter.next().ok_or(anyhow!("expected empty line, got <EOF>"))??;

    loop {
        let (conv_rules, last) = parse_conversion_block(&mut line_iter)?;
        transformed = convert_set_of_ranges(&transformed, &conv_rules);
        if last { break; }
    }

    let mut ret = i64::MAX;
    for item in transformed {
        ret = min(ret, item.0);
    }

    return Ok(ret);
}

fn parse_seed_line(line: String) -> Result<Vec<i64>> {
    let mut ret = Vec::<i64>::new();
    let mut iter = line.split(' ');
    //seeds:
    match iter.next() {
        Some(f) if f == "seeds:" =>  Ok(()),
        Some(f) => Err(anyhow!("expected token 'seeds:', got '{}'", f)),
        None => Err(anyhow!("expected token 'seeds', got <EOL>")),
    }?;

    //each seed number
    for seed in iter {
        ret.push(seed.parse::<i64>()?);
    }

    return Ok(ret);
}

fn parse_seed_line_as_ranges(line: String) -> Result<Vec<(i64, i64)>> {
    let mut ret = Vec::<(i64, i64)>::new();
    let mut iter = line.split(' ');
    //seeds:
    match iter.next() {
        Some(f) if f == "seeds:" =>  Ok(()),
        Some(f) => Err(anyhow!("expected token 'seeds:', got '{}'", f)),
        None => Err(anyhow!("expected token 'seeds', got <EOL>")),
    }?;

    //each seed number pair
    loop {
        let base = match iter.next() {
            Some(f) => f.parse::<i64>(),
            None => { break; }
        }?;

        let range_size = match iter.next() {
            Some(f) => f.parse::<i64>().map_err(anyhow::Error::msg),
            None => Err(anyhow!("expected range, got <EOL>")),
        }?;

        ret.push((base, range_size))
    }

    return Ok(ret);
}


fn parse_conversion_block(
    iter: &mut dyn Iterator<Item = Result<String, std::io::Error>>
) -> Result<(Vec<(i64, i64, i64)>, bool)> {

    let mut ret = Vec::<(i64, i64, i64)>::new();
    let mut done = false;
    let mut eof = false;
    //x-to-y map:
    iter.next()
        .ok_or(anyhow!("expected '<x>-to-<y>', got <EOF>"))?
        .map_err(|e| anyhow!("reading <x>-to-<y> line: {}", e))?;

    loop {
        let range_info = match iter.next() {
            Some(f) => {
                match f {
                    Ok(f) if f == "" => { done = true; Ok("".into()) },
                    Ok(f) => Ok(f),
                    Err(e) => Err(anyhow!("reading conversion line: {}", e)),
                }
            },
            None => { done = true; eof = true; Ok("".into()) }
        }?;

        if done { break }
        let conv_line = parse_conversion_line(range_info)?;
        ret.push(conv_line);
    }
    
    return Ok((ret, eof));
}

fn parse_conversion_line(line: String) -> Result<(i64, i64, i64)> {
    let cols = line.split(" ").collect::<Vec<&str>>();
    if cols.len() != 3 {
        return Err(anyhow!("expected 3 columns in conversion line, found {}", cols.len()))
    }

    return Ok((
        cols[0].parse::<i64>()?,
        cols[1].parse::<i64>()?,
        cols[2].parse::<i64>()?,
    ));
}

fn convert_set(input: &mut Vec<i64>, rules: &Vec<(i64, i64, i64)>) {
    for item in input {
        convert_item(item, rules)
    }
}

fn convert_item(item: &mut i64, rules: &Vec<(i64, i64, i64)>) {
    for rule in rules {
        if *item >= rule.1 && *item < rule.1 + rule.2 {
            *item = rule.0 + *item - rule.1;
            break
        }
    }
}

fn convert_set_of_ranges(input: &Vec<(i64, i64)>, rules: &Vec<(i64, i64, i64)>) -> Vec<(i64, i64)> {
    let mut ret = Vec::<(i64, i64)>::new();

    for item in input {
        convert_range(item, rules, &mut ret)
    }

    return ret;
}

fn convert_range(range: &(i64, i64), rules: &Vec<(i64, i64, i64)>, into: &mut Vec<(i64, i64)>) {
    for rule in rules {
        let (lower_opt, rest_opt) = split_range(range, rule.1);
        if rest_opt.is_none() {
            continue;
        }

        let (middle_opt, upper_opt) = split_range(
            &rest_opt.unwrap(), 
            rule.1 + rule.2,
        );
        if middle_opt.is_none() {
            continue;
        }

        let middle = middle_opt.unwrap();
        let new_base = rule.0 + middle.0 - rule.1;
        into.push((new_base, middle.1));
    
        if lower_opt.is_some() {
            convert_range(&lower_opt.unwrap(), rules, into);
        }

        if upper_opt.is_some() {
            convert_range(&upper_opt.unwrap(), rules, into);
        }

        return
    }

    into.push(*range);
}

fn split_range(range: &(i64, i64), split_line: i64) -> (Option<(i64, i64)>, Option<(i64, i64)>) {
    //This can be done with min and max somehow. Not sure if that would be faster...
    // still two comparisons? don't care now.
    if split_line <= range.0 {
        return (None, Some((range.0, range.1)));
    }

    if split_line >= range.0 + range.1 {
        return (Some((range.0, range.1)), None);
    }

    return (Some((range.0, split_line-range.0)), Some((split_line, range.0+range.1-split_line)));
}