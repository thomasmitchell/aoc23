use anyhow::{Result, anyhow};
use std::io::prelude::*;

fn main() -> Result<()> {
    let answer = solve2()?;
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

    fn with_dimensions(area: usize, width: usize) -> Grid {
        assert!(area % width == 0, "area not evenly divisible by width");
        return Grid {
            squares: vec![b'.'; area],
            width: width,
            start: 0,
        }
    }

    fn area(&self) -> usize { self.squares.len() }
    fn width(&self) -> usize { self.width }
    fn get(&self, position: usize) -> Result<&u8> {
        return self.squares.get(position).ok_or(anyhow!("tried to get out of bounds"));
    }

    fn set(&mut self, position: usize, val: u8) -> Result<()> {
        *self.squares.get_mut(position)
            .ok_or(anyhow!("set out of bounds"))? = val;
        if val == b'S' {
            self.start = position;
        }

        return Ok(());
    }

    fn print(&self) {
        let mut position = 0;
        while position < self.area() {
            let line: String = String::from_utf8(self.squares[position..position + self.width].to_vec())
                .expect("out of range on print");
            println!("{}", line);
            position += self.width;
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

    fn position(&self) -> usize { self.position }
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

fn parse_grid() -> Result<Grid> {
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

    return Ok(grid);
}

fn solve() -> Result<u64> {
    let grid = parse_grid()?;
    let mut ret = 0;
    walk_grid_map(&grid, |_| ret += 1)?;
    return Ok(ret/2);
}

fn solve2() -> Result<u64> {
    let grid = parse_grid()?;
    let mut clean_grid = Grid::with_dimensions(grid.area(), grid.width());
    walk_grid_map(&grid, |pos| {
        if let Ok(pipe) = pos.get() {
            clean_grid.set(pos.position(), *pipe).expect("could not set section of grid");
        }
    })?;

    let start_form = determine_start_form(&mut clean_grid)?;
    clean_grid.set(clean_grid.start, start_form)?;
    let ret = count_interior(&mut clean_grid)?;
    clean_grid.print();
    return Ok(ret);
}

fn walk_grid_map<F> (grid: &Grid, mut func: F) -> Result<()>
    where F: FnMut(&GridPosition)
{
    let mut pos = grid.position_at_start();
    let mut from_direction = go_legal_direction(&mut pos)
        .ok_or(anyhow!("no legal direction from starting position"))?;

    while !pos.at_start() {
        func(&pos);

        let legal_directions = directions_for_pipe(*pos.get()?)
            .ok_or(anyhow!("ended up outside pipe!"))?;
        let mut next_direction = legal_directions.0;
        if next_direction == from_direction.opposite() {
            next_direction = legal_directions.1;
        }

        pos.go(next_direction)?;
        from_direction = next_direction;
    }

    func(&pos);

    return Ok(())
}

fn count_interior(grid: &mut Grid) -> Result<u64> {
    let mut inside = false;
    let mut ret = 0;

    for i in 0..grid.area() {
        if i % grid.width() == 0 {
            inside = false;
        }

        let pipe = grid.get(i)?;
        match pipe {
            b'L' | b'J' | b'|'  => {
                inside = !inside;
            }
            b'.' => {
                if inside {
                    ret += 1;
                    grid.set(i, b'O')?;
                }
            }
            _ => {},
        }
    }

    return Ok(ret);
}

fn go_legal_direction(pos: &mut GridPosition) -> Option<Direction> {
    for dir in [Direction::Up, Direction::Left, Direction::Down, Direction::Right].iter() {
        if is_legal_direction(pos, *dir) {
            pos.go(*dir).expect("could not return to original location");
            return Some(*dir);
        }
    }

    return None;
}

fn determine_start_form(grid: &mut Grid) -> Result<u8> {
    let mut pos = grid.position_at_start();
    let mut valid_dirs = [Direction::Down; 2];
    let mut idx = 0;
    for dir in [Direction::Up, Direction::Left, Direction::Down, Direction::Right].iter() {
        if is_legal_direction(&mut pos, *dir) {
            valid_dirs[idx] = *dir;
            idx += 1;
        }
    }

    return match valid_dirs {
        [Direction::Up, Direction::Left] => Ok(b'J'),
        [Direction::Up, Direction::Down] => Ok(b'|'),
        [Direction::Up, Direction::Right] => Ok(b'L'),
        [Direction::Left, Direction::Down] => Ok(b'7'),
        [Direction::Left, Direction::Right] => Ok(b'-'),
        [Direction::Down, Direction::Right] => Ok(b'F'),
        _ => Err(anyhow!("unknown start character")),
    }
}

fn is_legal_direction(pos: &mut GridPosition, dir: Direction) -> bool {
    if let Ok(_) = pos.go(dir) {
        let pipe = pos.get().expect("could not get at navigated pipe").clone();
        let result = pos.go(dir.opposite());
        assert!(result.is_ok(), "could not go back to original position");

        let legal_dirs = directions_for_pipe(pipe); 

        if let Some(legal_dirs) = legal_dirs {
            let opposite = dir.opposite();
            if legal_dirs.0 == opposite || legal_dirs.1 == opposite {
                return true;
            }
        }
    }

    return false;
}