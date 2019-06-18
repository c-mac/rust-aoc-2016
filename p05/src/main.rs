use md5::compute;

fn main() {
    // let puzzle_input = "abc";
    let puzzle_input = "wtnhxymk";
    // let mut result: Vec<char> = Vec::new();
    let mut counter = 1;
    let mut answer: [Option<char>;8] = [None;8];
    dbg!(answer);

    loop {
        let format_counter = format!("{}", counter);
        let modified_input = puzzle_input.to_string() + &format_counter;
        let hex_hash = format!("{:x}", compute(modified_input));
        let position: char;
        let value: char;

        if &hex_hash[..5] == "00000" {
            let chars = hex_hash.chars().collect::<Vec<char>>();
            position = chars[5];
            value = chars[6];

            match position.to_string().parse::<usize>() {
                Ok(n) => {
                    if n <= 7 {
                        match answer[n] {
                            Some(v) => {},
                            None => {
                                answer[n] = Some(value);
                                dbg!((hex_hash, position, value, answer));
                            }
                        }
                    }
                },
                Error => {}
            }

            // result.push(dbg!(chars.nth(5).unwrap()));
        }

        if answer.iter().all(|&x| match x {
            Some(_) => true,
            None => false
        }) {
            break;
        }

        // if result.len() == 8 {
        //     break;
        // }

        counter += 1;
    }
    dbg!(answer.iter().map(|x| x.unwrap()).collect::<String>());
    // dbg!(result.iter().collect::<String>());
}
