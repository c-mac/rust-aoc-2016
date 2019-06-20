use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let contents = get_contents("input.txt")?;
    let mut counts: Vec<HashMap<char, usize>> = Vec::new();
    for _ in 0..8 {
        counts.push(HashMap::new());
    }
    for word in contents.trim().split("\n") {
        for (i, c) in word.chars().enumerate() {
            let count = counts[i].entry(c).or_insert(0);
            *count += 1;
        }
    }
    let answer = counts
        .iter()
        .map(|character_count| {
            let mut key_values = character_count
                .iter()
                .map(|c| (*(c.0), *(c.1)))
                .collect::<Vec<(char, usize)>>();
            key_values.sort_by(|a, b| a.1.cmp(&b.1));
            key_values[0].0
        })
        .collect::<String>();
    dbg!(answer);
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
