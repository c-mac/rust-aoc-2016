use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let contents = get_contents("input-cam.txt")?;
    let parsed = contents
        .trim()
        .split("\n")
        .map(|line| {
            let mut split_line = line.split("-").collect::<Vec<&str>>();
            let sector_id_and_checksum = split_line.pop().unwrap();
            let index = sector_id_and_checksum.find('[').unwrap();
            let len = sector_id_and_checksum.len();
            let sector_id = sector_id_and_checksum[..index]
                .to_string()
                .parse::<usize>()
                .unwrap();
            let checksum = sector_id_and_checksum[index + 1..len - 1].to_string();
            let characters = split_line.join("");
            let mut frequency: HashMap<char, usize> = HashMap::new();
            for c in characters.chars() {
                *frequency.entry(c).or_insert(0) += 1
            }
            (frequency, sector_id, checksum)
        })
        .collect::<Vec<(HashMap<char, usize>, usize, String)>>();
    let acc = parsed.iter().fold(0, |a, entry| {
        let frequency = &entry.0;
        let sector_id = entry.1;
        let checksum = &entry.2;
        let mut key_values = frequency
            .iter()
            .map(|c| (*(c.0), *(c.1)))
            .collect::<Vec<(char, usize)>>();
        key_values.sort_by(|a, b| match b.1.cmp(&a.1) {
            Ordering::Equal => a.0.cmp(&b.0),
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        });
        let computed_checksum = key_values[..5].iter().map(|c| c.0).collect::<String>();
        if computed_checksum == *checksum {
            a + sector_id
        } else {
            a
        }
    });
    dbg!(acc);

    contents
        .trim()
        .split("\n")
        .map(|line| {
            let mut split_line = line.split("-").collect::<Vec<&str>>();
            let sector_id_and_checksum = split_line.pop().unwrap();
            let index = sector_id_and_checksum.find('[').unwrap();
            let sector_id = sector_id_and_checksum[..index]
                .to_string()
                .parse::<usize>()
                .unwrap();
            let decrypted = split_line
                .iter()
                .map(|word| decrypt(word, sector_id))
                .collect::<Vec<String>>();
            // if decrypted == "NORTH POLE OBJECTS".to_string().split(' ').map(|c| c.to_string()).collect::<Vec<String>>() {
            dbg!(&decrypted);
            dbg!(&sector_id);
            // }
            decrypted
        })
        .collect::<Vec<Vec<String>>>();

    Ok(())
}

static ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn decrypt(word: &str, sector_id: usize) -> String {
    word.chars()
        .map(|c| {
            ALPHABET
                .chars()
                .nth((sector_id + ALPHABET.find(c.to_uppercase().nth(0).unwrap()).unwrap()) % 26)
                .unwrap()
        })
        .collect::<String>()
}

// figure out how to put this in a aoc utility library
// in this repository
fn get_contents(fname: &str) -> Result<String> {
    let mut f = File::open(fname)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents.trim().to_string())
}
