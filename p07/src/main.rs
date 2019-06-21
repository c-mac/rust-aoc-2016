use std::fs::File;
use std::io::Read;

use regex::Regex;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;


fn main() -> Result<()> {
    let re = Regex::new(r"([a-z])\w+").unwrap();
    // let hypernet_re = Regex::new(r"\[([a-z]*)\]");

    let contents = get_contents("input.txt")?;
    let mut num_ips = 0;

    for line in contents.trim().split("\n") {
        let mut has_abba = false;
        for (i, cap) in re.captures_iter(line).enumerate() {
            let m = cap.get(0).unwrap().as_str();
            if i % 2 == 0 {
                if is_abba(m) {
                    has_abba = true;
                }
            } else {
                if is_abba(m) {
                    has_abba = false;
                    break;
                }
            }
        }
        if has_abba {
            dbg!(line);
            num_ips += 1;
        }
    }
    dbg!(num_ips);
    Ok(())
}

fn is_abba(text: &str) -> bool {
    let char_vec = text.chars().collect::<Vec<char>>();
    for w in char_vec.windows(4) {
        if w[0] == w[3] && w[1] == w[2] && w[1] != w[0] {
            // dbg!(w);
            return true
        }
    }
    false
}

// figure out how to put this in a aoc utility library
// in this repository
fn get_contents(fname: &str) -> Result<String> {
    let mut f = File::open(fname)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents.trim().to_string())
}
