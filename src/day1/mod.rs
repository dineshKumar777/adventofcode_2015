use std::fs;

pub fn solve() {
    let path = std::path::Path::new("inputs/day1.txt");
    let input = fs::read_to_string(path).expect("Something went wrong when reading the file");
    let paran_vec: Vec<char> = input.chars().collect();
    let mut floor_num: i32 = 0;

    for p in &paran_vec {
        if *p == '(' {
            floor_num += 1;
        } else {
            floor_num -= 1;
        }
    }

    println!("part1 {}", floor_num);
    assert_eq!(232, floor_num, "part 1 solution should be 232");

    // part 2
    let mut basement_pos = 0;
    floor_num = 0; // reset part1
    for (index, p) in paran_vec.iter().enumerate() {
        if *p == '(' {
            floor_num += 1;
        } else {
            floor_num -= 1;
        }
        // println!("{index} => {p} => {floor_num}");

        if floor_num == -1 {
            basement_pos = index + 1;
            break;
        }
    }
    println!("part2 {}", basement_pos);
    assert_eq!(1783, basement_pos, "part 2 solution should be 1783");
}
