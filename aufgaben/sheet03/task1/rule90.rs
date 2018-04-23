//! Task 3.1: Rule 90

fn main() {
    fn print_line(line: &[bool]) {
        for b in line {
            print!("{}", if *b { "█" } else { " " });
        }
        println!("");
    }
    let mut input = read_input();

    for _ in 0..20 {
        print_line( &input );
        input = next_step( &input );
    }
}

/// Reads a valid initial configuration for our automaton from the terminal.
fn read_input() -> Vec<bool> {
    // This tries to read a string from the terminal, checks whether it's
    // valid (only contains 1's and 0's). If the user fails to input a correct
    // string, this routine will ask again until the user finally manages to
    // give us a correct string.
    //
    // You don't need to understand this routine yet; that's why I've written
    // it already ;-)
    //
    // You only need to use the `input` variable (of type `String`). You can
    // also assume that it only contains '0' and '1' chars.
    let input = {
        let mut buffer = String::new();

        loop {
            println!("Please give me the initial configuration (a string of '0' and '1'!):");
            buffer.clear();

            // `read_line` returns an error if the input isn't valid UTF8 or if
            // a strange IO error occured. We just panic in that case...
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("something went seriously wrong :O");

            if buffer.trim().chars().all(|c| c == '1' || c == '0') {
                break;
            }
        }

        buffer.trim().to_string()
    };

    let mut res = Vec::new();
    for c in input.chars() {
        res.push(if c == '0' { false } else { true });
    }
    res
}

fn next_step(line: &[bool]) -> Vec<bool> {
    fn get_one(p: bool, c: bool, n: bool) -> bool {
        if p && c && n {
            return false;
        } else if p && c && !n {
            return true;
        } else if p && !c && n {
            return false;
        } else if p && !c && !n {
            return true;
        } else if !p && c && n {
            return true;
        } else if !p && c && !n {
            return false;
        } else if !p && !c && n {
            return true;
        } else {
            return false;
        }
    }

    let mut res = Vec::new();
    for i in 0..line.len() {
        res.push(get_one(
            line[(line.len() + i - 1) % line.len()],
            line[i],
            line[(i + 1) % line.len()],
        ));
    }
    res
}

#[test]
fn rule90_rules() {
    assert_eq!(next_step(&[false, false, false]), vec![false, false, false]);
    assert_eq!(next_step(&[true, false, false]), vec![false, true, true]);
    assert_eq!(next_step(&[true, true, false]), vec![true, true, false]);
    assert_eq!(next_step(&[true, true, true]), vec![false, false, false]);
}
