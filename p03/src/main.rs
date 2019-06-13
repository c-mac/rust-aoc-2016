use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let fname = "input.txt";
    let contents = get_contents(fname)?;

    let cleaned_contents: Vec<&str> = contents.trim().split("\n").collect();
    let joined_contents = cleaned_contents.join(" ");
    let sides = joined_contents.split_whitespace().collect::<Vec<&str>>();
    let mut triangles: Vec<Vec<u32>> = Vec::new();
    for nine_sides in sides.chunks(9) {
        triangles.extend(process_chunk(&nine_sides));
    }
    dbg!(process_triangles(triangles));
    Ok(())
}

fn process_chunk(chunk: &[&str]) -> Vec<Vec<u32>> {
    vec![
        vec![chunk.get(0).unwrap().parse::<u32>().unwrap(), chunk.get(3).unwrap().parse::<u32>().unwrap(), chunk.get(6).unwrap().parse::<u32>().unwrap()],
        vec![chunk.get(1).unwrap().parse::<u32>().unwrap(), chunk.get(4).unwrap().parse::<u32>().unwrap(), chunk.get(7).unwrap().parse::<u32>().unwrap()],
        vec![chunk.get(2).unwrap().parse::<u32>().unwrap(), chunk.get(5).unwrap().parse::<u32>().unwrap(), chunk.get(8).unwrap().parse::<u32>().unwrap()],
    ]
}

fn process_triangles(data: Vec<Vec<u32>>) -> u32 {
    data.into_iter().map(|x| is_triangle(x))
        .collect::<Vec<u32>>().iter().sum::<u32>()
}

fn is_triangle(sides: Vec<u32>) -> u32 {
    let mut max = 0;
    let mut max_index = 0;
    for (i, side) in sides.iter().enumerate() {
        if *side > max {
            max = *side;
            max_index = i;
        }
    }

    match max_index {
        0 => {
            (sides[1] + sides[2] > max) as u32
        },
        1 => {
            (sides[0] + sides[2] > max) as u32
        },
        2 => {
            (sides[0] + sides[1] > max) as u32
        },
        _ => unreachable!()
    }
}

// figure out how to put this in a aoc utility library
// in this repository
fn get_contents(fname: &str) -> Result<String> {
    let mut f = File::open(fname)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents.trim().to_string())
}
