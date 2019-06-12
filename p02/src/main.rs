use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let state_map: HashMap<u32, [(char, Option<u32>); 4]> = [
        (
            1,
            [('U', None), ('R', Some(2)), ('D', Some(4)), ('L', None)],
        ),
        (
            2,
            [('U', None), ('R', Some(3)), ('D', Some(5)), ('L', Some(1))],
        ),
        (
            3,
            [('U', None), ('R', None), ('D', Some(6)), ('L', Some(2))],
        ),
        (
            4,
            [('U', Some(1)), ('R', Some(5)), ('D', Some(7)), ('L', None)],
        ),
        (
            5,
            [
                ('U', Some(2)),
                ('R', Some(6)),
                ('D', Some(8)),
                ('L', Some(4)),
            ],
        ),
        (
            6,
            [('U', Some(3)), ('R', None), ('D', Some(9)), ('L', Some(5))],
        ),
        (
            7,
            [('U', Some(4)), ('R', Some(8)), ('D', None), ('L', None)],
        ),
        (
            8,
            [('U', Some(5)), ('R', Some(9)), ('D', None), ('L', Some(7))],
        ),
        (
            9,
            [('U', Some(6)), ('R', None), ('D', None), ('L', Some(8))],
        ),
    ]
    .iter()
    .cloned()
    .collect();
    let mut state_machine: Vec<State> = Vec::new();
    for i in 1..10 {
        state_machine.push(State {
            value: i,
            neighbors: HashMap::new(),
        })
    }
    for (index, neighbors) in state_map {
        add_neighbors(&mut state_machine, index as usize, &neighbors);
    }
    let fname = "input.txt";
    let contents = get_contents(fname)?;
    let mut ret: Vec<u32> = Vec::new();
    let mut state = &state_machine[4];
    for line in contents.split("\n") {
        for direction in line.chars() {
            if let Some(next_state) = state.neighbors[&direction] {
                state = &state_machine[(next_state - 1) as usize];
            }
        }
        ret.push(state.value);
    }
    dbg!(ret);
    Ok(())
}

// figure out how to put this in a aoc utility library
// in this repository
fn get_contents(fname: &str) -> Result<String> {
    let mut f = File::open(fname)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents.trim().to_string())
}

fn add_neighbors(state_machine: &mut Vec<State>, index: usize, neighbors: &[(char, Option<u32>)]) {
    let state = &mut state_machine[index - 1];
    for neighbor in neighbors {
        state.neighbors.insert(neighbor.0, neighbor.1);
    }
}

#[derive(Debug)]
struct State {
    value: u32,
    neighbors: HashMap<char, Option<u32>>,
}
