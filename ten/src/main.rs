use anyhow::{Result, anyhow};
use std::io::prelude::*;

fn main() -> Result<()> {
    let answer = solve()?;
    println!("{}", answer);
    return Ok(())
}

#[derive(Clone, Copy, PartialEq)]
enum Direction { Left, Right, Up, Down }

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

struct Grid {
    squares: Vec<u8>,
    width: usize,
    start: usize,
}

impl Grid {
    fn new() -> Grid {
        return Grid {
            squares: Vec::new(),
            width: 0,
            start: 0,
        }
    }

    fn insert_line(&mut self, line: &str) -> Result<()> {
        if self.width != 0 && line.len() != self.width {
            return Err(anyhow!("new line does not match grid width"))
        }

        match line.find('S') {
            Some(i) => { self.start = self.squares.len() + i; },
            None => {},
        }

        self.width = line.len();
        self.squares.append(&mut line.as_bytes().to_vec());
        return Ok(());
    }

    fn position_at_start<'a> (&'a self) -> GridPosition<'a> {
        return GridPosition::<'a>::new(self, self.start);
    }
}

struct GridPosition<'a> {
    grid: &'a Grid,
    start: usize,
    position: usize,
}

impl <'a> GridPosition<'a> {
    fn new(grid: &'a Grid, position: usize) -> GridPosition<'a> {
        return GridPosition{grid: grid, start: position, position: position};
    }

    fn at_start(&self) -> bool { self.position == self.start }

    fn go(&mut self, dir: Direction) -> Result<()> {
        match dir {
            Direction::Left => self.left(),
            Direction::Right => self.right(),
            Direction::Up => self.up(),
            Direction::Down => self.down(),
        }
    }

    fn up(&mut self) -> Result<()> {
        //if it would go negative from the subtraction...
        if self.position < self.grid.width {
            return Err(anyhow!("cannot go up"));
        }

        self.position -= self.grid.width;
        return Ok(());
    }

    fn right(&mut self) -> Result<()> {
        if ((self.position + 1) % self.grid.width) >= self.grid.width {
            return Err(anyhow!("cannot go right"));
        }

        self.position += 1;
        return Ok(());
    }

    fn down(&mut self) -> Result<()> {
        if self.position + self.grid.width > self.grid.squares.len() {
            return Err(anyhow!("cannot go down"));
        }

        self.position += self.grid.width;
        return Ok(());
    }

    fn left(&mut self) -> Result<()> {
        if self.position % self.grid.width == 0 {
            return Err(anyhow!("cannot go left"));
        }

        self.position -= 1;
        return Ok(());
    }

    fn get(&self) -> Result<&u8> {
        return self.grid.squares.get(self.position).ok_or(anyhow!("out of range!"));
    }
}

fn directions_for_pipe(pipe: u8) -> Option<(Direction, Direction)> {
    match pipe {
        b'|' => Some((Direction::Up, Direction::Down)),
        b'-' => Some((Direction::Left, Direction::Right)),
        b'L' => Some((Direction::Up, Direction::Right)),
        b'7' => Some((Direction::Left, Direction::Down)),
        b'F' => Some((Direction::Right, Direction::Down)),
        b'J' => Some((Direction::Left, Direction::Up)),
        _ => None,
    }
}

fn solve() -> Result<u64> {
    let stdin = std::io::stdin();
    let line_iter = stdin.lock().lines();

    //let file = std::fs::File::open("./short_input2.txt")?;
    //let line_iter = std::io::BufReader::new(file).lines();

    let mut grid = Grid::new();

    for line in line_iter {
        let line = line?;
        if line.is_empty() { continue }
        grid.insert_line(&line)?;
    }

    let mut pos = grid.position_at_start();
    let mut from_direction = go_legal_direction(&mut pos)
        .ok_or(anyhow!("no legal direction from starting position"))?;

    let mut ret = 1;
    while !pos.at_start() {
        let legal_directions = directions_for_pipe(*pos.get()?)
            .ok_or(anyhow!("ended up outside pipe!"))?;
        let mut next_direction = legal_directions.0;
        if next_direction == from_direction.opposite() {
            next_direction = legal_directions.1;
        }

        pos.go(next_direction)?;
        from_direction = next_direction;
        ret += 1;
    }

    return Ok(ret/2);
}

fn go_legal_direction(pos: &mut GridPosition) -> Option<Direction> {
    for dir in [Direction::Up, Direction::Left, Direction::Down, Direction::Right].iter() {
        if let Ok(_) = pos.go(*dir) {
            let pipe = pos.get().expect("could not get at navigated pipe");
            let legal_dirs = directions_for_pipe(*pipe); 
            if let Some(legal_dirs) = legal_dirs {
                let opposite = dir.opposite();
                if legal_dirs.0 == opposite || legal_dirs.1 == opposite {
                    return Some(*dir);
                }
            }

            let result = pos.go(dir.opposite());
            assert!(result.is_ok(), "could not go back to original position");
        }
    }

    return None;
}