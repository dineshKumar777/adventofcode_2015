pub fn solve() {
    // should contain three vowels
    // char appears twice in a row
    // does not contain ab, cd, pq, xy

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
        println!(
            "{line} - rest_ltr {} - char_twice {} - three_vowels {} - nice_count {nice_string}",
            is_restricted_letters(line),
            is_char_appears_twice(&input_chars),
            is_contain_three_vowels(&input_chars)
        );
    }
    println!("part1 {nice_string}");
    assert_eq!(258, nice_string, "part 1 solution should be 258");
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
