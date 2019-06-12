use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

// From: https://stackoverflow.com/questions/28392008/more-concise-hashmap-initialization
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

fn main() -> Result<()> {
    let states = hashmap![
        'A' => hashmap![
            'U' => Some('6'),
            'R' => Some('B'),
            'D' => None,
            'L' => None
        ],
        'B' => hashmap![
            'U' => Some('7'),
            'R' => Some('C'),
            'D' => Some('D'),
            'L' => Some('A')
        ],
        'C' => hashmap![
            'U' => Some('8'),
            'R' => None,
            'D' => None,
            'L' => Some('B')
        ],
        'D' => hashmap![
            'U' => Some('B'),
            'R' => None,
            'D' => None,
            'L' => None
        ],
        '1' => hashmap![
            'U' => None,
            'R' => None,
            'D' => Some('3'),
            'L' => None
        ],
        '2' => hashmap![
            'U' => None,
            'R' => Some('3'),
            'D' => Some('6'),
            'L' => None
        ],
        '3' => hashmap![
            'U' => Some('1'),
            'R' => Some('4'),
            'D' => Some('7'),
            'L' => Some('2')
        ],
        '4' => hashmap![
            'U' => None,
            'R' => None,
            'D' => Some('8'),
            'L' => Some('3')
        ],
        '5' => hashmap![
            'U' => None,
            'R' => Some('6'),
            'D' => None,
            'L' => None
        ],
        '6' => hashmap![
            'U' => Some('2'),
            'R' => Some('7'),
            'D' => Some('A'),
            'L' => Some('5')
        ],
        '7' => hashmap![
            'U' => Some('3'),
            'R' => Some('8'),
            'D' => Some('B'),
            'L' => Some('6')
        ],
        '8' => hashmap![
            'U' => Some('4'),
            'R' => Some('9'),
            'D' => Some('C'),
            'L' => Some('7')
        ],
        '9' => hashmap![
            'U' => None,
            'R' => None,
            'D' => None,
            'L' => Some('8')
        ]
    ];
    let fname = "input-cam.txt";
    let contents = get_contents(fname)?;
    let mut ret: Vec<char> = Vec::new();
    // Initial state
    let mut state_char = '5';
    let mut state = &states[&state_char];
    for line in contents.split("\n") {
        for direction in line.chars() {
            if let Some(next_state_char) = state[&direction] {
                state_char = next_state_char;
                state = &states[&state_char];
            }
        }
        ret.push(state_char);
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
