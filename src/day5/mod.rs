pub fn solve() {
    let path = std::path::Path::new("inputs/day5.txt");
    let input = std::fs::read_to_string(path).expect("Something went wrong when reading the file");
    let mut nice_string = 0;

    for line in input.lines() {
        let input_chars: Vec<char> = line.chars().collect();

        if is_restricted_letters(line)
            && is_char_appears_twice(&input_chars)
            && is_contain_three_vowels(&input_chars)
        {
            nice_string += 1;
        }
    }
    println!("part1 {nice_string}");
    assert_eq!(258, nice_string, "part 1 solution should be 258");

    // part 2 notworking
    for line in input.lines() {
        let input_chars: Vec<char> = line.chars().collect();

        if is_contain_repeat_letter(&input_chars) && has_pairof_repeating_letters(line) {
            nice_string += 1;
        }
    }
    println!("part2 {nice_string}")
}

fn is_contain_repeat_letter(input: &Vec<char>) -> bool {
    let mut does_repeat = false;
    for (index, letter) in input.into_iter().enumerate() {
        if index < input.len() - 2 {
            if letter == &input[index + 2] {
                does_repeat = true;
                break;
            }
        }
    }
    println!("{input:?} doesrepeat {does_repeat}");
    does_repeat
}

fn has_pairof_repeating_letters(input: &str) -> bool {
    // i still don't understand how this works. in future analyze and understand
    input
        .chars()
        .zip(input.chars().skip(1))
        .zip(input.chars().skip(2))
        .any(|((c1, _), c2)| c1 == c2)
}

fn is_pairof_two_letters(input: &Vec<char>) -> bool {
    // this is not working
    let two_letters = &input
        .chunks(2)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>();

    let mut has_pair = false;

    println!("chunk {:?}", two_letters);

    'outer: for (index, pair) in two_letters.into_iter().enumerate() {
        for inner_index in index + 1..two_letters.len() {
            let temp = &two_letters[inner_index..];

            println!(
                "inner loop - {inner_index} {pair} - {:?} - len {} - vec {:?}",
                &temp[0],
                &temp.len(),
                &temp
            );
            if temp.len() > 0 && pair == &temp[0] {
                has_pair = true;
                break 'outer;
            }
        }
    }
    println!("haspair {has_pair}");
    has_pair
}

fn is_restricted_letters(input: &str) -> bool {
    let mut is_nice: bool = true;
    let restricted_letters = vec!["ab", "cd", "pq", "xy"];
    for rl in restricted_letters {
        if input.contains(rl) {
            is_nice = false;
            break;
        }
    }
    is_nice
}

fn is_char_appears_twice(input: &Vec<char>) -> bool {
    let mut is_nice: bool = false;
    for (index, d) in input.iter().enumerate() {
        if index < input.len() - 1 {
            if d == &input[index + 1] {
                is_nice = true;
                break;
            }
        }
    }
    is_nice
}

fn is_contain_three_vowels(input: &Vec<char>) -> bool {
    let mut count = 0;
    let mut is_nice = false;
    input.into_iter().for_each(|d| match d {
        'a' => count += 1,
        'e' => count += 1,
        'i' => count += 1,
        'o' => count += 1,
        'u' => count += 1,
        _ => (),
    });
    if count >= 3 {
        is_nice = true;
    }
    is_nice
}
