use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");

    // ? can allow an early return, which is why main has a return type
    let contents = get_contents("input.txt")?;
    let final_position = navigate(parse_contents(contents));
    let distance_traveled = final_position.0.abs() + final_position.1.abs();
    dbg!(distance_traveled);
    Ok(())
}

fn get_contents(fname: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(fname)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents.trim().to_string())
}

fn parse_contents(contents: String) -> Vec<(char, i32)> {
    contents
        .split(", ")
        .map(|x| {
            let mut c = x.chars();
            (
                c.next().unwrap(),
                dbg!(c.collect::<String>()).parse::<i32>().unwrap(),
            )
        })
        .collect()
}

fn navigate(directions: Vec<(char, i32)>) -> (i32, i32) {
    let mut position = (0, 0);
    let mut current_direction = 'N';

    let mut seen_positions: HashSet<(i32, i32)> = HashSet::new();

    for d in directions {
        match (d.0, current_direction) {
            ('L', 'N') | ('R', 'S') => {
                current_direction = 'W';
            }
            ('R', 'N') | ('L', 'S') => {
                current_direction = 'E';
            }
            ('L', 'E') | ('R', 'W') => {
                current_direction = 'N';
            }
            ('R', 'E') | ('L', 'W') => {
                current_direction = 'S';
            }
            _ => unreachable!(),
        }
        if let Some(p) = take_step(d.1, &mut position, &mut seen_positions, current_direction) {
            return p;
        }
    }
    position
}

fn take_step(
    num_steps: i32,
    position: &mut (i32, i32),
    seen_positions: &mut HashSet<(i32, i32)>,
    direction: char,
) -> Option<(i32, i32)> {
    for _ in 0..num_steps {
        if seen_positions.contains(&position) {
            return Some(*position);
        } else {
            seen_positions.insert(*position);
        }

        match direction {
            'N' => {
                position.1 += 1;
            }
            'S' => {
                position.1 -= 1;
            }
            'E' => {
                position.0 += 1;
            }
            'W' => {
                position.0 -= 1;
            }
            _ => unreachable!(),
        }
    }
    None
}
